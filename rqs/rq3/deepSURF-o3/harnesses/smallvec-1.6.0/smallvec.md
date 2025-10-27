# Crate Documentation

**Version:** 1.6.0

**Format Version:** 39

# Module `smallvec`

Small vectors in various sizes. These store a certain number of elements inline, and fall back
to the heap for larger allocations.  This can be a useful optimization for improving cache
locality and reducing allocator traffic for workloads that fit within the inline buffer.

## `no_std` support

By default, `smallvec` does not depend on `std`.  However, the optional
`write` feature implements the `std::io::Write` trait for vectors of `u8`.
When this feature is enabled, `smallvec` depends on `std`.

## Optional features

### `serde`

When this optional dependency is enabled, `SmallVec` implements the `serde::Serialize` and
`serde::Deserialize` traits.

### `write`

When this feature is enabled, `SmallVec<[u8; _]>` implements the `std::io::Write` trait.
This feature is not compatible with `#![no_std]` programs.

### `union`

**This feature requires Rust 1.49.**

When the `union` feature is enabled `smallvec` will track its state (inline or spilled)
without the use of an enum tag, reducing the size of the `smallvec` by one machine word.
This means that there is potentially no space overhead compared to `Vec`.
Note that `smallvec` can still be larger than `Vec` if the inline buffer is larger than two
machine words.

To use this feature add `features = ["union"]` in the `smallvec` section of Cargo.toml.
Note that this feature requires Rust 1.49.

Tracking issue: [rust-lang/rust#55149](https://github.com/rust-lang/rust/issues/55149)

### `const_generics`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

When this feature is enabled, `SmallVec` works with any arrays of any size, not just a fixed
list of sizes.

Tracking issue: [rust-lang/rust#44580](https://github.com/rust-lang/rust/issues/44580)

### `specialization`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

When this feature is enabled, `SmallVec::from(slice)` has improved performance for slices
of `Copy` types.  (Without this feature, you can use `SmallVec::from_slice` to get optimal
performance for `Copy` types.)

Tracking issue: [rust-lang/rust#31844](https://github.com/rust-lang/rust/issues/31844)

### `may_dangle`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

This feature makes the Rust compiler less strict about use of vectors that contain borrowed
references. For details, see the
[Rustonomicon](https://doc.rust-lang.org/1.42.0/nomicon/dropck.html#an-escape-hatch).

Tracking issue: [rust-lang/rust#34761](https://github.com/rust-lang/rust/issues/34761)

## Types

### Enum `CollectionAllocErr`

Error type for APIs with fallible heap allocation

```rust
pub enum CollectionAllocErr {
    CapacityOverflow,
    AllocErr {
        layout: alloc::alloc::Layout,
    },
}
```

#### Variants

##### `CapacityOverflow`

Overflow `usize::MAX` or other error during size computation

##### `AllocErr`

The allocator return an error

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `layout` | `alloc::alloc::Layout` | The layout that was passed to the allocator |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(_: LayoutErr) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
### Struct `Drain`

An iterator that removes the items from a `SmallVec` and yields them by value.

Returned from [`SmallVec::drain`][1].

[1]: struct.SmallVec.html#method.drain

```rust
pub struct Drain<''a, T: ''a + Array> {
    pub(crate) tail_start: usize,
    pub(crate) tail_len: usize,
    pub(crate) iter: slice::Iter<''a, <T as >::Item>,
    pub(crate) vec: core::ptr::NonNull<SmallVec<T>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tail_start` | `usize` |  |
| `tail_len` | `usize` |  |
| `iter` | `slice::Iter<''a, <T as >::Item>` |  |
| `vec` | `core::ptr::NonNull<SmallVec<T>>` |  |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<T as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **FusedIterator**
- **Freeze**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<T as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Enum `SmallVecData`

**Attributes:**

- `#[cfg(not(feature = "union"))]`

```rust
pub(crate) enum SmallVecData<A: Array> {
    Inline(core::mem::MaybeUninit<A>),
    Heap((*mut <A as >::Item, usize)),
}
```

#### Variants

##### `Inline`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `core::mem::MaybeUninit<A>` |  |

##### `Heap`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `(*mut <A as >::Item, usize)` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) unsafe fn inline(self: &Self) -> *const <A as >::Item { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn inline_mut(self: &mut Self) -> *mut <A as >::Item { /* ... */ }
  ```

- ```rust
  pub(crate) fn from_inline(inline: MaybeUninit<A>) -> SmallVecData<A> { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn into_inline(self: Self) -> MaybeUninit<A> { /* ... */ }
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
### Struct `SmallVec`

A `Vec`-like container that can store a small number of elements inline.

`SmallVec` acts like a vector, but can store a limited amount of data inline within the
`SmallVec` struct rather than in a separate allocation.  If the data exceeds this limit, the
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
  pub unsafe fn from_buf_and_len_unchecked(buf: MaybeUninit<A>, len: usize) -> SmallVec<A> { /* ... */ }
  ```
  Constructs a new `SmallVec` on the stack from an `A` without

- ```rust
  pub unsafe fn set_len(self: &mut Self, new_len: usize) { /* ... */ }
  ```
  Sets the length of a vector.

- ```rust
  pub(crate) fn inline_capacity() -> usize { /* ... */ }
  ```
  The maximum number of elements this vector can hold inline

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
  pub fn drain<R>(self: &mut Self, range: R) -> Drain<''_, A>
where
    R: RangeBounds<usize> { /* ... */ }
  ```
  Creates a draining iterator that removes the specified range in the vector

- ```rust
  pub fn push(self: &mut Self, value: <A as >::Item) { /* ... */ }
  ```
  Append an item to the vector.

- ```rust
  pub fn pop(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
  ```
  Remove an item from the end of the vector and return it, or None if empty.

- ```rust
  pub fn append<B>(self: &mut Self, other: &mut SmallVec<B>)
where
    B: Array<Item = <A as >::Item> { /* ... */ }
  ```
  Moves all the elements of `other` into `self`, leaving `other` empty.

- ```rust
  pub fn grow(self: &mut Self, new_cap: usize) { /* ... */ }
  ```
  Re-allocate to set the capacity to `max(new_cap, inline_size())`.

- ```rust
  pub fn try_grow(self: &mut Self, new_cap: usize) -> Result<(), CollectionAllocErr> { /* ... */ }
  ```
  Re-allocate to set the capacity to `max(new_cap, inline_size())`.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for `additional` more elements to be inserted.

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), CollectionAllocErr> { /* ... */ }
  ```
  Reserve capacity for `additional` more elements to be inserted.

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve the minimum capacity for `additional` more elements to be inserted.

- ```rust
  pub fn try_reserve_exact(self: &mut Self, additional: usize) -> Result<(), CollectionAllocErr> { /* ... */ }
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
  pub fn into_boxed_slice(self: Self) -> Box<[<A as >::Item]> { /* ... */ }
  ```
  Converts a `SmallVec` into a `Box<[T]>` without reallocating if the `SmallVec` has already spilled

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
  pub fn resize_with<F>(self: &mut Self, new_len: usize, f: F)
where
    F: FnMut() -> <A as >::Item { /* ... */ }
  ```
  Resizes the `SmallVec` in-place so that `len` is equal to `new_len`.

- ```rust
  pub unsafe fn from_raw_parts(ptr: *mut <A as >::Item, length: usize, capacity: usize) -> SmallVec<A> { /* ... */ }
  ```
  Creates a `SmallVec` directly from the raw components of another

- ```rust
  pub fn as_ptr(self: &Self) -> *const <A as >::Item { /* ... */ }
  ```
  Returns a raw pointer to the vector's buffer.

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut <A as >::Item { /* ... */ }
  ```
  Returns a raw mutable pointer to the vector's buffer.

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

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: I) -> &mut <I as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SmallVec<A> { /* ... */ }
    ```

- **Send**
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SmallVec<B>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = <A as >::Item>>(self: &mut Self, iterable: I) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SmallVec<A>) -> cmp::Ordering { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: I) -> &<I as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Receiver**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[<A as >::Item] { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SmallVec<A> { /* ... */ }
    ```

- **RefUnwindSafe**
- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = <A as >::Item>>(iterable: I) -> SmallVec<A> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SmallVec<A>) -> Option<cmp::Ordering> { /* ... */ }
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

##### Methods

- ```rust
  pub fn as_slice(self: &Self) -> &[<A as >::Item] { /* ... */ }
  ```
  Returns the remaining items of this iterator as a slice.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [<A as >::Item] { /* ... */ }
  ```
  Returns the remaining items of this iterator as a mutable slice.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

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

- **ExactSizeIterator**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<A> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<A as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **FusedIterator**
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
  pub(crate) fn get(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub(crate) fn increment_len(self: &mut Self, increment: usize) { /* ... */ }
  ```

##### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Unpin**
- **Freeze**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Traits

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

- `size`

Returns the number of items the array can hold.



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
- `[T; 17]` with <T>
- `[T; 18]` with <T>
- `[T; 19]` with <T>
- `[T; 20]` with <T>
- `[T; 21]` with <T>
- `[T; 22]` with <T>
- `[T; 23]` with <T>
- `[T; 24]` with <T>
- `[T; 25]` with <T>
- `[T; 26]` with <T>
- `[T; 27]` with <T>
- `[T; 28]` with <T>
- `[T; 29]` with <T>
- `[T; 30]` with <T>
- `[T; 31]` with <T>
- `[T; 32]` with <T>
- `[T; 36]` with <T>
- `[T; 64]` with <T>
- `[T; 96]` with <T>
- `[T; 128]` with <T>
- `[T; 256]` with <T>
- `[T; 512]` with <T>
- `[T; 1024]` with <T>
- `[T; 1536]` with <T>
- `[T; 2048]` with <T>
- `[T; 4096]` with <T>
- `[T; 8192]` with <T>
- `[T; 16384]` with <T>
- `[T; 24576]` with <T>
- `[T; 32768]` with <T>
- `[T; 65536]` with <T>
- `[T; 131072]` with <T>
- `[T; 262144]` with <T>
- `[T; 393216]` with <T>
- `[T; 524288]` with <T>
- `[T; 1048576]` with <T>

### Trait `ToSmallVec`

Convenience trait for constructing a `SmallVec`

```rust
pub trait ToSmallVec<A: Array> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `to_smallvec`

Construct a new `SmallVec` from a slice.



#### Implementations

This trait is implemented for the following types:

- `[<A as >::Item]` with <A: Array>

## Functions

### Function `infallible`

```rust
pub(crate) fn infallible<T>(result: Result<T, CollectionAllocErr>) -> T { /* ... */ }
```

### Function `layout_array`

FIXME: use `Layout::array` when we require a Rust version where it’s stable
https://github.com/rust-lang/rust/issues/55724

```rust
pub(crate) fn layout_array<T>(n: usize) -> Result<alloc::alloc::Layout, CollectionAllocErr> { /* ... */ }
```

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

### Macro `impl_array`

**Attributes:**

- `#[cfg(not(feature = "const_generics"))]`

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

