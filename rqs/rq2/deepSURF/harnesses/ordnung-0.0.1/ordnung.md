# Crate Documentation

**Version:** 0.0.1

**Format Version:** 39

# Module `ordnung`

# Ordnung

Fast, vector-based map implementation that preserves insertion order.

+ Map is implemented as a binary tree over a `Vec` for storage, with only
  two extra words per entry for book-keeping on 64-bit architectures.
+ A fast hash function with good random distribution is used to balance the
  tree. Ordnung makes no guarantees that the tree will be perfectly
  balanced, but key lookup should be approaching `O(log n)` in most cases.
+ Tree traversal is always breadth-first and happens over a single
  continuous block of memory, which makes it cache friendly.
+ Iterating over all entries is always `O(n)`, same as `Vec<(K, V)>`.
+ There are no buckets, so there is no need to re-bucket things when growing
  the map.

## When should you use this?

+ You need to preserve insertion order of the map.
+ Iterating over the map is very performance sensitive.
+ Your average map has fewer than 100 entries.
+ You have no a priori knowledge about the final size of the map when you
  start creating it.
+ Removing items from the map is very, very rare.

## Modules

## Module `compact`

This is meant to be API compatible drop in replacement for std [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html),
but made compact by cramming capacity and length into a single 64bit word.

```rust
use std::mem::size_of;

const WORD: usize = size_of::<usize>();

assert_eq!(size_of::<Vec<u8>>(), WORD * 3);
assert_eq!(size_of::<ordnung::compact::Vec<u8>>(), WORD * 2);
```

```rust
pub mod compact { /* ... */ }
```

### Types

#### Struct `Vec`

A contiguous growable array type, written `Vec<T>` but pronounced 'vector'.

```rust
pub struct Vec<T> {
    pub(in ::compact) ptr: core::ptr::NonNull<[T]>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ptr` | `core::ptr::NonNull<[T]>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Constructs a new, empty Vec<T>.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Constructs a new, empty Vec<T> with the specified capacity.

- ```rust
  pub fn push(self: &mut Self, val: T) { /* ... */ }
  ```
  Appends an element to the back of a collection.

- ```rust
  pub fn pop(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes the last element from a vector and returns it, or `None` if it is empty.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the vector, removing all values.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the vector.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements the vector can hold without reallocating.

- ```rust
  pub fn remove(self: &mut Self, index: usize) -> T { /* ... */ }
  ```
  Removes and returns the element at position `index` within the vector,

- ```rust
  pub const fn as_ptr(self: &Self) -> *const T { /* ... */ }
  ```
  Returns a raw pointer to the vector's buffer.

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut T { /* ... */ }
  ```
  Returns an unsafe mutable pointer to the vector's buffer.

- ```rust
  pub(in ::compact) fn set_len(self: &mut Self, len: usize) { /* ... */ }
  ```

- ```rust
  pub(in ::compact) fn parts(self: &Self) -> (usize, usize) { /* ... */ }
  ```

- ```rust
  pub(in ::compact) fn with<''a, R: ''a, F: FnOnce(&mut StdVec<T>) -> R>(self: &mut Self, f: F) -> R { /* ... */ }
  ```

- ```rust
  pub(in ::compact) fn from_stdvec_unchecked(stdvec: StdVec<T>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Vec<T>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(stdvec: StdVec<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<T>) -> Self { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[T] { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Vec<T>
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Vec<T> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

### Functions

#### Function `pack`

**Attributes:**

- `#[inline]`

```rust
pub(in ::compact) unsafe fn pack<T>(ptr: *mut T, len: usize, capacity: usize) -> core::ptr::NonNull<[T]> { /* ... */ }
```

#### Function `pack_unchecked`

**Attributes:**

- `#[inline]`

```rust
pub(in ::compact) unsafe fn pack_unchecked<T>(ptr: *mut T, len: usize, capacity: usize) -> core::ptr::NonNull<[T]> { /* ... */ }
```

### Constants and Statics

#### Constant `MASK_LO`

```rust
pub(in ::compact) const MASK_LO: usize = _;
```

#### Constant `MASK_HI`

```rust
pub(in ::compact) const MASK_HI: usize = _;
```

## Types

### Struct `Node`

```rust
pub(crate) struct Node<K, V> {
    pub key: K,
    pub hash: u64,
    pub value: V,
    pub left: core::cell::Cell<Option<core::num::NonZeroU32>>,
    pub right: core::cell::Cell<Option<core::num::NonZeroU32>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `K` |  |
| `hash` | `u64` |  |
| `value` | `V` |  |
| `left` | `core::cell::Cell<Option<core::num::NonZeroU32>>` |  |
| `right` | `core::cell::Cell<Option<core::num::NonZeroU32>>` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) const fn new(key: K, value: V, hash: u64) -> Self { /* ... */ }
  ```

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Node<K, V> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `Map`

A binary tree implementation of a string -> `JsonValue` map. You normally don't
have to interact with instances of `Object`, much more likely you will be
using the `JsonValue::Object` variant, which wraps around this struct.

```rust
pub struct Map<K, V> {
    pub(crate) store: Vec<Node<K, V>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `store` | `Vec<Node<K, V>>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `Map`.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Create a `Map` with a given capacity

- ```rust
  pub fn insert(self: &mut Self, key: K, value: V) -> Option<V> { /* ... */ }
  ```
  Inserts a key-value pair into the map.

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a reference to the value corresponding to the key.

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns `true` if the map contains a value for the specified key.

- ```rust
  pub fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the key.

- ```rust
  pub fn get_or_insert<F>(self: &mut Self, key: K, fill: F) -> &mut V
where
    F: FnOnce() -> V { /* ... */ }
  ```
  Get a mutable reference to entry at key. Inserts a new entry by

- ```rust
  pub fn remove<Q>(self: &mut Self, key: &Q) -> Option<V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Removes a key from the map, returning the value at the key if the key

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the map.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the map contains no elements.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

- ```rust
  pub(crate) fn find<Q>(self: &Self, key: &Q, hash: u64) -> FindResult<''_>
where
    K: Borrow<Q>,
    Q: Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, K, V> { /* ... */ }
  ```
  An iterator visiting all key-value pairs in insertion order.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, K, V> { /* ... */ }
  ```
  An iterator visiting all key-value pairs in insertion order, with

##### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Map<K, V> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = (IK, IV)> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, key: &Q) -> &V { /* ... */ }
    ```
    Returns a reference to the value corresponding to the supplied key.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
### Enum `FindResult`

```rust
pub(crate) enum FindResult<''find> {
    Hit(usize),
    Miss(Option<&''find core::cell::Cell<Option<core::num::NonZeroU32>>>),
}
```

#### Variants

##### `Hit`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### `Miss`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<&''find core::cell::Cell<Option<core::num::NonZeroU32>>>` |  |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Sync**
- **Freeze**
### Struct `Iter`

An iterator over the entries of a `Map`.

This struct is created by the [`iter`](./struct.Map.html#method.iter)
method on [`Map`](./struct.Map.html). See its documentation for more.

```rust
pub struct Iter<''a, K, V> {
    pub(crate) inner: slice::Iter<''a, Node<K, V>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `slice::Iter<''a, Node<K, V>>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn empty() -> Self { /* ... */ }
  ```
  Create an empty iterator that always returns `None`

##### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `IterMut`

A mutable iterator over the entries of a `Map`.

This struct is created by the [`iter_mut`](./struct.Map.html#method.iter_mut)
method on [`Map`](./struct.Map.html). See its documentation for more.

```rust
pub struct IterMut<''a, K, V> {
    pub(crate) inner: slice::IterMut<''a, Node<K, V>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `slice::IterMut<''a, Node<K, V>>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn empty() -> Self { /* ... */ }
  ```
  Create an empty iterator that always returns `None`

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
## Functions

### Function `hash_key`

**Attributes:**

- `#[inline]`

```rust
pub(crate) fn hash_key<H: Hash>(hash: H) -> u64 { /* ... */ }
```

## Re-exports

### Re-export `Vec`

```rust
pub use compact::Vec;
```

