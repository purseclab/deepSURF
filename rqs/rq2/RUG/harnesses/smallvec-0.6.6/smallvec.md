# Crate Documentation

**Version:** 0.6.6

**Format Version:** 39

# Module `smallvec`

Small vectors in various sizes. These store a certain number of elements inline, and fall back
to the heap for larger allocations.  This can be a useful optimization for improving cache
locality and reducing allocator traffic for workloads that fit within the inline buffer.

## no_std support

By default, `smallvec` depends on `libstd`. However, it can be configured to use the unstable
`liballoc` API instead, for use on platforms that have `liballoc` but not `libstd`.  This
configuration is currently unstable and is not guaranteed to work on all versions of Rust.

To depend on `smallvec` without `libstd`, use `default-features = false` in the `smallvec`
section of Cargo.toml to disable its `"std"` feature.

## `union` feature

When the `union` feature is enabled `smallvec` will track its state (inline or spilled)
without the use of an enum tag, reducing the size of the `smallvec` by one machine word.
This means that there is potentially no space overhead compared to `Vec`.
Note that `smallvec` can still be larger than `Vec` if the inline buffer is larger than two
machine words.

To use this feature add `features = ["union"]` in the `smallvec` section of Cargo.toml.
Note that this feature requires a nightly compiler (for now).

## Types

### Struct `Drain`

An iterator that removes the items from a `SmallVec` and yields them by value.

Returned from [`SmallVec::drain`][1].

[1]: struct.SmallVec.html#method.drain

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

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
### Enum `SmallVecData`

**Attributes:**

- `#[cfg(not(feature = "union"))]`

```rust
pub(crate) enum SmallVecData<A: Array> {
    Inline(std::mem::ManuallyDrop<A>),
    Heap((*mut <A as >::Item, usize)),
}
```

#### Variants

##### `Inline`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::mem::ManuallyDrop<A>` |  |

##### `Heap`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `(*mut <A as >::Item, usize)` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) unsafe fn inline(self: &Self) -> &A { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn inline_mut(self: &mut Self) -> &mut A { /* ... */ }
  ```

- ```rust
  pub(crate) fn from_inline(inline: A) -> SmallVecData<A> { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn into_inline(self: Self) -> A { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn heap(self: &Self) -> (*mut <A as >::Item, usize) { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn heap_mut(self: &mut Self) -> &mut (*mut <A as >::Item, usize) { /* ... */ }
  ```

- ```rust
  pub(crate) fn from_heap(ptr: *mut <A as >::Item, len: usize) -> SmallVecData<A> { /* ... */ }
  ```

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
### Struct `SmallVec`

A `Vec`-like container that can store a small number of elements inline.

`SmallVec` acts like a vector, but can store a limited amount of data inline within the
`Smallvec` struct rather than in a separate allocation.  If the data exceeds this limit, the
`SmallVec` will "spill" its data onto the heap, allocating a new buffer to hold it.

The amount of data that a `SmallVec` can store inline depends on its backing store. The backing
store can be any type that implements the `Array` trait; usually it is a small fixed-sized
array.  For example a `SmallVec<[u64; 8]>` can hold up to eight 64-bit integers inline.

## Example

```rust
use smallvec::SmallVec;
let mut v = SmallVec::<[u8; 4]>::new(); // initialize an empty vector

// The vector can hold up to 4 items without spilling onto the heap.
v.extend(0..4);
assert_eq!(v.len(), 4);
assert!(!v.spilled());

// Pushing another element will force the buffer to spill:
v.push(4);
assert_eq!(v.len(), 5);
assert!(v.spilled());
```

```rust
pub struct SmallVec<A: Array> {
    pub(crate) capacity: usize,
    pub(crate) data: SmallVecData<A>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `capacity` | `usize` |  |
| `data` | `SmallVecData<A>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> SmallVec<A> { /* ... */ }
  ```
  Construct an empty vector

- ```rust
  pub fn with_capacity(n: usize) -> Self { /* ... */ }
  ```
  Construct an empty vector with enough capacity pre-allocated to store at least `n`

- ```rust
  pub fn from_vec(vec: Vec<<A as >::Item>) -> SmallVec<A> { /* ... */ }
  ```
  Construct a new `SmallVec` from a `Vec<A::Item>`.

- ```rust
  pub fn from_buf(buf: A) -> SmallVec<A> { /* ... */ }
  ```
  Constructs a new `SmallVec` on the stack from an `A` without

- ```rust
  pub fn from_buf_and_len(buf: A, len: usize) -> SmallVec<A> { /* ... */ }
  ```
  Constructs a new `SmallVec` on the stack from an `A` without

- ```rust
  pub unsafe fn from_buf_and_len_unchecked(buf: A, len: usize) -> SmallVec<A> { /* ... */ }
  ```
  Constructs a new `SmallVec` on the stack from an `A` without

- ```rust
  pub unsafe fn set_len(self: &mut Self, new_len: usize) { /* ... */ }
  ```
  Sets the length of a vector.

- ```rust
  pub fn inline_size(self: &Self) -> usize { /* ... */ }
  ```
  The maximum number of elements this vector can hold inline

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  The number of elements stored in the vector

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the vector is empty

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  The number of items the vector can hold without reallocating

- ```rust
  pub(crate) fn triple(self: &Self) -> (*const <A as >::Item, usize, usize) { /* ... */ }
  ```
  Returns a tuple with (data ptr, len, capacity)

- ```rust
  pub(crate) fn triple_mut(self: &mut Self) -> (*mut <A as >::Item, &mut usize, usize) { /* ... */ }
  ```
  Returns a tuple with (data ptr, len ptr, capacity)

- ```rust
  pub fn spilled(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the data has spilled into a separate heap-allocated buffer.

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
  pub fn grow(self: &mut Self, new_cap: usize) { /* ... */ }
  ```
  Re-allocate to set the capacity to `max(new_cap, inline_size())`.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for `additional` more elements to be inserted.

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve the minimum capacity for `additional` more elements to be inserted.

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrink the capacity of the vector as much as possible.

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
  pub fn insert_many<I: IntoIterator<Item = <A as >::Item>>(self: &mut Self, index: usize, iterable: I) { /* ... */ }
  ```
  Insert multiple elements at position `index`, shifting all following elements toward the

- ```rust
  pub fn into_vec(self: Self) -> Vec<<A as >::Item> { /* ... */ }
  ```
  Convert a SmallVec to a Vec, without reallocating if the SmallVec has already spilled onto

- ```rust
  pub fn into_inner(self: Self) -> Result<A, Self> { /* ... */ }
  ```
  Convert the SmallVec into an `A` if possible. Otherwise return `Err(Self)`.

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
  pub unsafe fn from_raw_parts(ptr: *mut <A as >::Item, length: usize, capacity: usize) -> SmallVec<A> { /* ... */ }
  ```
  Creates a `SmallVec` directly from the raw components of another

- ```rust
  pub fn from_slice(slice: &[<A as >::Item]) -> Self { /* ... */ }
  ```
  Copy the elements from a slice into a new `SmallVec`.

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
  Creates a `SmallVec` with `n` copies of `elem`.

##### Trait Implementations

- **Sync**
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
    fn index_mut(self: &mut Self, index: ops::RangeTo<usize>) -> &mut [<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: ops::RangeFull) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **VecLike**
  - ```rust
    fn push(self: &mut Self, value: <A as >::Item) { /* ... */ }
    ```

- **Receiver**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **ExtendFromSlice**
  - ```rust
    fn extend_from_slice(self: &mut Self, other: &[<A as >::Item]) { /* ... */ }
    ```

- **Unpin**
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SmallVec<A> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SmallVec<A> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SmallVec<A>) -> Option<cmp::Ordering> { /* ... */ }
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

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [<A as >::Item]) -> SmallVec<A> { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<<A as >::Item>) -> SmallVec<A> { /* ... */ }
    ```

  - ```rust
    fn from(array: A) -> SmallVec<A> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SmallVec<A>) -> cmp::Ordering { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Freeze**
- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = <A as >::Item>>(iterable: I) -> SmallVec<A> { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = <A as >::Item>>(self: &mut Self, iterable: I) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SmallVec<B>) -> bool { /* ... */ }
    ```

  - ```rust
    fn ne(self: &Self, other: &SmallVec<B>) -> bool { /* ... */ }
    ```

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
    fn index(self: &Self, index: ops::RangeTo<usize>) -> &[<A as >::Item] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ops::RangeFull) -> &[<A as >::Item] { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Eq**
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

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

### Struct `IntoIter`

An iterator that consumes a `SmallVec` and yields its items by value.

Returned from [`SmallVec::into_iter`][1].

[1]: struct.SmallVec.html#method.into_iter

```rust
pub struct IntoIter<A: Array> {
    pub(crate) data: SmallVec<A>,
    pub(crate) current: usize,
    pub(crate) end: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `SmallVec<A>` |  |
| `current` | `usize` |  |
| `end` | `usize` |  |

#### Implementations

##### Trait Implementations

- **ExactSizeIterator**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
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

- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Traits

### Trait `VecLike`

**Attributes:**

- `#[deprecated(note = "Use `Extend` and `Deref<[T]>` instead")]`

**⚠️ Deprecated**: Use `Extend` and `Deref<[T]>` instead

Common operations implemented by both `Vec` and `SmallVec`.

This can be used to write generic code that works with both `Vec` and `SmallVec`.

## Example

```rust
use smallvec::{VecLike, SmallVec};

fn initialize<V: VecLike<u8>>(v: &mut V) {
    for i in 0..5 {
        v.push(i);
    }
}

let mut vec = Vec::new();
initialize(&mut vec);

let mut small_vec = SmallVec::<[u8; 8]>::new();
initialize(&mut small_vec);
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

#### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>
- `SmallVec<A>` with <A: Array>

### Trait `ExtendFromSlice`

Trait to be implemented by a collection that can be extended from a slice

## Example

```rust
use smallvec::{ExtendFromSlice, SmallVec};

fn initialize<V: ExtendFromSlice<u8>>(v: &mut V) {
    v.extend_from_slice(b"Test!");
}

let mut vec = Vec::new();
initialize(&mut vec);
assert_eq!(&vec, b"Test!");

let mut small_vec = SmallVec::<[u8; 8]>::new();
initialize(&mut small_vec);
assert_eq!(&small_vec as &[_], b"Test!");
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
- `SmallVec<A>` with <A: Array>

### Trait `Array`

Types that can be used as the backing store for a SmallVec

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

## Functions

### Function `deallocate`

```rust
pub(crate) unsafe fn deallocate<T>(ptr: *mut T, capacity: usize) { /* ... */ }
```

## Macros

### Macro `debug_unreachable`

**Attributes:**

- `#[cfg(not(feature = "union"))]`

`panic!()` in debug builds, optimization hint in release.

```rust
pub(crate) macro_rules! debug_unreachable {
    /* macro_rules! debug_unreachable {
    () => { ... };
    ($e:expr) => { ... };
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

### Macro `impl_array`

```rust
pub(crate) macro_rules! impl_array {
    /* macro_rules! impl_array {
    ($($size:expr),+) => { ... };
} */
}
```

### Macro `smallvec`

**Attributes:**

- `#[macro_export]`

Creates a [`SmallVec`] containing the arguments.

`smallvec!` allows `SmallVec`s to be defined with the same syntax as array expressions.
There are two forms of this macro:

- Create a [`SmallVec`] containing a given list of elements:

```
# #[macro_use] extern crate smallvec;
# use smallvec::SmallVec;
# fn main() {
let v: SmallVec<[_; 128]> = smallvec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
# }
```

- Create a [`SmallVec`] from a given element and size:

```
# #[macro_use] extern crate smallvec;
# use smallvec::SmallVec;
# fn main() {
let v: SmallVec<[_; 0x8000]> = smallvec![1; 3];
assert_eq!(v, SmallVec::from_buf([1, 1, 1]));
# }
```

Note that unlike array expressions this syntax supports all elements
which implement [`Clone`] and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `smallvec![Rc::new(1); 5]` will create a vector of five references
to the same boxed integer value, not five references pointing to independently
boxed integers.

```rust
pub macro_rules! smallvec {
    /* macro_rules! smallvec {
    (@one $x:expr) => { ... };
    ($elem:expr; $n:expr) => { ... };
    ($($x:expr),*$(,)*) => { ... };
} */
}
```

