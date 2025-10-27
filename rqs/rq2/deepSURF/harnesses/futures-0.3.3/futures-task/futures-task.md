# Crate Documentation

**Version:** 0.3.3

**Format Version:** 39

# Module `futures_task`

Tools for working with tasks.

## Modules

## Module `spawn`

```rust
pub(crate) mod spawn { /* ... */ }
```

### Modules

## Module `if_alloc`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub(in ::spawn) mod if_alloc { /* ... */ }
```

### Types

#### Struct `SpawnError`

An error that occurred during spawning.

```rust
pub struct SpawnError {
    pub(in ::spawn) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_priv` | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub fn shutdown() -> Self { /* ... */ }
  ```
  Spawning failed because the executor has been shut down.

- ```rust
  pub fn is_shutdown(self: &Self) -> bool { /* ... */ }
  ```
  Check whether spawning failed to the executor being shut down.

###### Trait Implementations

- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Error**
### Traits

#### Trait `Spawn`

The `Spawn` trait allows for pushing futures onto an executor that will
run them to completion.

```rust
pub trait Spawn {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `spawn_obj`: Spawns a future that will be run to completion.

##### Provided Methods

- ```rust
  fn status(self: &Self) -> Result<(), SpawnError> { /* ... */ }
  ```
  Determines whether the executor is able to spawn new tasks.

##### Implementations

This trait is implemented for the following types:

- `&Sp` with <Sp: ?Sized + Spawn>
- `&mut Sp` with <Sp: ?Sized + Spawn>
- `alloc::boxed::Box<Sp>` with <Sp: ?Sized + Spawn>
- `alloc::rc::Rc<Sp>` with <Sp: ?Sized + Spawn>
- `alloc::sync::Arc<Sp>` with <Sp: ?Sized + Spawn>

#### Trait `LocalSpawn`

The `LocalSpawn` is similar to [`Spawn`], but allows spawning futures
that don't implement `Send`.

```rust
pub trait LocalSpawn {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `spawn_local_obj`: Spawns a future that will be run to completion.

##### Provided Methods

- ```rust
  fn status_local(self: &Self) -> Result<(), SpawnError> { /* ... */ }
  ```
  Determines whether the executor is able to spawn new tasks.

##### Implementations

This trait is implemented for the following types:

- `&Sp` with <Sp: ?Sized + LocalSpawn>
- `&mut Sp` with <Sp: ?Sized + LocalSpawn>
- `alloc::boxed::Box<Sp>` with <Sp: ?Sized + LocalSpawn>
- `alloc::rc::Rc<Sp>` with <Sp: ?Sized + LocalSpawn>
- `alloc::sync::Arc<Sp>` with <Sp: ?Sized + LocalSpawn>

## Module `arc_wake`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub(crate) mod arc_wake { /* ... */ }
```

### Traits

#### Trait `ArcWake`

A way of waking up a specific task.

By implementing this trait, types that are expected to be wrapped in an `Arc`
can be converted into [`Waker`] objects.
Those Wakers can be used to signal executors that a task it owns
is ready to be `poll`ed again.

Currently, there are two ways to convert `ArcWake` into [`Waker`]:

* [`waker`](super::waker()) converts `Arc<impl ArcWake>` into [`Waker`].
* [`waker_ref`](super::waker_ref()) converts `&Arc<impl ArcWake>` into [`WakerRef`] that
  provides access to a [`&Waker`][`Waker`].

[`Waker`]: std::task::Waker
[`WakerRef`]: super::WakerRef

```rust
pub trait ArcWake: Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wake_by_ref`: Indicates that the associated task is ready to make progress and should

##### Provided Methods

- ```rust
  fn wake(self: Arc<Self>) { /* ... */ }
  ```
  Indicates that the associated task is ready to make progress and should

## Module `waker`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub(crate) mod waker { /* ... */ }
```

### Functions

#### Function `waker_vtable`

```rust
pub(crate) fn waker_vtable<W: ArcWake>() -> &''static core::task::RawWakerVTable { /* ... */ }
```

#### Function `waker`

Creates a [`Waker`] from an `Arc<impl ArcWake>`.

The returned [`Waker`] will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

```rust
pub fn waker<W>(wake: alloc::sync::Arc<W>) -> core::task::Waker
where
    W: ArcWake { /* ... */ }
```

#### Function `increase_refcount`

**Attributes:**

- `#[allow(clippy::redundant_clone)]`

```rust
pub(in ::waker) unsafe fn increase_refcount<T: ArcWake>(data: *const ()) { /* ... */ }
```

#### Function `clone_arc_raw`

```rust
pub(in ::waker) unsafe fn clone_arc_raw<T: ArcWake>(data: *const ()) -> core::task::RawWaker { /* ... */ }
```

#### Function `wake_arc_raw`

```rust
pub(in ::waker) unsafe fn wake_arc_raw<T: ArcWake>(data: *const ()) { /* ... */ }
```

#### Function `wake_by_ref_arc_raw`

```rust
pub(in ::waker) unsafe fn wake_by_ref_arc_raw<T: ArcWake>(data: *const ()) { /* ... */ }
```

#### Function `drop_arc_raw`

```rust
pub(in ::waker) unsafe fn drop_arc_raw<T: ArcWake>(data: *const ()) { /* ... */ }
```

## Module `waker_ref`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub(crate) mod waker_ref { /* ... */ }
```

### Types

#### Struct `WakerRef`

A [`Waker`] that is only valid for a given lifetime.

Note: this type implements [`Deref<Target = Waker>`](std::ops::Deref),
so it can be used to get a `&Waker`.

```rust
pub struct WakerRef<''a> {
    pub(in ::waker_ref) waker: core::mem::ManuallyDrop<core::task::Waker>,
    pub(in ::waker_ref) _marker: core::marker::PhantomData<&''a ()>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `waker` | `core::mem::ManuallyDrop<core::task::Waker>` |  |
| `_marker` | `core::marker::PhantomData<&''a ()>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(waker: &''a Waker) -> Self { /* ... */ }
  ```
  Create a new [`WakerRef`] from a [`Waker`] reference.

- ```rust
  pub fn new_unowned(waker: ManuallyDrop<Waker>) -> Self { /* ... */ }
  ```
  Create a new [`WakerRef`] from a [`Waker`] that must not be dropped.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Unpin**
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Waker { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
### Functions

#### Function `waker_ref`

**Attributes:**

- `#[inline]`

Creates a reference to a [`Waker`] from a reference to `Arc<impl ArcWake>`.

The resulting [`Waker`] will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

```rust
pub fn waker_ref<W>(wake: &alloc::sync::Arc<W>) -> WakerRef<''_>
where
    W: ArcWake { /* ... */ }
```

## Module `future_obj`

```rust
pub(crate) mod future_obj { /* ... */ }
```

### Modules

## Module `if_alloc`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub(in ::future_obj) mod if_alloc { /* ... */ }
```

### Types

#### Struct `LocalFutureObj`

A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + 'a>`.

This custom trait object was introduced as currently it is not possible to
take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
contexts.

```rust
pub struct LocalFutureObj<''a, T> {
    pub(in ::future_obj) future: *mut dyn Future<Output = T> + ''static,
    pub(in ::future_obj) drop_fn: unsafe fn(*mut dyn Future<Output = T> + ''static),
    pub(in ::future_obj) _marker: core::marker::PhantomData<&''a ()>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `future` | `*mut dyn Future<Output = T> + ''static` |  |
| `drop_fn` | `unsafe fn(*mut dyn Future<Output = T> + ''static)` |  |
| `_marker` | `core::marker::PhantomData<&''a ()>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<F: UnsafeFutureObj<''a, T> + ''a>(f: F) -> LocalFutureObj<''a, T> { /* ... */ }
  ```
  Create a `LocalFutureObj` from a custom trait object representation.

- ```rust
  pub unsafe fn into_future_obj(self: Self) -> FutureObj<''a, T> { /* ... */ }
  ```
  Converts the `LocalFutureObj` into a `FutureObj`.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(f: FutureObj<''a, T>) -> LocalFutureObj<''a, T> { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Box<F>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Box<dyn Future<Output = ()> + ''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Pin<Box<F>>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Pin<Box<dyn Future<Output = ()> + ''a>>) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<T> { /* ... */ }
    ```

- **UnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
#### Struct `FutureObj`

A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + Send + 'a>`.

This custom trait object was introduced as currently it is not possible to
take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
contexts.

You should generally not need to use this type outside of `no_std` or when
implementing `Spawn`, consider using `BoxFuture` instead.

```rust
pub struct FutureObj<''a, T>(pub(in ::future_obj) LocalFutureObj<''a, T>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `LocalFutureObj<''a, T>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<F: UnsafeFutureObj<''a, T> + Send>(f: F) -> FutureObj<''a, T> { /* ... */ }
  ```
  Create a `FutureObj` from a custom trait object representation.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<T> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(f: FutureObj<''a, T>) -> LocalFutureObj<''a, T> { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Box<F>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Box<dyn Future<Output = ()> + Send + ''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Pin<Box<F>>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(boxed: Pin<Box<dyn Future<Output = ()> + Send + ''a>>) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `UnsafeFutureObj`

A custom implementation of a future trait object for `FutureObj`, providing
a vtable with drop support.

This custom representation is typically used only in `no_std` contexts,
where the default `Box`-based implementation is not available.

# Safety

See the safety notes on individual methods for what guarantees an
implementor must provide.

```rust
pub unsafe trait UnsafeFutureObj<''a, T>: ''a {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `into_raw`: Convert an owned instance into a (conceptually owned) fat pointer.
- `drop`: Drops the future represented by the given fat pointer.

##### Implementations

This trait is implemented for the following types:

- `&''a mut F` with <''a, T, F>
- `&''a mut dyn Future<Output = T> + Unpin + ''a` with <''a, T>
- `core::pin::Pin<&''a mut F>` with <''a, T, F>
- `core::pin::Pin<&''a mut dyn Future<Output = T> + ''a>` with <''a, T>
- `alloc::boxed::Box<F>` with <''a, T, F>
- `alloc::boxed::Box<dyn Future<Output = T> + ''a>` with <''a, T: ''a>
- `alloc::boxed::Box<dyn Future<Output = T> + Send + ''a>` with <''a, T: ''a>
- `core::pin::Pin<alloc::boxed::Box<F>>` with <''a, T, F>
- `core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T> + ''a>>` with <''a, T: ''a>
- `core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + ''a>>` with <''a, T: ''a>

### Functions

#### Function `remove_future_lifetime`

**Attributes:**

- `#[allow(single_use_lifetimes)]`
- `#[allow(clippy::transmute_ptr_to_ptr)]`

```rust
pub(in ::future_obj) unsafe fn remove_future_lifetime<''a, T>(ptr: *mut dyn Future<Output = T> + ''a) -> *mut dyn Future<Output = T> + ''static { /* ... */ }
```

#### Function `remove_drop_lifetime`

**Attributes:**

- `#[allow(single_use_lifetimes)]`

```rust
pub(in ::future_obj) unsafe fn remove_drop_lifetime<''a, T>(ptr: unsafe fn(*mut dyn Future<Output = T> + ''a)) -> unsafe fn(*mut dyn Future<Output = T> + ''static) { /* ... */ }
```

## Module `noop_waker`

Utilities for creating zero-cost wakers that don't do anything.

```rust
pub(crate) mod noop_waker { /* ... */ }
```

### Functions

#### Function `noop_clone`

```rust
pub(in ::noop_waker) unsafe fn noop_clone(_data: *const ()) -> core::task::RawWaker { /* ... */ }
```

#### Function `noop`

```rust
pub(in ::noop_waker) unsafe fn noop(_data: *const ()) { /* ... */ }
```

#### Function `noop_raw_waker`

```rust
pub(in ::noop_waker) fn noop_raw_waker() -> core::task::RawWaker { /* ... */ }
```

#### Function `noop_waker`

**Attributes:**

- `#[inline]`

Create a new [`Waker`] which does
nothing when `wake()` is called on it.

# Examples

```
use futures::task::noop_waker;
let waker = noop_waker();
waker.wake();
```

```rust
pub fn noop_waker() -> core::task::Waker { /* ... */ }
```

#### Function `noop_waker_ref`

**Attributes:**

- `#[inline]`
- `#[cfg(feature = "std")]`

Get a static reference to a [`Waker`] which
does nothing when `wake()` is called on it.

# Examples

```
use futures::task::noop_waker_ref;
let waker = noop_waker_ref();
waker.wake_by_ref();
```

```rust
pub fn noop_waker_ref() -> &''static core::task::Waker { /* ... */ }
```

### Constants and Statics

#### Constant `NOOP_WAKER_VTABLE`

```rust
pub(in ::noop_waker) const NOOP_WAKER_VTABLE: core::task::RawWakerVTable = _;
```

## Macros

### Macro `cfg_target_has_atomic`

```rust
pub(crate) macro_rules! cfg_target_has_atomic {
    /* macro_rules! cfg_target_has_atomic {
    ($($item:item)*) => { ... };
} */
}
```

## Re-exports

### Re-export `Spawn`

```rust
pub use crate::spawn::Spawn;
```

### Re-export `SpawnError`

```rust
pub use crate::spawn::SpawnError;
```

### Re-export `LocalSpawn`

```rust
pub use crate::spawn::LocalSpawn;
```

### Re-export `ArcWake`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub use crate::arc_wake::ArcWake;
```

### Re-export `waker`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub use crate::waker::waker;
```

### Re-export `waker_ref`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub use crate::waker_ref::waker_ref;
```

### Re-export `WakerRef`

**Attributes:**

- `#[cfg(feature = "alloc")]`

```rust
pub use crate::waker_ref::WakerRef;
```

### Re-export `FutureObj`

```rust
pub use crate::future_obj::FutureObj;
```

### Re-export `LocalFutureObj`

```rust
pub use crate::future_obj::LocalFutureObj;
```

### Re-export `UnsafeFutureObj`

```rust
pub use crate::future_obj::UnsafeFutureObj;
```

### Re-export `noop_waker`

```rust
pub use crate::noop_waker::noop_waker;
```

### Re-export `noop_waker_ref`

**Attributes:**

- `#[cfg(feature = "std")]`

```rust
pub use crate::noop_waker::noop_waker_ref;
```

### Re-export `Context`

```rust
pub use core::task::Context;
```

### Re-export `Poll`

```rust
pub use core::task::Poll;
```

### Re-export `Waker`

```rust
pub use core::task::Waker;
```

### Re-export `RawWaker`

```rust
pub use core::task::RawWaker;
```

### Re-export `RawWakerVTable`

```rust
pub use core::task::RawWakerVTable;
```

