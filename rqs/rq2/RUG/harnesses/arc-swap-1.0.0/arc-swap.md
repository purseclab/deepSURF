# Crate Documentation

**Version:** 1.0.0

**Format Version:** 39

# Module `arc_swap`

Making [`Arc`][Arc] itself atomic

The [`ArcSwap`] type is a container for an `Arc` that can be changed atomically. Semantically,
it is similar to something like `Atomic<Arc<T>>` (if there was such a thing) or
`RwLock<Arc<T>>` (but without the need for the locking). It is optimized for read-mostly
scenarios, with consistent performance characteristics.

# Motivation

There are many situations in which one might want to have some data structure that is often
read and seldom updated. Some examples might be a configuration of a service, routing tables,
snapshot of some data that is renewed every few minutes, etc.

In all these cases one needs:
* Being able to read the current value of the data structure, fast, often and concurrently from
  many threads.
* Using the same version of the data structure over longer period of time ‒ a query should be
  answered by a consistent version of data, a packet should be routed either by an old or by a
  new version of the routing table but not by a combination, etc.
* Perform an update without disrupting the processing.

The first idea would be to use [`RwLock<T>`][RwLock] and keep a read-lock for the whole time of
processing. Update would, however, pause all processing until done.

Better option would be to have [`RwLock<Arc<T>>`][RwLock]. Then one would lock, clone the [Arc]
and unlock. This suffers from CPU-level contention (on the lock and on the reference count of
the [Arc]) which makes it relatively slow. Depending on the implementation, an update may be
blocked for arbitrary long time by a steady inflow of readers.

```rust
# use std::sync::{Arc, RwLock};
# use once_cell::sync::Lazy;
# struct RoutingTable; struct Packet; impl RoutingTable { fn route(&self, _: Packet) {} }
static ROUTING_TABLE: Lazy<RwLock<Arc<RoutingTable>>> = Lazy::new(|| {
    RwLock::new(Arc::new(RoutingTable))
});

fn process_packet(packet: Packet) {
    let table = Arc::clone(&ROUTING_TABLE.read().unwrap());
    table.route(packet);
}
# fn main() { process_packet(Packet); }
```

The [ArcSwap] can be used instead, which solves the above problems and has better performance
characteristics than the [RwLock], both in contended and non-contended scenarios.

```rust
# use arc_swap::ArcSwap;
# use once_cell::sync::Lazy;
# struct RoutingTable; struct Packet; impl RoutingTable { fn route(&self, _: Packet) {} }
static ROUTING_TABLE: Lazy<ArcSwap<RoutingTable>> = Lazy::new(|| {
    ArcSwap::from_pointee(RoutingTable)
});

fn process_packet(packet: Packet) {
    let table = ROUTING_TABLE.load();
    table.route(packet);
}
# fn main() { process_packet(Packet); }
```

# Crate contents

At the heart of the crate there are [`ArcSwap`] and [`ArcSwapOption`] types, containers for an
[`Arc`] and [`Option<Arc>`][Option].

Technically, these are type aliases for partial instantiations of the [`ArcSwapAny`] type. The
[`ArcSwapAny`] is more flexible and allows tweaking of many things (can store other things than
[`Arc`]s, can configure the locking [`Strategy`]). For details about the tweaking, see the
documentation of the [`strategy`] module and the [`RefCnt`] trait.

The [`cache`] module provides means for speeding up read access of the contained data at the
cost of delayed reclamation.

The [`access`] module can be used to do projections into the contained data to separate parts
of application from each other (eg. giving a component access to only its own part of
configuration while still having it reloaded as a whole).

# Before using

The data structure is a bit niche. Before using, please check the
[limitations and common pitfalls][docs::limitations] and the [performance
characteristics][docs::performance], including choosing the right [read
operation][docs::performance#read-operations].

You can also get an inspiration about what's possible in the [common patterns][docs::patterns]
section.

# Examples

```rust
use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_utils::thread;

fn main() {
    let config = ArcSwap::from(Arc::new(String::default()));
    thread::scope(|scope| {
        scope.spawn(|_| {
            let new_conf = Arc::new("New configuration".to_owned());
            config.store(new_conf);
        });
        for _ in 0..10 {
            scope.spawn(|_| {
                loop {
                    let cfg = config.load();
                    if !cfg.is_empty() {
                        assert_eq!(**cfg, "New configuration");
                        return;
                    }
                }
            });
        }
    }).unwrap();
}
```

[RwLock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html

## Modules

## Module `access`

Abstracting over accessing parts of stored value.

Sometimes, there's a big globalish data structure (like a configuration for the whole program).
Then there are parts of the program that need access to up-to-date version of their *part* of
the configuration, but for reasons of code separation and reusability, it is not desirable to
pass the whole configuration to each of the parts.

This module provides means to grant the parts access to the relevant subsets of such global
data structure while masking the fact it is part of the bigger whole from the component.

Note that the [`cache`][crate::cache] module has its own [`Access`][crate::cache::Access] trait
that serves a similar purpose, but with cached access. The signatures are different, therefore
an incompatible trait.

# The general idea

Each part of the code accepts generic [`Access<T>`][Access] for the `T` of its interest. This
provides means to load current version of the structure behind the scenes and get only the
relevant part, without knowing what the big structure is.

For technical reasons, the [`Access`] trait is not object safe. If type erasure is desired, it
is possible use the [`DynAccess`][crate::access::DynAccess] instead, which is object safe, but
slightly slower.

For some cases, it is possible to use [`ArcSwapAny::map`]. If that is not flexible enough, the
[`Map`] type can be created directly.

Note that the [`Access`] trait is also implemented for [`ArcSwapAny`] itself. Additionally,
there's the [`Constant`][crate::access::Constant] helper type, which is useful mostly for
testing (it doesn't allow reloading).

# Performance

In general, these utilities use [`ArcSwapAny::load`] internally and then apply the provided
transformation. This has several consequences:

* Limitations of the [`load`][ArcSwapAny::load] apply ‒ including the recommendation to not
  hold the returned guard object for too long, but long enough to get consistency.
* The transformation should be cheap ‒ optimally just borrowing into the structure.

# Examples

```rust
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use arc_swap::ArcSwap;
use arc_swap::access::{Access, Constant, Map};

fn work_with_usize<A: Access<usize> + Send + 'static>(a: A) {
    thread::spawn(move || {
        loop {
            let value = a.load();
            println!("{}", *value);
            // Not strictly necessary, but dropping the guard can free some resources, like
            // slots for tracking what values are still in use. We do it before the sleeping,
            // not at the end of the scope.
            drop(value);
            thread::sleep(Duration::from_millis(50));
        }
    });
}

// Passing the whole thing directly
// (If we kept another Arc to it, we could change the value behind the scenes)
work_with_usize(Arc::new(ArcSwap::from_pointee(42)));

// Passing a subset of a structure
struct Cfg {
    value: usize,
}

let cfg = Arc::new(ArcSwap::from_pointee(Cfg { value: 0 }));
work_with_usize(Map::new(Arc::clone(&cfg), |cfg: &Cfg| &cfg.value));
cfg.store(Arc::new(Cfg { value: 42 }));

// Passing a constant that can't change. Useful mostly for testing purposes.
work_with_usize(Constant(42));
```

```rust
pub mod access { /* ... */ }
```

### Types

#### Struct `Map`

An adaptor to provide access to a part of larger structure.

This is the *active* part of this module. Use the [module documentation](index.html) for the
details.

```rust
pub struct Map<A, T, F> {
    pub(in ::access) access: A,
    pub(in ::access) projection: F,
    pub(in ::access) _t: std::marker::PhantomData<fn() -> T>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `access` | `A` |  |
| `projection` | `F` |  |
| `_t` | `std::marker::PhantomData<fn() -> T>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<R>(access: A, projection: F) -> Self
where
    F: Fn(&T) -> &R { /* ... */ }
  ```
  Creates a new instance.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **DynAccess**
  - ```rust
    fn load(self: &Self) -> DynGuard<T> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Map<A, T, F> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Access**
  - ```rust
    fn load(self: &Self) -> <Self as >::Guard { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
#### Struct `Constant`

Access to an constant.

This wraps a constant value to provide [`Access`] to it. It is constant in the sense that,
unlike [`ArcSwapAny`] and [`Map`], the loaded value will always stay the same (there's no
remote `store`).

The purpose is mostly testing and plugging a parameter that works generically from code that
doesn't need the updating functionality.

```rust
pub struct Constant<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Constant<T>) -> bool { /* ... */ }
    ```

- **Send**
- **DynAccess**
  - ```rust
    fn load(self: &Self) -> DynGuard<T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Constant<T> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Constant<T>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Copy**
- **Unpin**
- **StructuralPartialEq**
- **Access**
  - ```rust
    fn load(self: &Self) -> <Self as >::Guard { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Constant<T>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

### Traits

#### Trait `Access`

Abstracts over ways code can get access to a value of type `T`.

This is the trait that parts of code will use when accessing a subpart of the big data
structure. See the [module documentation](index.html) for details.

```rust
pub trait Access<T> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Guard`: A guard object containing the value and keeping it alive.

###### Required Methods

- `load`: The loading method.

##### Implementations

This trait is implemented for the following types:

- `P` with <T, A: Access<T>, P: Deref<Target = A>>
- `super::ArcSwapAny<T, S>` with <T: RefCnt, S: Strategy<T>>
- `super::ArcSwapAny<std::sync::Arc<T>, S>` with <T, S: Strategy<std::sync::Arc<T>>>
- `super::ArcSwapAny<std::rc::Rc<T>, S>` with <T, S: Strategy<std::rc::Rc<T>>>
- `Map<A, T, F>` with <A, T, F, R>
- `Constant<T>` with <T: Clone>

#### Trait `DynAccess`

An object-safe version of the [`Access`] trait.

This can be used instead of the [`Access`] trait in case a type erasure is desired. This has
the effect of performance hit (due to boxing of the result and due to dynamic dispatch), but
makes certain code simpler and possibly makes the executable smaller.

This is automatically implemented for everything that implements [`Access`].

# Examples

```rust
use std::thread;

use arc_swap::access::{Constant, DynAccess};

fn do_something(value: Box<dyn DynAccess<usize> + Send>) {
    thread::spawn(move || {
        let v = value.load();
        println!("{}", *v);
    });
}

do_something(Box::new(Constant(42)));
```

```rust
pub trait DynAccess<T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `load`: The equivalent of [`Access::load`].

##### Implementations

This trait is implemented for the following types:

- `A` with <T, A>

## Module `as_raw`

```rust
pub(crate) mod as_raw { /* ... */ }
```

### Modules

## Module `sealed`

```rust
pub(in ::as_raw) mod sealed { /* ... */ }
```

### Traits

#### Trait `Sealed`

```rust
pub trait Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `&''a T` with <''a, T: RefCnt>
- `&''a super::Guard<T>` with <''a, T: RefCnt>
- `super::Guard<T>` with <''a, T: RefCnt>
- `*mut T` with <T>
- `*const T` with <T>

### Traits

#### Trait `AsRaw`

A trait describing things that can be turned into a raw pointer.

This is just an abstraction of things that can be passed to the
[`compare_and_swap`](struct.ArcSwapAny.html#method.compare_and_swap).

# Examples

```
use std::ptr;
use std::sync::Arc;

use arc_swap::ArcSwapOption;

let a = Arc::new(42);
let shared = ArcSwapOption::from(Some(Arc::clone(&a)));

shared.compare_and_swap(&a, Some(Arc::clone(&a)));
shared.compare_and_swap(&None::<Arc<_>>, Some(Arc::clone(&a)));
shared.compare_and_swap(shared.load(), Some(Arc::clone(&a)));
shared.compare_and_swap(&shared.load(), Some(Arc::clone(&a)));
shared.compare_and_swap(ptr::null(), Some(Arc::clone(&a)));
```

```rust
pub trait AsRaw<T>: Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_raw`: Converts the value into a raw pointer.

##### Implementations

This trait is implemented for the following types:

- `&''a T` with <''a, T: RefCnt>
- `&''a super::Guard<T>` with <''a, T: RefCnt>
- `super::Guard<T>` with <''a, T: RefCnt>
- `*mut T` with <T>
- `*const T` with <T>

## Module `cache`

**Attributes:**

- `#![deny(unsafe_code)]`

Caching handle into the [ArcSwapAny].

The [Cache] keeps a copy of the internal [Arc] for faster access.

[Arc]: std::sync::Arc

```rust
pub mod cache { /* ... */ }
```

### Types

#### Struct `Cache`

Caching handle for [`ArcSwapAny`][ArcSwapAny].

Instead of loading the [`Arc`][Arc] on every request from the shared storage, this keeps
another copy inside itself. Upon request it only cheaply revalidates it is up to
date. If it is, access is significantly faster. If it is stale, the [load_full] is done and the
cache value is replaced. Under a read-heavy loads, the measured speedup are 10-25 times,
depending on the architecture.

There are, however, downsides:

* The handle needs to be kept around by the caller (usually, one per thread). This is fine if
  there's one global `ArcSwapAny`, but starts being tricky with eg. data structures build from
  them.
* As it keeps a copy of the [Arc] inside the cache, the old value may be kept alive for longer
  period of time ‒ it is replaced by the new value on [load][Cache::load]. You may not want to
  use this if dropping the old value in timely manner is important (possibly because of
  releasing large amount of RAM or because of closing file handles).

# Examples

```rust
# fn do_something<V>(_v: V) { }
use std::sync::Arc;

use arc_swap::{ArcSwap, Cache};

let shared = Arc::new(ArcSwap::from_pointee(42));
// Start 10 worker threads...
for _ in 0..10 {
    let mut cache = Cache::new(Arc::clone(&shared));
    std::thread::spawn(move || {
        // Keep loading it like mad..
        loop {
            let value = cache.load();
            do_something(value);
        }
    });
}
shared.store(Arc::new(12));
```

[Arc]: std::sync::Arc
[load_full]: ArcSwapAny::load_full

```rust
pub struct Cache<A, T> {
    pub(in ::cache) arc_swap: A,
    pub(in ::cache) cached: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `arc_swap` | `A` |  |
| `cached` | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(arc_swap: A) -> Self { /* ... */ }
  ```
  Creates a new caching handle.

- ```rust
  pub fn arc_swap(self: &Self) -> &<A as >::Target { /* ... */ }
  ```
  Gives access to the (possibly shared) cached [`ArcSwapAny`].

- ```rust
  pub fn load(self: &mut Self) -> &T { /* ... */ }
  ```
  Loads the currently held value.

- ```rust
  pub(in ::cache) fn load_no_revalidate(self: &Self) -> &T { /* ... */ }
  ```

- ```rust
  pub(in ::cache) fn revalidate(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn map<F, U>(self: Self, f: F) -> MapCache<A, T, F>
where
    F: FnMut(&T) -> &U { /* ... */ }
  ```
  Turns this cache into a cache with a projection inside the cached value.

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Access**
  - ```rust
    fn load(self: &mut Self) -> &<T as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(arc_swap: A) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache<A, T> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `MapCache`

An implementation of a cache with a projection into the accessed value.

This is the implementation structure for [`Cache::map`]. It can't be created directly and it
should be used through the [`Access`] trait.

```rust
pub struct MapCache<A, T, F> {
    pub(in ::cache) inner: Cache<A, T>,
    pub(in ::cache) projection: F,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `Cache<A, T>` |  |
| `projection` | `F` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MapCache<A, T, F> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Access**
  - ```rust
    fn load(self: &mut Self) -> &U { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `Access`

Generalization of caches providing access to `T`.

This abstracts over all kinds of caches that can provide a cheap access to values of type `T`.
This is useful in cases where some code doesn't care if the `T` is the whole structure or just
a part of it.

See the example at [`Cache::map`].

```rust
pub trait Access<T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `load`: Loads the value from cache.

##### Implementations

This trait is implemented for the following types:

- `Cache<A, T>` with <A, T, S>
- `MapCache<A, T, F>` with <A, T, S, F, U>

## Module `compile_fail_tests`

```rust,compile_fail
let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
std::thread::spawn(|| {
    drop(shared);
});
```

```rust
let shared = arc_swap::ArcSwap::from_pointee(42);
std::thread::spawn(|| {
    drop(shared);
});
```

```rust,compile_fail
let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
let guard = shared.load();
std::thread::spawn(|| {
    drop(guard);
});
```

```rust
let shared = arc_swap::ArcSwap::from_pointee(42);
let guard = shared.load();
std::thread::spawn(|| {
    drop(guard);
});
```

```rust,compile_fail
let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
crossbeam_utils::thread::scope(|scope| {
    scope.spawn(|_| {
        let _ = &shared;
    });
}).unwrap();
```

```rust
let shared = arc_swap::ArcSwap::from_pointee(42);
crossbeam_utils::thread::scope(|scope| {
    scope.spawn(|_| {
        let _ = &shared;
    });
}).unwrap();
```

```rust,compile_fail
let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
let guard = shared.load();
crossbeam_utils::thread::scope(|scope| {
    scope.spawn(|_| {
        let _ = &guard;
    });
}).unwrap();
```

```rust
let shared = arc_swap::ArcSwap::from_pointee(42);
let guard = shared.load();
crossbeam_utils::thread::scope(|scope| {
    scope.spawn(|_| {
        let _ = &guard;
    });
}).unwrap();
```

See that ArcSwapAny<Rc> really isn't Send.
```rust
use std::sync::Arc;
use arc_swap::ArcSwapAny;

let a: ArcSwapAny<Arc<usize>> = ArcSwapAny::new(Arc::new(42));
std::thread::spawn(move || drop(a));
```

```rust,compile_fail
use std::rc::Rc;
use arc_swap::ArcSwapAny;

let a: ArcSwapAny<Rc<usize>> = ArcSwapAny::new(Rc::new(42));
std::thread::spawn(move || drop(a));
```

```rust
pub(crate) mod compile_fail_tests { /* ... */ }
```

## Module `debt`

```rust
pub(crate) mod debt { /* ... */ }
```

### Types

#### Struct `Debt`

One debt slot.

```rust
pub(crate) struct Debt(pub(in ::debt) std::sync::atomic::AtomicUsize);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::atomic::AtomicUsize` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new(ptr: usize) -> Option<&''static Self> { /* ... */ }
  ```
  Creates a new debt.

- ```rust
  pub(crate) fn pay<T: RefCnt>(self: &Self, ptr: *const <T as >::Base) -> bool { /* ... */ }
  ```
  Tries to pay the given debt.

- ```rust
  pub(crate) fn pay_all<T: RefCnt>(ptr: *const <T as >::Base) { /* ... */ }
  ```
  Pays all the debts on the given pointer.

###### Trait Implementations

- **Unpin**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `Slots`

**Attributes:**

- `#[repr(align(64))]`

```rust
pub(in ::debt) struct Slots(pub(in ::debt) [Debt; 8]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[Debt; 8]` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Slots { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Send**
- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Node`

**Attributes:**

- `#[repr(C)]`

One thread-local node for debts.

```rust
pub(in ::debt) struct Node {
    pub(in ::debt) slots: Slots,
    pub(in ::debt) next: Option<&''static Node>,
    pub(in ::debt) in_use: std::sync::atomic::AtomicBool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `slots` | `Slots` |  |
| `next` | `Option<&''static Node>` |  |
| `in_use` | `std::sync::atomic::AtomicBool` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::debt) fn get() -> &''static Self { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `DebtHead`

A wrapper around a node pointer, to un-claim the node on thread shutdown.

```rust
pub(in ::debt) struct DebtHead {
    pub(in ::debt) node: std::cell::Cell<Option<&''static Node>>,
    pub(in ::debt) offset: std::cell::Cell<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `node` | `std::cell::Cell<Option<&''static Node>>` |  |
| `offset` | `std::cell::Cell<usize>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `traverse`

Goes through the debt linked list.

This traverses the linked list, calling the closure on each node. If the closure returns
`Some`, it terminates with that value early, otherwise it runs to the end.

```rust
pub(in ::debt) fn traverse<R, F: FnMut(&''static Node) -> Option<R>>(f: F) -> Option<R> { /* ... */ }
```

### Constants and Statics

#### Constant `DEBT_SLOT_CNT`

```rust
pub(in ::debt) const DEBT_SLOT_CNT: usize = 8;
```

#### Constant `NO_DEBT`

The value of pointer `1` should be pretty safe, for two reasons:

* It's an odd number, but the pointers we have are likely aligned at least to the word size,
  because the data at the end of the `Arc` has the counters.
* It's in the very first page where NULL lives, so it's not mapped.

```rust
pub(crate) const NO_DEBT: usize = 1;
```

#### Static `DEBT_HEAD`

The head of the debt chain.

```rust
pub(in ::debt) static DEBT_HEAD: std::sync::atomic::AtomicPtr<Node> = _;
```

#### Constant `THREAD_HEAD`

A debt node assigned to this thread.

```rust
pub(in ::debt) const THREAD_HEAD: $crate::thread::LocalKey<DebtHead> = _;
```

## Module `docs`

Additional documentation.

Here we have some more general topics that might be good to know that just don't fit to the
crate level intro.

Also, there were some previous blog posts about the crate which you might find interesting.

# Atomic orderings

Each operation on the [`ArcSwapAny`] with [`DefaultStrategy`] type callable concurrently (eg.
[`load`], but not [`into_inner`]) contains at least one [`SeqCst`] atomic read-write operation,
therefore even operations on different instances have a defined global order of operations.

# Features

The `weak` feature adds the ability to use arc-swap with the [`Weak`] pointer too,
through the [`ArcSwapWeak`] type. The needed std support is stabilized in rust version 1.45 (as
of now in beta).

The `experimental-strategies` enables few more strategies that can be used. Note that these
**are not** part of the API stability guarantees and they may be changed, renamed or removed at
any time.

# Minimal compiler version

The `1` versions will compile on all compilers supporting the 2018 edition. Note that this
applies only if no additional feature flags are enabled and does not apply to compiling or
running tests.

[`ArcSwapAny`]: crate::ArcSwapAny
[`ArcSwapWeak`]: crate::ArcSwapWeak
[`load`]: crate::ArcSwapAny::load
[`into_inner`]: crate::ArcSwapAny::into_inner
[`DefaultStrategy`]: crate::DefaultStrategy
[`SeqCst`]: std::sync::atomic::Ordering::SeqCst
[`Weak`]: std::sync::Weak

```rust
pub mod docs { /* ... */ }
```

### Modules

## Module `internal`

Internal details.

While the other parts of documentation are useful to users of the crate, this part is probably
helpful only if you want to look into the code or are curious about how it works internally.

Also note that any of these details may change in future versions and are not part of the
stability guarantees. Don't rely on anything here.

# Storing the [`Arc`].

The [`Arc`] can be turned into a raw pointer and back. This is abstracted by the [`RefCnt`]
trait and it is technically possible to implement it for custom types (this crate also
implements it for [`Rc`] and [`Weak`], though the actual usefulness of these is a bit
questionable).

The raw pointer is stored inside an [`AtomicPtr`].

# Protection of reference counts

The first idea would be to just use [`AtomicPtr`] with whatever the [`Arc::into_raw`] returns.
Then replacing it would be fine (there's no need to update ref counts). The load needs to
increment the reference count ‒ one still stays inside and another is returned to the caller.
This is done by re-creating the Arc from the raw pointer and then cloning it, throwing one
instance away (without destroying it).

This approach has a problem. There's a short time between we read the raw pointer and increment
the count. If some other thread replaces the stored Arc and throws it away, the ref count could
drop to 0, get destroyed and we would be trying to bump ref counts in a ghost, which would be
totally broken.

To prevent this, we actually use two approaches in a hybrid manner.

The first one is based on hazard pointers idea, but slightly modified. There's a global
repository of pointers that owe a reference. When someone swaps a pointer, it walks this list
and pays all the debts (and takes them out of the repository).

For simplicity and performance, storing into the repository is fallible. If storing into the
repository fails (because the thread used up all its own slots, or because the pointer got
replaced in just the wrong moment and it can't confirm the reservation), unlike the full
hazard-pointers approach, we don't retry, but fall back onto secondary strategy.

Each reader registers itself so it can be tracked, but only as a number. Each writer first
switches the pointer. Then it takes a snapshot of all the current readers and waits until all of
them confirm bumping their reference count. Only then the writer returns to the caller, handing
it the ownership of the Arc and allowing possible bad things (like being destroyed) to happen to
it. This has its own disadvantages, so it is only the second approach.

# Unsafety

All the uses of the unsafe keyword is just to turn the raw pointer back to Arc. It originated
from an Arc in the first place, so the only thing to ensure is it is still valid. That means its
ref count never dropped to 0.

At the beginning, there's ref count of 1 stored in the raw pointer (and maybe some others
elsewhere, but we can't rely on these). This 1 stays there for the whole time the pointer is
stored there. When the arc is replaced, this 1 is returned to the caller, so we just have to
make sure no more readers access it by that time.

# Tracking of readers

The simple way would be to have a count of all readers that could be in the dangerous area
between reading the pointer and bumping the reference count. We could „lock“ the ref count by
incrementing this atomic counter and „unlock“ it when done. The writer would just have to
busy-wait for this number to drop to 0 ‒ then there are no readers at all. This is safe, but a
steady inflow of readers could make a writer wait forever.

Therefore, we separate readers into two groups, odd and even ones (see below how). When we see
both groups to drop to 0 (not necessarily at the same time, though), we are sure all the
previous readers were flushed ‒ each of them had to be either odd or even.

To do that, we define a generation. A generation is a number, incremented at certain times and a
reader decides by this number if it is odd or even.

One of the writers may increment the generation when it sees a zero in the next-generation's
group (if the writer sees 0 in the odd group and the current generation is even, all the current
writers are even ‒ so it remembers it saw odd-zero and increments the generation, so new readers
start to appear in the odd group and the even has a chance to drop to zero later on). Only one
writer does this switch, but all that witness the zero can remember it.

We also split the reader threads into shards ‒ we have multiple copies of the counters, which
prevents some contention and sharing of the cache lines. The writer reads them all and sums them
up.

# Leases and debts

Instead of incrementing the reference count, the pointer reference can be owed. In such case, it
is recorded into a global storage. As each thread has its own storage (the global storage is
composed of multiple thread storages), the readers don't contend. When the pointer is no longer
in use, the debt is erased.

The writer pays all the existing debts, therefore the reader have the full Arc with ref count at
that time. The reader is made aware the debt was paid and decrements the reference count.

# Memory orders

## Synchronizing the data pointed to by the pointer.

We have AcqRel (well, SeqCst, but that's included) on the swap and Acquire on the loads. In case
of the double read around the debt allocation, we do that on the *second*, because of ABA.
That's also why that SeqCst on the allocation of debt itself is not enough.

## The generation lock

Second, the dangerous area when we borrowed the pointer but haven't yet incremented its ref
count needs to stay between incrementing and decrementing the reader count (in either group). To
accomplish that, using Acquire on the increment and Release on the decrement would be enough.
The loads in the writer use Acquire to complete the edge and make sure no part of the dangerous
area leaks outside of it in the writers view. This Acquire, however, forms the edge only with
the *latest* decrement. By making both the increment and decrement AcqRel, we effectively chain
the edges together.

Now the hard part :-). We need to ensure that whatever zero a writer sees is not stale in the
sense that it happened before the switch of the pointer. In other words, we need to make sure
that at the time we start to look for the zeroes, we already see all the current readers. To do
that, we need to synchronize the time lines of the pointer itself and the corresponding group
counters. As these are separate, unrelated, atomics, it calls for SeqCst ‒ on the swap and on
the increment. This'll guarantee that they'll know which happened first (either increment or the
swap), making a base line for the following operations (load of the pointer or looking for
zeroes).

# Memory orders around debts

The linked list of debt nodes only grows. The shape of the list (existence of nodes) is
synchronized through Release on creation and Acquire on load on the head pointer.

The debts work similar to locks ‒ Acquire and Release make all the pointer manipulation at the
interval where it is written down. However, we use the SeqCst on the allocation of the debt for
the same reason we do so with the generation lock.

In case the writer pays the debt, it sees the new enough data (for the same reasons the stale
zeroes are not seen). The reference count on the Arc is AcqRel and makes sure it is not
destroyed too soon. The writer traverses all the slots, therefore they don't need to synchronize
with each other.

# Orderings on the rest

We don't really care much if we use a stale generation number ‒ it only works to route the
readers into one or another bucket, but even if it was completely wrong, it would only slow the
waiting for 0 down. So, the increments of it are just hints.

All other operations can be Relaxed (they either only claim something, which doesn't need to
synchronize with anything else, or they are failed attempts at something ‒ and another attempt
will be made, the successful one will do the necessary synchronization).

[`RefCnt`]: crate::RefCnt
[`Arc`]: std::sync::Arc
[`Arc::into_raw`]: std::sync::Arc::into_raw
[`Rc`]: std::rc::Rc
[`Weak`]: std::sync::Weak
[`AtomicPtr`]: std::sync::atomic::AtomicPtr

```rust
pub mod internal { /* ... */ }
```

## Module `limitations`

Limitations and common pitfalls.

# Sized types

This currently works only for `Sized` types. Unsized types have „fat pointers“, which are twice
as large as the normal ones. The [`AtomicPtr`] doesn't support them. One could use something
like `AtomicU128` for them. The catch is this doesn't exist and the difference would make it
really hard to implement the debt storage/stripped down hazard pointers.

A workaround is to use double indirection:

```rust
# use arc_swap::ArcSwap;
// This doesn't work:
// let data: ArcSwap<[u8]> = ArcSwap::new(Arc::from([1, 2, 3]));

// But this does:
let data: ArcSwap<Box<[u8]>> = ArcSwap::from_pointee(Box::new([1, 2, 3]));
# drop(data);
```

# Too many [`Guard`]s

There's only limited number of "fast" slots for borrowing from [`ArcSwap`] for each single
thread (currently 8, but this might change in future versions). If these run out, the algorithm
falls back to slower path.

If too many [`Guard`]s are kept around, the performance might be poor. These are not intended
to be stored in data structures or used across async yield points.

[`ArcSwap`]: crate::ArcSwap
[`Guard`]: crate::Guard
[`AtomicPtr`]: std::sync::atomic::AtomicPtr

# No `Clone` implementation

Previous version implemented [`Clone`], but it turned out to be very confusing to people, since
it created fully independent [`ArcSwap`]. Users expected the instances to be tied to each
other, that store in one would change the result of future load of the other.

To emulate the original behaviour, one can do something like this:

```rust
# use arc_swap::ArcSwap;
# let old = ArcSwap::from_pointee(42);
let new = ArcSwap::new(old.load_full());
# let _ = new;
```

```rust
pub mod limitations { /* ... */ }
```

## Module `patterns`

Common use patterns

Here are some common patterns one can use for inspiration. These are mostly covered by examples
at the right type in the crate, but this lists them at a single place.

# Sharing of configuration data

We want to share configuration from some source with rare updates to some high performance
worker threads. It can be configuration in its true sense, or a routing table.

The idea here is, each new version is a newly allocated in its own [`Arc`]. It is then stored
into a *shared* `ArcSwap` instance.

Each worker then loads the current version before each work chunk. In case a new version is
stored, the worker keeps using the loaded one until it ends the work chunk and, if it's the
last one to have the version, deallocates it automatically by dropping the [`Guard`]

Note that the configuration needs to be passed through a *single shared* [`ArcSwap`]. That
means we need to share that instance and we do so through an [`Arc`] (one could use a global
variable instead).

Therefore, what we have is `Arc<ArcSwap<Config>>`.

```rust
# use std::sync::Arc;
# use std::thread;
# use std::time::Duration;
#
# use arc_swap::ArcSwap;
# struct Work;
# impl Work { fn fetch() -> Self { Work } fn perform(&self, _: &Config) {} }
#
#[derive(Debug, Default)]
struct Config {
    // ... Stuff in here ...
}

// We wrap the ArcSwap into an Arc, so we can share it between threads.
let config = Arc::new(ArcSwap::from_pointee(Config::default()));
// The configuration thread
thread::spawn({
    let config = Arc::clone(&config);
    move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            // Actually, load it from somewhere
            let new_config = Arc::new(Config::default());
            config.store(new_config);
        }
    }
});

// The worker thread
for _ in 0..10 {
    thread::spawn({
        let config = Arc::clone(&config);
        move || {
            loop {
                let work = Work::fetch();
                let config = config.load();
                work.perform(&config);
            }
        }
    });
}
```

# Consistent snapshots

While one probably wants to get a fresh instance every time a work chunk is available,
therefore there would be one [`load`] for each work chunk, it is often also important that the
configuration doesn't change in the *middle* of processing of one chunk. Therefore, one
commonly wants *exactly* one [`load`] for the work chunk, not *at least* one. If the processing
had multiple phases, one would use something like this:

```rust
# use std::sync::Arc;
#
# use arc_swap::ArcSwap;
# struct Config;
# struct Work;
# impl Work {
#     fn fetch() -> Self { Work }
#     fn phase_1(&self, _: &Config) {}
#     fn phase_2(&self, _: &Config) {}
# }
# let config = Arc::new(ArcSwap::from_pointee(Config));
let work = Work::fetch();
let config = config.load();
work.phase_1(&config);
// We keep the same config value here
work.phase_2(&config);
```

Over this:

```rust
# use std::sync::Arc;
#
# use arc_swap::ArcSwap;
# struct Config;
# struct Work;
# impl Work {
#     fn fetch() -> Self { Work }
#     fn phase_1(&self, _: &Config) {}
#     fn phase_2(&self, _: &Config) {}
# }
# let config = Arc::new(ArcSwap::from_pointee(Config));
let work = Work::fetch();
work.phase_1(&config.load());
// WARNING!! This is broken, because in between phase_1 and phase_2, the other thread could
// have replaced the config. Then each phase would be performed with a different one and that
// could lead to surprises.
work.phase_2(&config.load());
```

# Caching of the configuration

Let's say that the work chunks are really small, but there's *a lot* of them to work on. Maybe
we are routing packets and the configuration is the routing table that can sometimes change,
but mostly doesn't.

There's an overhead to [`load`]. If the work chunks are small enough, that could be measurable.
We can reach for [`Cache`]. It makes loads much faster (in the order of accessing local
variables) in case nothing has changed. It has two costs, it makes the load slightly slower in
case the thing *did* change (which is rare) and if the worker is inactive, it holds the old
cached value alive.

This is OK for our use case, because the routing table is usually small enough so some stale
instances taking a bit of memory isn't an issue.

The part that takes care of updates stays the same as above.

```rust
# use std::sync::Arc;
# use std::thread;
# use arc_swap::{ArcSwap, Cache};
# struct Packet; impl Packet { fn receive() -> Self { Packet } }

#[derive(Debug, Default)]
struct RoutingTable {
    // ... Stuff in here ...
}

impl RoutingTable {
    fn route(&self, _: Packet) {
        // ... Interesting things are done here ...
    }
}

let routing_table = Arc::new(ArcSwap::from_pointee(RoutingTable::default()));

for _ in 0..10 {
    thread::spawn({
        let routing_table = Arc::clone(&routing_table);
        move || {
            let mut routing_table = Cache::new(routing_table);
            loop {
                let packet = Packet::receive();
                // This load is cheaper, because we cache in the private Cache thing.
                // But if the above receive takes a long time, the Cache will keep the stale
                // value  alive until this time (when it will get replaced by up to date value).
                let current = routing_table.load();
                current.route(packet);
            }
        }
    });
}
```

# Projecting into configuration field

We have a larger application, composed of multiple components. Each component has its own
`ComponentConfig` structure. Then, the whole application has a `Config` structure that contains
a component config for each component:

```rust
# struct ComponentConfig;

struct Config {
    component: ComponentConfig,
    // ... Some other components and things ...
}
# let c = Config { component: ComponentConfig };
# let _ = c.component;
```

We would like to use [`ArcSwap`] to push updates to the components. But for various reasons,
it's not a good idea to put the whole `ArcSwap<Config>` to each component, eg:

* That would make each component depend on the top level config, which feels reversed.
* It doesn't allow reusing the same component in multiple applications, as these would have
  different `Config` structures.
* One needs to build the whole `Config` for tests.
* There's a risk of entanglement, that the component would start looking at configuration of
  different parts of code, which would be hard to debug.

We also could have a separate `ArcSwap<ComponentConfig>` for each component, but that also
doesn't feel right, as we would have to push updates to multiple places and they could be
inconsistent for a while and we would have to decompose the `Config` structure into the parts,
because we need our things in [`Arc`]s to be put into [`ArcSwap`].

This is where the [`Access`] trait comes into play. The trait abstracts over things that can
give access to up to date version of specific T. That can be a [`Constant`] (which is useful
mostly for the tests, where one doesn't care about the updating), it can be an
[`ArcSwap<T>`][`ArcSwap`] itself, but it also can be an [`ArcSwap`] paired with a closure to
project into the specific field. The [`DynAccess`] is similar, but allows type erasure. That's
more convenient, but a little bit slower.

```rust
# use std::sync::Arc;
# use arc_swap::ArcSwap;
# use arc_swap::access::{DynAccess, Map};

#[derive(Debug, Default)]
struct ComponentConfig;

struct Component {
    config: Box<dyn DynAccess<ComponentConfig>>,
}

#[derive(Debug, Default)]
struct Config {
    component: ComponentConfig,
}

let config = Arc::new(ArcSwap::from_pointee(Config::default()));

let component = Component {
    config: Box::new(Map::new(Arc::clone(&config), |config: &Config| &config.component)),
};
# let _ = component.config;
```

One would use `Box::new(Constant(ComponentConfig))` in unittests instead as the `config` field.

The [`Cache`] has its own [`Access`][crate::cache::Access] trait for similar purposes.

[`Arc`]: std::sync::Arc
[`Guard`]: crate::Guard
[`load`]: crate::ArcSwapAny::load
[`ArcSwap`]: crate::ArcSwap
[`Cache`]: crate::cache::Cache
[`Access`]: crate::access::Access
[`DynAccess`]: crate::access::DynAccess
[`Constant`]: crate::access::Constant

```rust
pub mod patterns { /* ... */ }
```

## Module `performance`

Performance characteristics.

There are several performance advantages of [`ArcSwap`] over [`RwLock`].

## Lock-free readers

All the read operations are always [lock-free]. Most of the time, they are actually
[wait-free], the notable exception is the first [`load`] access in each thread (across all the
instances of [`ArcSwap`]), as it sets up some thread-local data structures.

Whenever the documentation talks about *contention* in the context of [`ArcSwap`], it talks
about contention on the CPU level ‒ multiple cores having to deal with accessing the same cache
line. This slows things down (compared to each one accessing its own cache line), but an
eventual progress is still guaranteed and the cost is significantly lower than parking threads
as with mutex-style contention.

Unfortunately writers are *not* [lock-free]. A reader stuck (suspended/killed) in a critical
section (few instructions long in case of [`load`]) may block a writer from completion.
Nevertheless, a steady inflow of new readers nor other writers will not block the writer.

## Speeds

The base line speed of read operations is similar to using an *uncontended* [`Mutex`].
However, [`load`] suffers no contention from any other read operations and only slight
ones during updates. The [`load_full`] operation is additionally contended only on
the reference count of the [`Arc`] inside ‒ so, in general, while [`Mutex`] rapidly
loses its performance when being in active use by multiple threads at once and
[`RwLock`] is slow to start with, [`ArcSwap`] mostly keeps its performance even when read by
many threads in parallel.

Write operations are considered expensive. A write operation is more expensive than access to
an *uncontended* [`Mutex`] and on some architectures even slower than uncontended
[`RwLock`]. However, it is faster than either under contention.

There are some (very unscientific) [benchmarks] within the source code of the library.

The exact numbers are highly dependant on the machine used (both absolute numbers and relative
between different data structures). Not only architectures have a huge impact (eg. x86 vs ARM),
but even AMD vs. Intel or two different Intel processors. Therefore, if what matters is more
the speed than the wait-free guarantees, you're advised to do your own measurements.

Further speed improvements may be gained by the use of the [`Cache`].

## Consistency

The combination of [wait-free] guarantees of readers and no contention between concurrent
[`load`]s provides *consistent* performance characteristics of the synchronization mechanism.
This might be important for soft-realtime applications (the CPU-level contention caused by a
recent update/write operation might be problematic for some hard-realtime cases, though).

## Choosing the right reading operation

There are several load operations available. While the general go-to one should be
[`load`], there may be situations in which the others are a better match.

The [`load`] usually only borrows the instance from the shared [`ArcSwap`]. This makes
it faster, because different threads don't contend on the reference count. There are two
situations when this borrow isn't possible. If the content gets changed, all existing
[`Guard`]s are promoted to contain an owned instance. The promotion is done by the
writer, but the readers still need to decrement the reference counts of the old instance when
they no longer use it, contending on the count.

The other situation derives from internal implementation. The number of borrows each thread can
have at each time (across all [`Guard`]s) is limited. If this limit is exceeded, an onwed
instance is created instead.

Therefore, if you intend to hold onto the loaded value for extended time span, you may prefer
[`load_full`]. It loads the pointer instance ([`Arc`]) without borrowing, which is
slower (because of the possible contention on the reference count), but doesn't consume one of
the borrow slots, which will make it more likely for following [`load`]s to have a slot
available. Similarly, if some API needs an owned `Arc`, [`load_full`] is more convenient and
potentially faster then first [`load`]ing and then cloning that [`Arc`].

Additionally, it is possible to use a [`Cache`] to get further speed improvement at the
cost of less comfortable API and possibly keeping the older values alive for longer than
necessary.

[`ArcSwap`]: crate::ArcSwap
[`Cache`]: crate::cache::Cache
[`Guard`]: crate::Guard
[`load`]: crate::ArcSwapAny::load
[`load_full`]: crate::ArcSwapAny::load_full
[`Arc`]: std::sync::Arc
[`Mutex`]: std::sync::Mutex
[`RwLock`]: std::sync::RwLock
[benchmarks]: https://github.com/vorner/arc-swap/tree/master/benchmarks
[lock-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Lock-freedom
[wait-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Wait-freedom

```rust
pub mod performance { /* ... */ }
```

## Module `gen_lock`

Customization of where and how the generation lock works.

By default, all the [`ArcSwapAny`](../struct.ArcSwapAny.html) instances share the same
generation lock. This is to save space in them (they have the same size as a single pointer),
because the default lock is quite a large data structure (it's sharded, to prevent too much
contention between different threads). This has the disadvantage that a lock on one instance
influences another instance.

The things in this module allow customizing how the lock behaves. The default one is
[`Global`](struct.Global.html). If you want to use independent but unsharded lock, use the
[`PrivateUnsharded`](struct.PrivateUnsharded.html) (or the
[`IndependentArcSwap`](../type.IndependentArcSwap.html) type alias).

Or you can implement your own lock, but you probably should study the internals of the library
first.

# Not Implemented Yet

These variants would probably make sense, but haven't been written yet:

* A lock storage that is shared, but only between a certain group of pointers. It could be
  either as a reference (but then each `ArcSwap` would get a bit bigger), or a macro that could
  generate an independent but global storage.

```rust
pub(crate) mod gen_lock { /* ... */ }
```

### Types

#### Struct `Shard`

**Attributes:**

- `#[repr(align(64))]`

A single shard.

This is one copy of place where the library keeps tracks of generation locks. It consists of a
pair of counters and allows double-buffering readers (therefore, even if there's a never-ending
stream of readers coming in, writer will get through eventually).

To avoid contention and sharing of the counters between readers, we don't have one pair of
generation counters, but several. The reader picks one shard and uses that, while the writer
looks through all of them. This is still not perfect (two threads may choose the same ID), but
it helps.

Each [`LockStorage`](trait.LockStorage.html) must provide a (non-empty) array of these.

```rust
pub struct Shard(pub(crate) [std::sync::atomic::AtomicUsize; 2]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[std::sync::atomic::AtomicUsize; 2]` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[AtomicUsize; 2] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Shard { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
#### Type Alias `Shards`

```rust
pub(in ::gen_lock) type Shards = [Shard; 9];
```

#### Struct `Global`

The default, global lock.

The lock is stored out-of-band, globally. This means that one `ArcSwap` with this lock storage
is only one machine word large, but a lock on one instance blocks the other, independent ones.

It has several shards so threads are less likely to collide (HW-contend) on them.

```rust
pub struct Global;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Global { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Global { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **LockStorage**
  - ```rust
    fn gen_idx(self: &Self) -> &AtomicUsize { /* ... */ }
    ```

  - ```rust
    fn shards(self: &Self) -> &[Shard; 9] { /* ... */ }
    ```

  - ```rust
    fn choose_shard(self: &Self) -> usize { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Send**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `PrivateUnsharded`

A single „shard“ that is stored inline, inside the corresponding `ArcSwap`. Therefore, locks on
each instance won't influence any other instances. On the other hand, the `ArcSwap` itself gets
bigger and doesn't have multiple shards, so concurrent uses might contend each other a bit.

Note that there`s a type alias [`IndependentArcSwap`](../type.IndependentArcSwap.html) that can
be used instead.

```rust
pub struct PrivateUnsharded {
    pub(in ::gen_lock) gen_idx: std::sync::atomic::AtomicUsize,
    pub(in ::gen_lock) shard: [[std::sync::atomic::AtomicUsize; 2]; 1],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `gen_idx` | `std::sync::atomic::AtomicUsize` |  |
| `shard` | `[[std::sync::atomic::AtomicUsize; 2]; 1]` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> PrivateUnsharded { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LockStorage**
  - ```rust
    fn gen_idx(self: &Self) -> &AtomicUsize { /* ... */ }
    ```

  - ```rust
    fn shards(self: &Self) -> &[<Self as >::Shard; 1] { /* ... */ }
    ```

  - ```rust
    fn choose_shard(self: &Self) -> usize { /* ... */ }
    ```

#### Struct `GenLock`

```rust
pub(crate) struct GenLock<''a> {
    pub(in ::gen_lock) slot: &''a std::sync::atomic::AtomicUsize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `slot` | `&''a std::sync::atomic::AtomicUsize` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new<S: LockStorage + ''a>(storage: &''a S) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **UnwindSafe**
- **Send**
### Traits

#### Trait `LockStorage`

Abstraction of the place where generation locks are stored.

The trait is unsafe because if the trait messes up with the values stored in there in any way
(or makes the values available to something else that messes them up), this can cause UB and
daemons and discomfort to users and such. The library expects it is the only one storing values
there. In other words, it is expected the trait is only a dumb storage and doesn't actively do
anything.

```rust
pub unsafe trait LockStorage: Default {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Shard`: Type of one shard.
- `Shards`: The type for keeping several shards.

###### Required Methods

- `gen_idx`: Access to the generation index.
- `shards`: Access to the shards storage.
- `choose_shard`: Pick one shard of the all selected.

##### Implementations

This trait is implemented for the following types:

- `Global`
- `PrivateUnsharded`

### Functions

#### Function `snapshot`

```rust
pub(in ::gen_lock) fn snapshot(shard: &[std::sync::atomic::AtomicUsize; 2]) -> [usize; 2] { /* ... */ }
```

#### Function `wait_for_readers`

```rust
pub(crate) fn wait_for_readers<S: LockStorage>(storage: &S) { /* ... */ }
```

### Constants and Statics

#### Constant `SHARD_CNT`

Number of shards (see [`Shard`]).

```rust
pub(in ::gen_lock) const SHARD_CNT: usize = 9;
```

#### Constant `YIELD_EVERY`

If waiting in a spin loop, do a thread yield to the OS scheduler this many iterations

```rust
pub(in ::gen_lock) const YIELD_EVERY: usize = 16;
```

#### Constant `MAX_GUARDS`

Maximum number of guards in the critical section

```rust
pub(in ::gen_lock) const MAX_GUARDS: usize = _;
```

#### Constant `GEN_CNT`

How many generations we have in the lock.

```rust
pub(crate) const GEN_CNT: usize = 2;
```

#### Static `GEN_IDX`

```rust
pub(in ::gen_lock) static GEN_IDX: std::sync::atomic::AtomicUsize = _;
```

#### Static `SHARDS`

The global shards.

```rust
pub(in ::gen_lock) static SHARDS: [Shard; 9] = _;
```

#### Static `THREAD_ID_GEN`

Global counter of threads.

We specifically don't use ThreadId here, because it is opaque and doesn't give us a number :-(.

```rust
pub(in ::gen_lock) static THREAD_ID_GEN: std::sync::atomic::AtomicUsize = _;
```

#### Constant `THREAD_SHARD`

A shard a thread has chosen.

The default value is just a marker it hasn't been set.

```rust
pub(in ::gen_lock) const THREAD_SHARD: $crate::thread::LocalKey<std::cell::Cell<usize>> = _;
```

### Macros

#### Macro `sh`

```rust
pub(crate) macro_rules! sh {
    /* macro_rules! sh {
    () => { ... };
} */
}
```

## Module `ref_cnt`

```rust
pub(crate) mod ref_cnt { /* ... */ }
```

### Traits

#### Trait `RefCnt`

A trait describing smart reference counted pointers.

Note that in a way [`Option<Arc<T>>`][Option] is also a smart reference counted pointer, just
one that can hold NULL.

The trait is unsafe, because a wrong implementation will break the [ArcSwapAny]
implementation and lead to UB.

This is not actually expected for downstream crate to implement, this is just means to reuse
code for [Arc] and [`Option<Arc>`][Option] variants. However, it is theoretically possible (if
you have your own [Arc] implementation).

It is also implemented for [Rc], but that is not considered very useful (because the
[ArcSwapAny] is not `Send` or `Sync`, therefore there's very little advantage for it to be
atomic).

# Safety

Aside from the obvious properties (like that incrementing and decrementing a reference count
cancel each out and that having less references tracked than how many things actually point to
the value is fine as long as the count doesn't drop to 0), it also must satisfy that if two
pointers have the same value, they point to the same object. This is specifically not true for
ZSTs, but it is true for `Arc`s of ZSTs, because they have the reference counts just after the
value. It would be fine to point to a type-erased version of the same object, though (if one
could use this trait with unsized types in the first place).

Furthermore, the type should be Pin (eg. if the type is cloned or moved, it should still
point/deref to the same place in memory).

[Arc]: std::sync::Arc
[Rc]: std::rc::Rc
[ArcSwapAny]: crate::ArcSwapAny

```rust
pub unsafe trait RefCnt: Clone {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Base`: The base type the pointer points to.

###### Required Methods

- `into_ptr`: Converts the smart pointer into a raw pointer, without affecting the reference count.
- `as_ptr`: Provides a view into the smart pointer as a raw pointer.
- `from_ptr`: Converts a raw pointer back into the smart pointer, without affecting the reference count.

##### Provided Methods

- ```rust
  fn inc(me: &Self) -> *mut <Self as >::Base { /* ... */ }
  ```
  Increments the reference count by one.

- ```rust
  unsafe fn dec(ptr: *const <Self as >::Base) { /* ... */ }
  ```
  Decrements the reference count by one.

##### Implementations

This trait is implemented for the following types:

- `std::sync::Arc<T>` with <T>
- `std::rc::Rc<T>` with <T>
- `Option<T>` with <T: RefCnt>

## Module `strategy`

Strategies for protecting the reference counts.

There are multiple algorithms how to protect the reference counts while they're being updated
by multiple threads, each with its own set of pros and cons. The [`DefaultStrategy`] is used by
default and should generally be the least surprising option. It is possible to pick a different
strategy.

For now, the traits in here are sealed and don't expose any methods to the users of the crate.
This is because we are not confident about the details just yet. In the future it may be
possible for downstream users to implement their own, but for now it is only so users can
choose one of the provided.

It is expected that future strategies would come with different capabilities and limitations.
In particular, some that are not "tight" in the cleanup (delay the cleanup) or not support the
compare and swap operations.

Currently, we have these strategies:

* [`DefaultStrategy`] (this one is used implicitly)
* [`IndependentStrategy`]
* [`RwLock<()>`][std::sync::RwLock]

# Testing

Formally, the [`RwLock<()>`][std::sync::RwLock] may be used as a strategy too. It doesn't have
the performance characteristics or lock-free guarantees of the others, but it is much simpler
and contains less `unsafe` code (actually, less code altogether). Therefore, it can be used for
testing purposes and cross-checking.

Note that generally, using [`RwLock<Arc<T>>`][std::sync::RwLock] is likely to be better
performance wise. So if the goal is to not use third-party unsafe code, only the one in
[`std`], that is the better option. This is provided mostly for investigation and testing of
[`ArcSwap`] itself or algorithms written to use [`ArcSwap`].

*This is not meant to be used in production code*.

# Experimental strategies

There are some more strategies inside the [`experimental`] module. Note that these **are not**
subject to the API stability guarantees and can be changed, renamed or removed at any time.

They are available only with the `experimental-strategies` feature.

[`ArcSwap`]: crate::ArcSwap
[`load`]: crate::ArcSwapAny::load

```rust
pub mod strategy { /* ... */ }
```

### Modules

## Module `gen_lock`

```rust
pub(in ::strategy) mod gen_lock { /* ... */ }
```

### Types

#### Struct `GenLockStrategy`

```rust
pub struct GenLockStrategy<L>(pub(crate) L);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `L` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Strategy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenLockStrategy<L> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> GenLockStrategy<L> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CaS**
  - ```rust
    unsafe fn compare_and_swap<C: crate::as_raw::AsRaw<<T as >::Base>>(self: &Self, storage: &AtomicPtr<<T as >::Base>, current: C, new: T) -> <Self as >::Protected { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **InnerStrategy**
  - ```rust
    unsafe fn load(self: &Self, storage: &AtomicPtr<<T as >::Base>) -> <Self as >::Protected { /* ... */ }
    ```

  - ```rust
    unsafe fn wait_for_readers(self: &Self, _: *const <T as >::Base) { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `hybrid`

```rust
pub(in ::strategy) mod hybrid { /* ... */ }
```

### Types

#### Struct `HybridProtection`

```rust
pub struct HybridProtection<T: RefCnt> {
    pub(in ::strategy::hybrid) debt: Option<&''static crate::debt::Debt>,
    pub(in ::strategy::hybrid) ptr: std::mem::ManuallyDrop<T>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `debt` | `Option<&''static crate::debt::Debt>` |  |
| `ptr` | `std::mem::ManuallyDrop<T>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::strategy::hybrid) unsafe fn new(ptr: *const <T as >::Base, debt: Option<&''static Debt>) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::strategy::hybrid) fn attempt(storage: &AtomicPtr<<T as >::Base>) -> Option<Self> { /* ... */ }
  ```

###### Trait Implementations

- **Protected**
  - ```rust
    fn from_inner(ptr: T) -> Self { /* ... */ }
    ```

  - ```rust
    fn into_inner(self: Self) -> T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `HybridStrategy`

```rust
pub struct HybridStrategy<F> {
    pub(in ::strategy::hybrid) fallback: F,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fallback` | `F` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Send**
- **Default**
  - ```rust
    fn default() -> HybridStrategy<F> { /* ... */ }
    ```

- **InnerStrategy**
  - ```rust
    unsafe fn load(self: &Self, storage: &AtomicPtr<<T as >::Base>) -> <Self as >::Protected { /* ... */ }
    ```

  - ```rust
    unsafe fn wait_for_readers(self: &Self, old: *const <T as >::Base) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Strategy**
- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CaS**
  - ```rust
    unsafe fn compare_and_swap<C: crate::as_raw::AsRaw<<T as >::Base>>(self: &Self, storage: &AtomicPtr<<T as >::Base>, current: C, new: T) -> <Self as >::Protected { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HybridStrategy<F> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

## Module `rw_lock`

```rust
pub(in ::strategy) mod rw_lock { /* ... */ }
```

## Module `sealed`

```rust
pub(crate) mod sealed { /* ... */ }
```

### Traits

#### Trait `Protected`

```rust
pub trait Protected<T>: Borrow<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `into_inner`
- `from_inner`

##### Implementations

This trait is implemented for the following types:

- `HybridProtection<T>` with <T: RefCnt>
- `T` with <T: RefCnt>

#### Trait `InnerStrategy`

```rust
pub trait InnerStrategy<T: RefCnt> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Protected`

###### Required Methods

- `load`
- `wait_for_readers`

##### Implementations

This trait is implemented for the following types:

- `GenLockStrategy<L>` with <T: RefCnt, L: LockStorage>
- `HybridStrategy<F>` with <T, F>
- `std::sync::RwLock<()>` with <T: RefCnt>

#### Trait `CaS`

```rust
pub trait CaS<T: RefCnt>: InnerStrategy<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `compare_and_swap`

##### Implementations

This trait is implemented for the following types:

- `GenLockStrategy<L>` with <T: RefCnt, L: LockStorage>
- `HybridStrategy<super::gen_lock::GenLockStrategy<L>>` with <T: RefCnt, L: LockStorage>
- `std::sync::RwLock<()>` with <T: RefCnt>

### Types

#### Type Alias `DefaultStrategy`

The default strategy.

It is used by the type aliases [`ArcSwap`][crate::ArcSwap] and
[`ArcSwapOption`][crate::ArcSwapOption]. Only the other strategies need to be used explicitly.

It is optimized for read heavy situations. The readers are wait-free (with the exception of the
first [`load`] in each thread, which is merely lock-free), writers are not lock-free. The
reclamation is tight ‒ the resource is released as soon as possible.

Each thread has a limited number of "fast" slots. If a thread holds less than this number,
loading is fast and does not suffer from contention (unlike using [`RwLock`][std::sync::RwLock]
or even updating reference counts on the [`Arc`][std::sync::Arc]). In other words, no matter
how many threads concurrently read, they should not be affecting performance of each other in
any way.

If the slots run out (the thread holds too many [`Guard`][crate::Guard], the loading becomes
slower (because the reference counts need to be updated).

Currently, the implementation is a hybrid between stripped-down hazard pointers and one-sided
spin lock. The hazard pointers are the primary fast path. The spin locks are shared between all
instances, therefore the fallbacks may influence other instances.

However, the actual implementation can change in future versions for something else with
similar or better properties.

[`load`]: crate::ArcSwapAny::load

```rust
pub type DefaultStrategy = self::hybrid::HybridStrategy<self::gen_lock::GenLockStrategy<crate::gen_lock::Global>>;
```

#### Type Alias `IndependentStrategy`

Strategy for isolating instances.

It is similar to [`DefaultStrategy`], however the spin lock is not sharded (therefore multiple
concurrent threads might get bigger hit when multiple threads have to fall back). Nevertheless,
each instance has a private spin lock, not influencing the other instances. That also makes
them bigger in memory.

The hazard pointers are still shared between all instances.

The purpose of this strategy is meant for cases where a single instance is going to be
"tortured" a lot, so it should not overflow to other instances.

This too may be changed for something else (but with at least as good guarantees, primarily
that other instances won't get influenced by the "torture").

```rust
pub type IndependentStrategy = self::hybrid::HybridStrategy<self::gen_lock::GenLockStrategy<crate::gen_lock::PrivateUnsharded>>;
```

### Traits

#### Trait `Strategy`

A strategy for protecting the reference counted pointer `T`.

This chooses the algorithm for how the reference counts are protected. Note that the user of
the crate can't implement the trait and can't access any method; this is hopefully temporary
measure to make sure the interface is not part of the stability guarantees of the crate. Once
enough experience is gained with implementing various strategies, it will be un-sealed and
users will be able to provide their own implementation.

For now, the trait works only as a bound to talk about the types that represent strategies.

```rust
pub trait Strategy<T: RefCnt>: sealed::InnerStrategy<T> {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `S` with <T: RefCnt, S: sealed::InnerStrategy<T>>

#### Trait `CaS`

An extension of the [`Strategy`], allowing for compare and swap operation.

The compare and swap operation is "advanced" and not all strategies need to support them.
Therefore, it is a separate trait.

Similarly, it is not yet made publicly usable or implementable and works only as a bound.

```rust
pub trait CaS<T: RefCnt>: sealed::CaS<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `S` with <T: RefCnt, S: sealed::CaS<T>>

## Types

### Struct `Guard`

A temporary storage of the pointer.

This guard object is returned from most loading methods (with the notable exception of
[`load_full`](struct.ArcSwapAny.html#method.load_full)). It dereferences to the smart pointer
loaded, so most operations are to be done using that.

```rust
pub struct Guard<T: RefCnt, S: Strategy<T> = DefaultStrategy> {
    pub(crate) inner: <S as >::Protected,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `<S as >::Protected` |  |

#### Implementations

##### Methods

- ```rust
  pub fn into_inner(lease: Self) -> T { /* ... */ }
  ```
  Converts it into the held value.

- ```rust
  pub fn from_inner(inner: T) -> Self { /* ... */ }
  ```
  Create a guard for a given value `inner`.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(inner: T) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Receiver**
- **Sync**
- **RefUnwindSafe**
- **Access**
  - ```rust
    fn load(self: &Self) -> <P as Access<T>>::Guard { /* ... */ }
    ```

- **Freeze**
- **AsRaw**
  - ```rust
    fn as_raw(self: &Self) -> *mut <T as >::Base { /* ... */ }
    ```

  - ```rust
    fn as_raw(self: &Self) -> *mut <T as >::Base { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sealed**
- **DynAccess**
  - ```rust
    fn load(self: &Self) -> DynGuard<T> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

### Struct `ArcSwapAny`

An atomic storage for a reference counted smart pointer like [`Arc`] or `Option<Arc>`.

This is a storage where a smart pointer may live. It can be read and written atomically from
several threads, but doesn't act like a pointer itself.

One can be created [`from`] an [`Arc`]. To get the pointer back, use the
[`load`](#method.load).

# Note

This is the common generic implementation. This allows sharing the same code for storing
both `Arc` and `Option<Arc>` (and possibly other similar types).

In your code, you most probably want to interact with it through the
[`ArcSwap`](type.ArcSwap.html) and [`ArcSwapOption`](type.ArcSwapOption.html) aliases. However,
the methods they share are described here and are applicable to both of them. That's why the
examples here use `ArcSwap` ‒ but they could as well be written with `ArcSwapOption` or
`ArcSwapAny`.

# Type parameters

* `T`: The smart pointer to be kept inside. This crate provides implementation for `Arc<_>` and
  `Option<Arc<_>>` (`Rc` too, but that one is not practically useful). But third party could
  provide implementations of the [`RefCnt`] trait and plug in others.
* `S`: Chooses the [strategy] used to protect the data inside. They come with various
  performance trade offs, the default [`DefaultStrategy`] is good rule of thumb for most use
  cases.

# Examples

```rust
# use std::sync::Arc;
# use arc_swap::ArcSwap;
let arc = Arc::new(42);
let arc_swap = ArcSwap::from(arc);
assert_eq!(42, **arc_swap.load());
// It can be read multiple times
assert_eq!(42, **arc_swap.load());

// Put a new one in there
let new_arc = Arc::new(0);
assert_eq!(42, *arc_swap.swap(new_arc));
assert_eq!(0, **arc_swap.load());
```

[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`from`]: https://doc.rust-lang.org/nightly/std/convert/trait.From.html#tymethod.from
[`RefCnt`]: trait.RefCnt.html

```rust
pub struct ArcSwapAny<T: RefCnt, S: Strategy<T> = DefaultStrategy> {
    pub(crate) ptr: std::sync::atomic::AtomicPtr<<T as >::Base>,
    pub(crate) _phantom_arc: std::marker::PhantomData<T>,
    pub(crate) strategy: S,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ptr` | `std::sync::atomic::AtomicPtr<<T as >::Base>` | The actual pointer, extracted from the Arc. |
| `_phantom_arc` | `std::marker::PhantomData<T>` | We are basically an Arc in disguise. Inherit parameters from Arc by pretending to contain<br>it. |
| `strategy` | `S` | Strategy to protect the data. |

#### Implementations

##### Methods

- ```rust
  pub fn new(val: T) -> Self
where
    S: Default { /* ... */ }
  ```
  Constructs a new storage.

- ```rust
  pub fn with_strategy(val: T, strategy: S) -> Self { /* ... */ }
  ```
  Constructs a new storage while customizing the protection strategy.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Extracts the value inside.

- ```rust
  pub fn load_full(self: &Self) -> T { /* ... */ }
  ```
  Loads the value.

- ```rust
  pub fn load(self: &Self) -> Guard<T, S> { /* ... */ }
  ```
  Provides a temporary borrow of the object inside.

- ```rust
  pub fn store(self: &Self, val: T) { /* ... */ }
  ```
  Replaces the value inside this instance.

- ```rust
  pub fn swap(self: &Self, new: T) -> T { /* ... */ }
  ```
  Exchanges the value inside this instance.

- ```rust
  pub fn compare_and_swap<C>(self: &Self, current: C, new: T) -> Guard<T, S>
where
    C: AsRaw<<T as >::Base>,
    S: CaS<T> { /* ... */ }
  ```
  Swaps the stored Arc if it equals to `current`.

- ```rust
  pub fn rcu<R, F>(self: &Self, f: F) -> T
where
    F: FnMut(&T) -> R,
    R: Into<T>,
    S: CaS<T> { /* ... */ }
  ```
  Read-Copy-Update of the pointer inside.

- ```rust
  pub fn map<I, R, F>(self: &Self, f: F) -> Map<&Self, I, F>
where
    F: Fn(&I) -> &R,
    Self: Access<I> { /* ... */ }
  ```
  Provides an access to an up to date projection of the carried data.

- ```rust
  pub fn from_pointee(val: T) -> Self
where
    S: Default { /* ... */ }
  ```
  A convenience constructor directly from the pointed-to value.

- ```rust
  pub fn from_pointee<V: Into<Option<T>>>(val: V) -> Self
where
    S: Default { /* ... */ }
  ```
  A convenience constructor directly from a pointed-to value.

- ```rust
  pub fn empty() -> Self
where
    S: Default { /* ... */ }
  ```
  A convenience constructor for an empty value.

##### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(val: T) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Access**
  - ```rust
    fn load(self: &Self) -> <Self as >::Guard { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self) -> <Self as >::Guard { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self) -> <Self as >::Guard { /* ... */ }
    ```

- **Send**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

- **DynAccess**
  - ```rust
    fn load(self: &Self) -> DynGuard<T> { /* ... */ }
    ```

- **UnwindSafe**
### Type Alias `ArcSwap`

An atomic storage for `Arc`.

This is a type alias only. Most of its methods are described on
[`ArcSwapAny`](struct.ArcSwapAny.html).

```rust
pub type ArcSwap<T> = ArcSwapAny<std::sync::Arc<T>>;
```

### Type Alias `ArcSwapOption`

An atomic storage for `Option<Arc>`.

This is very similar to [`ArcSwap`](type.ArcSwap.html), but allows storing NULL values, which
is useful in some situations.

This is a type alias only. Most of the methods are described on
[`ArcSwapAny`](struct.ArcSwapAny.html). Even though the examples there often use `ArcSwap`,
they are applicable to `ArcSwapOption` with appropriate changes.

# Examples

```
use std::sync::Arc;
use arc_swap::ArcSwapOption;

let shared = ArcSwapOption::from(None);
assert!(shared.load_full().is_none());
assert!(shared.swap(Some(Arc::new(42))).is_none());
assert_eq!(42, **shared.load_full().as_ref().unwrap());
```

```rust
pub type ArcSwapOption<T> = ArcSwapAny<Option<std::sync::Arc<T>>>;
```

### Type Alias `IndependentArcSwap`

An atomic storage that doesn't share the internal generation locks with others.

This makes it bigger and it also might suffer contention (on the HW level) if used from many
threads at once. On the other hand, it can't block writes in other instances.

See the [`IndependentStrategy`] for further details.

```rust
pub type IndependentArcSwap<T> = ArcSwapAny<std::sync::Arc<T>, IndependentStrategy>;
```

## Functions

### Function `ptr_eq`

**Attributes:**

- `#[allow(clippy::needless_pass_by_value)]`

Comparison of two pointer-like things.

```rust
pub(crate) fn ptr_eq<Base, A, B>(a: A, b: B) -> bool
where
    A: AsRaw<Base>,
    B: AsRaw<Base> { /* ... */ }
```

## Re-exports

### Re-export `AsRaw`

```rust
pub use crate::as_raw::AsRaw;
```

### Re-export `Cache`

```rust
pub use crate::cache::Cache;
```

### Re-export `RefCnt`

```rust
pub use crate::ref_cnt::RefCnt;
```

### Re-export `DefaultStrategy`

```rust
pub use crate::strategy::DefaultStrategy;
```

### Re-export `IndependentStrategy`

```rust
pub use crate::strategy::IndependentStrategy;
```

