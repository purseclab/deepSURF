# Crate Documentation

**Version:** 1.0.8

**Format Version:** 39

# Module `stackvector`

Vector-like class allocated entirely on the stack.

Shallow wrapper around an underlying `Array`, which panics if the
array bounds are exceeded.

# no_std support

By default, `smallvec` depends on `libstd`. However, it can be configured to use the unstable
`liballoc` API instead, for use on platforms that have `liballoc` but not `libstd`.  This
configuration is currently unstable and is not guaranteed to work on all versions of Rust.

To depend on `smallvec` without `libstd`, use `default-features = false` in the `smallvec`
section of Cargo.toml to disable its `"std"` feature.

Adapted from Servo's smallvec:
    https://github.com/servo/rust-smallve

StackVec is distributed under the same terms as the smallvec and
lexical, that is, it is dual licensed under either the MIT or Apache
2.0 license.

## Modules

## Module `lib`

Facade around the core features for name mangling.

```rust
pub(crate) mod lib { /* ... */ }
```

## Types

### Struct `Drain`

An iterator that removes the items from a `StackVec` and yields them by value.

Returned from [`StackVec::drain`][1].

[1]: struct.StackVec.html#method.drain

```rust
pub struct Drain<''a, T: ''a> {
    pub(crate) iter: slice::IterMut<''a, T>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `slice::IterMut<''a, T>` |  |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **ExactSizeIterator**
- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `SetLenOnDrop`

Set the length of the vec when the `SetLenOnDrop` value goes out of scope.

Copied from https://github.com/rust-lang/rust/pull/36355

```rust
pub(crate) struct SetLenOnDrop<''a> {
    pub(crate) len: &''a mut usize,
    pub(crate) local_len: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `len` | `&''a mut usize` |  |
| `local_len` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn new(len: &''a mut usize) -> Self { /* ... */ }
  ```

- ```rust
  pub(crate) fn increment_len(self: &mut Self, increment: usize) { /* ... */ }
  ```

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

### Struct `StackVec`

A `Vec`-like container that stores elements on the stack.

The amount of data that a `StackVec` can store inline depends on its backing store. The backing
store can be any type that implements the `Array` trait; usually it is a small fixed-sized
array.  For example a `StackVec<[u64; 8]>` can hold up to eight 64-bit integers inline.

## Example

```rust,should_panic
use stackvector::StackVec;
let mut v = StackVec::<[u8; 4]>::new(); // initialize an empty vector

// The vector can hold up to 4 items without spilling onto the heap.
v.extend(0..4);
assert_eq!(v.len(), 4);

// Pushing another element will force the buffer to spill and panic:
v.push(4);
```

```rust
pub struct StackVec<A: Array> {
    pub length: usize,
    pub data: mem::ManuallyDrop<A>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `length` | `usize` |  |
| `data` | `mem::ManuallyDrop<A>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> StackVec<A> { /* ... */ }
  ```
  Construct an empty vector

- ```rust
  pub fn from_vec(vec: Vec<<A as >::Item>) -> StackVec<A> { /* ... */ }
  ```
  Construct a new `StackVec` from a `Vec<A::Item>`.

- ```rust
  pub unsafe fn from_vec_unchecked(vec: Vec<<A as >::Item>) -> StackVec<A> { /* ... */ }
  ```
  Construct a new `StackVec` from a `Vec<A::Item>` without bounds checking.

- ```rust
  pub fn from_buf(buf: A) -> StackVec<A> { /* ... */ }
  ```
  Constructs a new `StackVec` on the stack from an `A` without

- ```rust
  pub fn from_buf_and_len(buf: A, len: usize) -> StackVec<A> { /* ... */ }
  ```
  Constructs a new `StackVec` on the stack from an `A` without

- ```rust
  pub unsafe fn from_buf_and_len_unchecked(buf: A, len: usize) -> StackVec<A> { /* ... */ }
  ```
  Constructs a new `StackVec` on the stack from an `A` without

- ```rust
  pub unsafe fn set_len(self: &mut Self, new_len: usize) { /* ... */ }
  ```
  Sets the length of a vector.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  The number of elements stored in the vector.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  If the vector is empty.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  The number of items the vector can hold.

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, <A as >::Item> { /* ... */ }
  ```
  Empty the vector and return an iterator over its former contents.

- ```rust
  pub fn push(self: &mut Self, value: <A as >::Item) { /* ... */ }
  ```
  Append an item to the vector.

- ```rust
  pub fn pop(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
  ```
  Remove an item from the end of the vector and return it, or None if empty.

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```
  Shorten the vector, keeping the first `len` elements and dropping the rest.

- ```rust
  pub fn as_slice(self: &Self) -> &[<A as >::Item] { /* ... */ }
  ```
  Extracts a slice containing the entire vector.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
  ```
  Extracts a mutable slice of the entire vector.

- ```rust
  pub fn swap_remove(self: &mut Self, index: usize) -> <A as >::Item { /* ... */ }
  ```
  Remove the element at position `index`, replacing it with the last element.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Remove all elements from the vector.

- ```rust
  pub fn remove(self: &mut Self, index: usize) -> <A as >::Item { /* ... */ }
  ```
  Remove and return the element at position `index`, shifting all elements after it to the

- ```rust
  pub fn insert(self: &mut Self, index: usize, element: <A as >::Item) { /* ... */ }
  ```
  Insert an element at position `index`, shifting all elements after it to the right.

- ```rust
  pub fn insert_many<I: iter::IntoIterator<Item = <A as >::Item>>(self: &mut Self, index: usize, iterable: I) { /* ... */ }
  ```
  Insert multiple elements at position `index`, shifting all following elements toward the

- ```rust
  pub fn into_vec(self: Self) -> Vec<<A as >::Item> { /* ... */ }
  ```
  Convert a StackVec to a Vec.

- ```rust
  pub fn into_inner(self: Self) -> Result<A, Self> { /* ... */ }
  ```
  Convert the StackVec into an `A`.

- ```rust
  pub fn retain<F: FnMut(&mut <A as >::Item) -> bool>(self: &mut Self, f: F) { /* ... */ }
  ```
  Retains only the elements specified by the predicate.

- ```rust
  pub fn dedup(self: &mut Self)
where
    <A as >::Item: PartialEq<<A as >::Item> { /* ... */ }
  ```
  Removes consecutive duplicate elements.

- ```rust
  pub fn dedup_by<F>(self: &mut Self, same_bucket: F)
where
    F: FnMut(&mut <A as >::Item, &mut <A as >::Item) -> bool { /* ... */ }
  ```
  Removes consecutive duplicate elements using the given equality relation.

- ```rust
  pub fn dedup_by_key<F, K>(self: &mut Self, key: F)
where
    F: FnMut(&mut <A as >::Item) -> K,
    K: PartialEq<K> { /* ... */ }
  ```
  Removes consecutive elements that map to the same key.

- ```rust
  pub fn from_slice(slice: &[<A as >::Item]) -> Self { /* ... */ }
  ```
  Copy the elements from a slice into a new `StackVec`.

- ```rust
  pub fn insert_from_slice(self: &mut Self, index: usize, slice: &[<A as >::Item]) { /* ... */ }
  ```
  Copy elements from a slice into the vector at position `index`, shifting any following

- ```rust
  pub fn extend_from_slice(self: &mut Self, slice: &[<A as >::Item]) { /* ... */ }
  ```
  Copy elements from a slice and append them to the vector.

- ```rust
  pub fn resize(self: &mut Self, len: usize, value: <A as >::Item) { /* ... */ }
  ```
  Resizes the vector so that its length is equal to `len`.

- ```rust
  pub fn from_elem(elem: <A as >::Item, n: usize) -> Self { /* ... */ }
  ```
  Creates a `StackVec` with `n` copies of `elem`.

##### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &<A as >::Item { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::Range<usize>) -> &[<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::RangeFrom<usize>) -> &[<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::RangeFull) -> &[<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::RangeTo<usize>) -> &[<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::RangeInclusive<usize>) -> &[<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::RangeToInclusive<usize>) -> &[<A as >::Item] { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: iter::IntoIterator<Item = <A as >::Item>>(self: &mut Self, iterable: I) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [<A as >::Item]) -> StackVec<A> { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<<A as >::Item>) -> StackVec<A> { /* ... */ }
    ```

  - ```rust
    fn from(array: A) -> StackVec<A> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: usize) -> &mut <A as >::Item { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::Range<usize>) -> &mut [<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::RangeFrom<usize>) -> &mut [<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::RangeFull) -> &mut [<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::RangeTo<usize>) -> &mut [<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::RangeInclusive<usize>) -> &mut [<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::RangeToInclusive<usize>) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **UnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &StackVec<A>) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **VecLike**
  - ```rust
    fn push(self: &mut Self, value: <A as >::Item) { /* ... */ }
    ```

  - ```rust
    fn pop(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StackVec<B>) -> bool { /* ... */ }
    ```

  - ```rust
    fn ne(self: &Self, other: &StackVec<B>) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &StackVec<A>) -> cmp::Ordering { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: iter::IntoIterator<Item = <A as >::Item>>(iterable: I) -> StackVec<A> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> StackVec<A> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StackVec<A> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Receiver**
- **Eq**
- **ExtendFromSlice**
  - ```rust
    fn extend_from_slice(self: &mut Self, other: &[<A as >::Item]) { /* ... */ }
    ```

### Struct `IntoIter`

An iterator that consumes a `StackVec` and yields its items by value.

Returned from [`StackVec::into_iter`][1].

[1]: struct.StackVec.html#method.into_iter

```rust
pub struct IntoIter<A: Array> {
    pub(crate) data: StackVec<A>,
    pub(crate) current: usize,
    pub(crate) end: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `StackVec<A>` |  |
| `current` | `usize` |  |
| `end` | `usize` |  |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
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

- **ExactSizeIterator**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **Unpin**
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

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
## Traits

### Trait `PointerMethods`

```rust
pub(crate) trait PointerMethods {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `padd`

#### Implementations

This trait is implemented for the following types:

- `*const T` with <T>
- `*mut T` with <T>

### Trait `Array`

Types that can be used as the backing store for a StackVec

```rust
pub unsafe trait Array {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Item`: The type of the array's elements.

##### Required Methods

- `size`: Returns the number of items the array can hold.
- `ptr`: Returns a pointer to the first element of the array.
- `ptr_mut`: Returns a mutable pointer to the first element of the array.

#### Implementations

This trait is implemented for the following types:

- `[T; 0]` with <T>
- `[T; 1]` with <T>
- `[T; 2]` with <T>
- `[T; 3]` with <T>
- `[T; 4]` with <T>
- `[T; 5]` with <T>
- `[T; 6]` with <T>
- `[T; 7]` with <T>
- `[T; 8]` with <T>
- `[T; 9]` with <T>
- `[T; 10]` with <T>
- `[T; 11]` with <T>
- `[T; 12]` with <T>
- `[T; 13]` with <T>
- `[T; 14]` with <T>
- `[T; 15]` with <T>
- `[T; 16]` with <T>
- `[T; 20]` with <T>
- `[T; 24]` with <T>
- `[T; 32]` with <T>
- `[T; 36]` with <T>
- `[T; 64]` with <T>
- `[T; 128]` with <T>
- `[T; 256]` with <T>
- `[T; 512]` with <T>
- `[T; 1024]` with <T>
- `[T; 2048]` with <T>
- `[T; 4096]` with <T>
- `[T; 8192]` with <T>
- `[T; 16384]` with <T>
- `[T; 32768]` with <T>
- `[T; 65536]` with <T>
- `[T; 131072]` with <T>
- `[T; 262144]` with <T>
- `[T; 524288]` with <T>
- `[T; 1048576]` with <T>

### Trait `VecLike`

**Attributes:**

- `#[deprecated(note = "Use `Extend` and `Deref<[T]>` instead")]`

**⚠️ Deprecated**: Use `Extend` and `Deref<[T]>` instead

Common operations implemented by both `Vec` and `StackVec`.

This can be used to write generic code that works with both `Vec` and `StackVec`.

## Example

```rust
use stackvector::{VecLike, StackVec};

fn initialize<V: VecLike<u8>>(v: &mut V) {
    for i in 0..5 {
        v.push(i);
    }
}

let mut vec = Vec::new();
initialize(&mut vec);

let mut stack_vec = StackVec::<[u8; 8]>::new();
initialize(&mut stack_vec);
```

```rust
pub trait VecLike<T>: ops::Index<usize, Output = T> + ops::IndexMut<usize> + ops::Index<ops::Range<usize>, Output = [T]> + ops::IndexMut<ops::Range<usize>> + ops::Index<ops::RangeFrom<usize>, Output = [T]> + ops::IndexMut<ops::RangeFrom<usize>> + ops::Index<ops::RangeTo<usize>, Output = [T]> + ops::IndexMut<ops::RangeTo<usize>> + ops::Index<ops::RangeFull, Output = [T]> + ops::IndexMut<ops::RangeFull> + ops::DerefMut<Target = [T]> + Extend<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `push`: Append an element to the vector.
- `pop`: Pop an element from the end of the vector.

#### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>
- `StackVec<A>` with <A: Array>

### Trait `ExtendFromSlice`

Trait to be implemented by a collection that can be extended from a slice

## Example

```rust
use stackvector::{ExtendFromSlice, StackVec};

fn initialize<V: ExtendFromSlice<u8>>(v: &mut V) {
    v.extend_from_slice(b"Test!");
}

let mut vec = Vec::new();
initialize(&mut vec);
assert_eq!(&vec, b"Test!");

let mut stack_vec = StackVec::<[u8; 8]>::new();
initialize(&mut stack_vec);
assert_eq!(&stack_vec as &[_], b"Test!");
```

```rust
pub trait ExtendFromSlice<T> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `extend_from_slice`: Extends a collection from a slice of its element type

#### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T: Clone>
- `StackVec<A>` with <A: Array>

## Macros

### Macro `impl_array`

```rust
pub(crate) macro_rules! impl_array {
    /* macro_rules! impl_array {
    ($($size:expr),+) => { ... };
} */
}
```

### Macro `impl_index`

```rust
pub(crate) macro_rules! impl_index {
    /* macro_rules! impl_index {
    ($index_type: ty, $output_type: ty) => { ... };
} */
}
```

### Macro `stackvec`

**Attributes:**

- `#[macro_export]`

Creates a [`StackVec`] containing the arguments.

`stackvec!` allows `StackVec`s to be defined with the same syntax as array expressions.
There are two forms of this macro:

- Create a [`StackVec`] containing a given list of elements:

```
# #[macro_use] extern crate stackvector;
# use stackvector::StackVec;
# fn main() {
let v: StackVec<[_; 128]> = stackvec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
# }
```

- Create a [`StackVec`] from a given element and size:

```
# #[macro_use] extern crate stackvector;
# use stackvector::StackVec;
# fn main() {
let v: StackVec<[_; 0x8000]> = stackvec![1; 3];
assert_eq!(v, StackVec::from_buf([1, 1, 1]));
# }
```

Note that unlike array expressions this syntax supports all elements
which implement [`Clone`] and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `stackvec![Rc::new(1); 5]` will create a vector of five references
to the same boxed integer value, not five references pointing to independently
boxed integers.

```rust
pub macro_rules! stackvec {
    /* macro_rules! stackvec {
    (@one $x:expr) => { ... };
    ($elem:expr; $n:expr) => { ... };
    ($($x:expr),*$(,)*) => { ... };
} */
}
```

