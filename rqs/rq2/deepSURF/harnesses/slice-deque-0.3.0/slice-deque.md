# Crate Documentation

**Version:** 0.3.0

**Format Version:** 39

# Module `slice_deque`

A double-ended queue that `Deref`s into a slice.

The double-ended queue in the standard library ([`VecDeque`]) is
implemented using a growable ring buffer (`0` represents uninitialized
memory, and `T` represents one element in the queue):

```rust
// [ 0 | 0 | 0 | T | T | T | 0 ]
//               ^:head  ^:tail
```

When the queue grows beyond the end of the allocated buffer, its tail wraps
around:

```rust
// [ T | T | 0 | T | T | T | T ]
//       ^:tail  ^:head
```

As a consequence, [`VecDeque`] cannot `Deref` into a slice, since its
elements do not, in general, occupy a contiguous memory region. This
complicates the implementation and its interface (for example, there is no
`as_slice` method, but [`as_slices`] returns a pair of slices) and has
negative performance consequences (e.g. need to account for wrap around
while iterating over the elements).

This crates provides [`SliceDeque`], a double-ended queue implemented with
a growable *virtual* ring-buffer.

A virtual ring-buffer implementation is very similar to the one used in
`VecDeque`. The main difference is that a virtual ring-buffer maps two
adjacent regions of virtual memory to the same region of physical memory:

```rust
// Virtual memory:
//
//  __________region_0_________ __________region_1_________
// [ 0 | 0 | 0 | T | T | T | 0 | 0 | 0 | 0 | T | T | T | 0 ]
//               ^:head  ^:tail
//
// Physical memory:
//
// [ 0 | 0 | 0 | T | T | T | 0 ]
//               ^:head  ^:tail
```

That is, both the virtual memory regions `0` and `1` above (top) map to
the same physical memory (bottom). Just like `VecDeque`, when the queue
grows beyond the end of the allocated physical memory region, the queue
wraps around, and new elements continue to be appended at the beginning of
the queue. However, because `SliceDeque` maps the physical memory to two
adjacent memory regions, in virtual memory space the queue maintais the
ilusion of a contiguous memory layout:

```rust
// Virtual memory:
//
//  __________region_0_________ __________region_1_________
// [ T | T | 0 | T | T | T | T | T | T | 0 | T | T | T | T ]
//               ^:head              ^:tail
//
// Physical memory:
//
// [ T | T | 0 | T | T | T | T ]
//       ^:tail  ^:head
```

Since processes in many Operating Systems only deal with virtual memory
addresses, leaving the mapping to physical memory to the CPU Memory
Management Unit (MMU), [`SliceDeque`] is able to `Deref`s into a slice in
those systems.

This simplifies [`SliceDeque`]'s API and implementation, giving it a
performance advantage over [`VecDeque`] in some situations.

In general, you can think of [`SliceDeque`] as a `Vec` with `O(1)`
`pop_front` and amortized `O(1)` `push_front` methods.

The main drawbacks of [`SliceDeque`] are:

* constrained platform support: by necessity [`SliceDeque`] must use the
platform-specific virtual memory facilities of the underlying operating
system. While [`SliceDeque`] can work on all major operating systems,
currently only `MacOS X` is supported.

* no global allocator support: since the `Alloc`ator API does not support
virtual memory, to use platform-specific virtual memory support
[`SliceDeque`] must bypass the global allocator and talk directly to the
operating system. This can have negative performance consequences since
growing [`SliceDeque`] is always going to incur the cost of some system
calls.

* capacity constrained by virtual memory facilities: [`SliceDeque`] must
allocate two adjacent memory regions that map to the same region of
physical memory. Most operating systems allow this operation to be
performed exclusively on memory pages (or memory allocations that are
multiples of a memory page). As a consequence, the smalles [`SliceDeque`]
that can be created has typically a capacity of 2 memory pages, and it can
grow only to capacities that are a multiple of a memory page.

The main advantages of [`SliceDeque`] are:

* nicer API: since it `Deref`s to a slice, all operations that work on
slices are available for `SliceDeque`.

* efficient iteration: as efficient as for slices.

* simpler serialization: since one can just serialize/deserialize a single
slice.

All in all, if your double-ended queues are small (smaller than a memory
page) or they get resized very often, `VecDeque` can perform better than
[`SliceDeque`]. Otherwise, [`SliceDeque`] typically performs better (see
the benchmarks), but platform support and global allocator bypass are two
reasons to weight in against its usage.

[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
[`as_slices`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.as_slices
[`SliceDeque`]: struct.SliceDeque.html

## Modules

## Module `macros`

**Attributes:**

- `#[macro_use]`

Macros and utilities.

```rust
pub(crate) mod macros { /* ... */ }
```

### Types

#### Struct `TinyAsciiString`

Small Ascii String. Used to write errors in `no_std` environments.

```rust
pub struct TinyAsciiString {
    pub(in ::macros) buf: [u8; 512],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `[u8; 512]` | A buffer for the ascii string |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new string initialized to zero.

- ```rust
  pub unsafe fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Converts the Tiny Ascii String to an UTF-8 string (unchecked).

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Write**
  - ```rust
    fn write_str(self: &mut Self, s: &str) -> Result<(), crate::fmt::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Macros

#### Macro `tiny_str`

```rust
pub(crate) macro_rules! tiny_str {
    /* macro_rules! tiny_str {
    ($($t:tt)*) => { ... };
} */
}
```

## Module `mirrored`

Mirrored memory buffer.

```rust
pub(crate) mod mirrored { /* ... */ }
```

### Modules

## Module `buffer`

Implements a mirrored memory buffer.

```rust
pub(in ::mirrored) mod buffer { /* ... */ }
```

### Types

#### Struct `Buffer`

Mirrored memory buffer of length `len`.

The buffer elements in range `[0, len/2)` are mirrored into the range
`[len/2, len)`.

```rust
pub struct Buffer<T> {
    pub(in ::mirrored::buffer) ptr: core::ptr::NonNull<T>,
    pub(in ::mirrored::buffer) len: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ptr` | `core::ptr::NonNull<T>` | Pointer to the first element in the buffer. |
| `len` | `usize` | Length of the buffer:<br><br>* it is NOT always a multiple of 2<br>* the elements in range `[0, len/2)` are mirrored into the range<br>`[len/2, len)`. |

##### Implementations

###### Methods

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Number of elements in the buffer.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Is the buffer empty?

- ```rust
  pub unsafe fn ptr(self: &Self) -> *mut T { /* ... */ }
  ```
  Pointer to the first element in the buffer.

- ```rust
  pub unsafe fn as_slice(self: &Self) -> &[T] { /* ... */ }
  ```
  Interprets contents as a slice.

- ```rust
  pub unsafe fn as_mut_slice(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Interprets contents as a mut slice.

- ```rust
  pub unsafe fn get(self: &Self, i: usize) -> &T { /* ... */ }
  ```
  Interprets content as a slice and access the `i`-th element.

- ```rust
  pub unsafe fn get_mut(self: &mut Self, i: usize) -> &mut T { /* ... */ }
  ```
  Interprets content as a mut slice and access the `i`-th element.

- ```rust
  pub(in ::mirrored::buffer) fn empty_len() -> usize { /* ... */ }
  ```

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new empty `Buffer`.

- ```rust
  pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self { /* ... */ }
  ```
  Creates a new empty `Buffer` from a `ptr` and a `len`.

- ```rust
  pub fn size_in_bytes(len: usize) -> usize { /* ... */ }
  ```
  Total number of bytes in the buffer.

- ```rust
  pub fn uninitialized(len: usize) -> Result<Self, AllocError> { /* ... */ }
  ```
  Create a mirrored buffer containing `len` `T`s where the first half of

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
### Functions

#### Function `no_required_allocation_units`

Number of required memory allocation units to hold `bytes`.

```rust
pub(in ::mirrored::buffer) fn no_required_allocation_units(bytes: usize) -> usize { /* ... */ }
```

## Module `linux`

**Attributes:**

- `#[cfg(all(any(target_os = "linux", target_os = "android"),
not(feature = "unix_sysv")))]`

Non-racy linux-specific mirrored memory allocation.

```rust
pub(in ::mirrored) mod linux { /* ... */ }
```

### Functions

#### Function `memfd_create`

[`memfd_create`] - create an anonymous file

[`memfd_create`]: http://man7.org/linux/man-pages/man2/memfd_create.2.html

```rust
pub(in ::mirrored::linux) fn memfd_create(name: *const libc::c_char, flags: libc::c_uint) -> libc::c_long { /* ... */ }
```

#### Function `allocation_granularity`

Returns the size of a memory allocation unit.

In Linux-like systems this equals the page-size.

```rust
pub fn allocation_granularity() -> usize { /* ... */ }
```

#### Function `errno`

Reads `errno`.

```rust
pub(in ::mirrored::linux) fn errno() -> libc::c_int { /* ... */ }
```

#### Function `allocate_mirrored`

Allocates an uninitialzied buffer that holds `size` bytes, where
the bytes in range `[0, size / 2)` are mirrored into the bytes in
range `[size / 2, size)`.

On Linux the algorithm is as follows:

* 1. Allocate a memory-mapped file containing `size / 2` bytes.
* 2. Map the file into `size` bytes of virtual memory.
* 3. Map the file into the last `size / 2` bytes of the virtual memory
region      obtained in step 2.

This algorithm doesn't have any races.

# Panics

If `size` is zero or `size / 2` is not a multiple of the
allocation granularity.

```rust
pub fn allocate_mirrored(size: usize) -> Result<*mut u8, super::AllocError> { /* ... */ }
```

#### Function `deallocate_mirrored`

Deallocates the mirrored memory region at `ptr` of `size` bytes.

# Unsafe

`ptr` must have been obtained from a call to `allocate_mirrored(size)`,
otherwise the behavior is undefined.

# Panics

If `size` is zero or `size / 2` is not a multiple of the
allocation granularity, or `ptr` is null.

```rust
pub unsafe fn deallocate_mirrored(ptr: *mut u8, size: usize) { /* ... */ }
```

#### Function `print_error`

**Attributes:**

- `#[cfg(all(debug_assertions, feature = "use_std"))]`

Prints last os error at `location`.

```rust
pub(in ::mirrored::linux) fn print_error(location: &str) { /* ... */ }
```

### Types

#### Enum `AllocError`

Allocation error.

```rust
pub enum AllocError {
    Oom,
    Other,
}
```

##### Variants

###### `Oom`

The system is Out-of-memory.

###### `Other`

Other allocation errors (not out-of-memory).

Race conditions, exhausted file descriptors, etc.

##### Implementations

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
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

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut crate::fmt::Formatter<''_>) -> crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
### Re-exports

#### Re-export `Buffer`

```rust
pub use self::buffer::Buffer;
```

## Module `intrinsics`

**Attributes:**

- `#[cfg(not(feature = "unstable"))]`

A stable version of the `core::intrinsics` module.

```rust
pub(crate) mod intrinsics { /* ... */ }
```

### Functions

#### Function `unlikely`

**Attributes:**

- `#[inline(always)]`

Like `core::intrinsics::unlikely` but does nothing.

```rust
pub unsafe fn unlikely<T>(x: T) -> T { /* ... */ }
```

#### Function `assume`

**Attributes:**

- `#[inline(always)]`

Like `core::intrinsics::assume` but does nothing.

```rust
pub unsafe fn assume<T>(x: T) -> T { /* ... */ }
```

#### Function `arith_offset`

**Attributes:**

- `#[inline(always)]`

Like `core::intrinsics::arith_offset` but doing pointer to integer
conversions.

```rust
pub unsafe fn arith_offset<T>(dst: *const T, offset: isize) -> *const T { /* ... */ }
```

## Types

### Struct `SliceDeque`

A double-ended queue that derefs into a slice.

It is implemented with a growable virtual ring buffer.

```rust
pub struct SliceDeque<T> {
    pub(crate) elems_: core::ptr::NonNull<[T]>,
    pub(crate) buf: Buffer<T>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `elems_` | `core::ptr::NonNull<[T]>` | Elements in the queue. |
| `buf` | `Buffer<T>` | Mirrored memory buffer. |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new empty deque.

- ```rust
  pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize, elems: &mut [T]) -> Self { /* ... */ }
  ```
  Creates a SliceDeque from its raw components.

- ```rust
  pub fn with_capacity(n: usize) -> Self { /* ... */ }
  ```
  Create an empty deque with capacity to hold `n` elements.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements that the deque can hold without

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Number of elements in the ring buffer.

- ```rust
  pub fn is_full(self: &Self) -> bool { /* ... */ }
  ```
  Is the ring buffer full ?

- ```rust
  pub fn as_slice(self: &Self) -> &[T] { /* ... */ }
  ```
  Extracts a slice containing the entire deque.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Extracts a mutable slice containing the entire deque.

- ```rust
  pub fn as_slices(self: &Self) -> (&[T], &[T]) { /* ... */ }
  ```
  Returns a pair of slices, where the first slice contains the contents

- ```rust
  pub fn as_mut_slices(self: &mut Self) -> (&mut [T], &mut [T]) { /* ... */ }
  ```
  Returns a pair of slices, where the first slice contains the contents

- ```rust
  pub unsafe fn tail_head_slice(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Returns the slice of uninitialized memory between the `tail` and the

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), AllocError> { /* ... */ }
  ```
  Attempts to reserve capacity for inserting at least `additional`

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserves capacity for inserting at least `additional` elements without

- ```rust
  pub(crate) fn reserve_capacity(self: &mut Self, new_capacity: usize) -> Result<(), AllocError> { /* ... */ }
  ```
  Attempts to reserve capacity for `new_capacity` elements. Does nothing

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserves the minimum capacity for exactly `additional` more elements to

- ```rust
  pub(crate) fn grow_policy(self: &Self, additional: usize) -> usize { /* ... */ }
  ```
  Growth policy of the deque. The capacity is going to be a multiple of

- ```rust
  pub unsafe fn move_head_unchecked(self: &mut Self, x: isize) { /* ... */ }
  ```
  Moves the deque head by `x`.

- ```rust
  pub unsafe fn move_head(self: &mut Self, x: isize) { /* ... */ }
  ```
  Moves the deque head by `x`.

- ```rust
  pub unsafe fn move_tail_unchecked(self: &mut Self, x: isize) { /* ... */ }
  ```
  Moves the deque tail by `x`.

- ```rust
  pub unsafe fn move_tail(self: &mut Self, x: isize) { /* ... */ }
  ```
  Moves the deque tail by `x`.

- ```rust
  pub(crate) unsafe fn append_elements(self: &mut Self, other: *const [T]) { /* ... */ }
  ```
  Appends elements to `self` from `other`.

- ```rust
  pub unsafe fn steal_from_slice(s: &[T]) -> Self { /* ... */ }
  ```
  Steal the elements from the slice `s`. You should `mem::forget` the

- ```rust
  pub fn append(self: &mut Self, other: &mut Self) { /* ... */ }
  ```
  Moves all the elements of `other` into `Self`, leaving `other` empty.

- ```rust
  pub fn front(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Provides a reference to the first element, or `None` if the deque is

- ```rust
  pub fn front_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Provides a mutable reference to the first element, or `None` if the

- ```rust
  pub fn back(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Provides a reference to the last element, or `None` if the deque is

- ```rust
  pub fn back_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Provides a mutable reference to the last element, or `None` if the

- ```rust
  pub fn try_push_front(self: &mut Self, value: T) -> Result<(), (T, AllocError)> { /* ... */ }
  ```
  Attempts to prepend `value` to the deque.

- ```rust
  pub fn push_front(self: &mut Self, value: T) { /* ... */ }
  ```
  Prepends `value` to the deque.

- ```rust
  pub fn try_push_back(self: &mut Self, value: T) -> Result<(), (T, AllocError)> { /* ... */ }
  ```
  Attempts to appends `value` to the deque.

- ```rust
  pub fn push_back(self: &mut Self, value: T) { /* ... */ }
  ```
  Appends `value` to the deque.

- ```rust
  pub fn pop_front(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes the first element and returns it, or `None` if the deque is

- ```rust
  pub fn pop_back(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes the last element from the deque and returns it, or `None` if it

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrinks the capacity of the deque as much as possible.

- ```rust
  pub fn truncate_back(self: &mut Self, len: usize) { /* ... */ }
  ```
  Shortens the deque by removing excess elements from the back.

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```
  Shortens the deque by removing excess elements from the back.

- ```rust
  pub fn truncate_front(self: &mut Self, len: usize) { /* ... */ }
  ```
  Shortens the deque by removing excess elements from the front.

- ```rust
  pub fn drain<R>(self: &mut Self, range: R) -> Drain<''_, T>
where
    R: ops::RangeBounds<usize> { /* ... */ }
  ```
  Creates a draining iterator that removes the specified range in the

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Removes all values from the deque.

- ```rust
  pub fn swap_remove_back(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Removes the element at `index` and return it in `O(1)` by swapping the

- ```rust
  pub fn swap_remove_front(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Removes the element at `index` and returns it in `O(1)` by swapping the

- ```rust
  pub fn insert(self: &mut Self, index: usize, element: T) { /* ... */ }
  ```
  Inserts an `element` at `index` within the deque, shifting all elements

- ```rust
  pub fn remove(self: &mut Self, index: usize) -> T { /* ... */ }
  ```
  Removes and returns the element at position `index` within the deque.

- ```rust
  pub fn split_off(self: &mut Self, at: usize) -> Self { /* ... */ }
  ```
  Splits the collection into two at the given index.

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(&T) -> bool { /* ... */ }
  ```
  Retains only the elements specified by the predicate.

- ```rust
  pub fn dedup_by_key<F, K>(self: &mut Self, key: F)
where
    F: FnMut(&mut T) -> K,
    K: PartialEq { /* ... */ }
  ```
  Removes all but the first of consecutive elements in the deque that

- ```rust
  pub fn dedup_by<F>(self: &mut Self, same_bucket: F)
where
    F: FnMut(&mut T, &mut T) -> bool { /* ... */ }
  ```
  Removes all but the first of consecutive elements in the deque

- ```rust
  pub(crate) fn extend_with<E: ExtendWith<T>>(self: &mut Self, n: usize, value: E) { /* ... */ }
  ```
  Extend the `SliceDeque` by `n` values, using the given generator.

- ```rust
  pub(crate) fn extend_desugared<I: Iterator<Item = T>>(self: &mut Self, iterator: I) { /* ... */ }
  ```
  Extend for a general iterator.

- ```rust
  pub fn splice<R, I>(self: &mut Self, range: R, replace_with: I) -> Splice<''_, <I as >::IntoIter>
where
    R: ops::RangeBounds<usize>,
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Creates a splicing iterator that replaces the specified range in the

- ```rust
  pub fn drain_filter<F>(self: &mut Self, filter: F) -> DrainFilter<''_, T, F>
where
    F: FnMut(&mut T) -> bool { /* ... */ }
  ```
  Creates an iterator which uses a closure to determine if an element

- ```rust
  pub fn extend_from_slice(self: &mut Self, other: &[T]) { /* ... */ }
  ```
  Clones and appends all elements in a slice to the `SliceDeque`.

- ```rust
  pub fn resize(self: &mut Self, new_len: usize, value: T) { /* ... */ }
  ```
  Modifies the `SliceDeque` in-place so that `len()` is equal to

- ```rust
  pub fn resize_default(self: &mut Self, new_len: usize) { /* ... */ }
  ```
  Resizes the `SliceDeque` in-place so that `len` is equal to `new_len`.

- ```rust
  pub fn dedup(self: &mut Self) { /* ... */ }
  ```
  Removes consecutive repeated elements in the deque.

- ```rust
  pub fn remove_item(self: &mut Self, item: &T) -> Option<T> { /* ... */ }
  ```
  Removes the first instance of `item` from the deque if the item exists.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```
    Creates a consuming iterator, that is, one that moves each value out of

  - ```rust
    fn into_iter(self: Self) -> slice::Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> slice::IterMut<''a, T> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[T] { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **SpecExtend**
  - ```rust
    fn from_iter(iterator: I) -> Self { /* ... */ }
    ```

  - ```rust
    fn spec_extend(self: &mut Self, iter: I) { /* ... */ }
    ```

  - ```rust
    fn from_iter(iterator: I) -> Self { /* ... */ }
    ```

  - ```rust
    fn spec_extend(self: &mut Self, iterator: I) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iter: I) { /* ... */ }
    ```

  - ```rust
    fn extend<I: IntoIterator<Item = &''a T>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SliceDeque<B>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b mut [B]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Vec<B>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 0]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 0]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 1]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 1]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 2]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 2]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 3]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 3]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 4]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 4]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 5]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 5]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 6]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 6]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 7]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 7]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 9]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 9]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 10]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 10]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 11]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 11]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 12]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 12]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 13]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 13]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 14]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 14]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 15]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 15]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 16]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 16]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 17]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 17]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 18]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 18]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 19]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 19]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 20]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 20]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 21]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 21]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 22]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 22]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 23]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 23]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 24]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 24]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 25]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 25]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 26]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 26]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 27]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 27]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 28]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 28]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 29]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 29]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 30]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 30]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 31]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 31]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[B; 32]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b [B; 32]) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a [T]) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(s: &''a [T]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: &''a mut [T]) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, other: &Self) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Unpin**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

- **Receiver**
### Struct `Drain`

A draining iterator for `SliceDeque<T>`.

This `struct` is created by the [`drain`] method on [`SliceDeque`].

[`drain`]: struct.SliceDeque.html#method.drain
[`SliceDeque`]: struct.SliceDeque.html

```rust
pub struct Drain<''a, T: ''a> {
    pub(crate) tail_start: usize,
    pub(crate) tail_len: usize,
    pub(crate) iter: slice::Iter<''a, T>,
    pub(crate) deq: core::ptr::NonNull<SliceDeque<T>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tail_start` | `usize` | Index of tail to preserve |
| `tail_len` | `usize` | Length of tail |
| `iter` | `slice::Iter<''a, T>` | Current remaining range to remove |
| `deq` | `core::ptr::NonNull<SliceDeque<T>>` | A shared mutable pointer to the deque (with shared ownership). |

#### Implementations

##### Methods

- ```rust
  pub(crate) unsafe fn fill<I: Iterator<Item = T>>(self: &mut Self, replace_with: &mut I) -> bool { /* ... */ }
  ```
  The range from `self.deq.tail` to `self.tail()_start` contains elements

- ```rust
  pub(crate) unsafe fn move_tail_unchecked(self: &mut Self, extra_capacity: usize) { /* ... */ }
  ```
  Make room for inserting more elements before the tail.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

### Struct `IntoIter`

An iterator that moves out of a deque.

This `struct` is created by the `into_iter` method on
[`SliceDeque`][`SliceDeque`] (provided by the [`IntoIterator`] trait).

[`SliceDeque`]: struct.SliceDeque.html
[`IntoIterator`]: ../../std/iter/trait.IntoIterator.html

```rust
pub struct IntoIter<T> {
    pub(crate) buf: core::ptr::NonNull<T>,
    pub(crate) cap: usize,
    pub(crate) ptr: *const T,
    pub(crate) end: *const T,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `core::ptr::NonNull<T>` | NonNull pointer to the buffer |
| `cap` | `usize` | Capacity of the buffer. |
| `ptr` | `*const T` | Pointer to the first element. |
| `end` | `*const T` | Pointer to one-past-the-end. |

#### Implementations

##### Methods

- ```rust
  pub fn as_slice(self: &Self) -> &[T] { /* ... */ }
  ```
  Returns the remaining items of this iterator as a slice.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Returns the remaining items of this iterator as a mutable slice.

##### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

### Struct `ExtendElement`

TODO: docs

```rust
pub(crate) struct ExtendElement<T>(pub(crate) T);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExtendWith**
  - ```rust
    fn next(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `ExtendDefault`

TODO: docs

```rust
pub(crate) struct ExtendDefault;
```

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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
- **ExtendWith**
  - ```rust
    fn next(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
### Struct `Splice`

A splicing iterator for `SliceDeque`.

This struct is created by the [`splice()`] method on [`SliceDeque`]. See
its documentation for more.

[`splice()`]: struct.SliceDeque.html#method.splice
[`SliceDeque`]: struct.SliceDeque.html

```rust
pub struct Splice<''a, I: Iterator + ''a> {
    pub(crate) drain: Drain<''a, <I as >::Item>,
    pub(crate) replace_with: I,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `drain` | `Drain<''a, <I as >::Item>` | TODO: docs |
| `replace_with` | `I` | TODO: docs |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Sync**
### Struct `DrainFilter`

An iterator produced by calling `drain_filter` on `SliceDeque`.

```rust
pub struct DrainFilter<''a, T: ''a, F> {
    pub(crate) deq: &''a mut SliceDeque<T>,
    pub(crate) idx: usize,
    pub(crate) del: usize,
    pub(crate) old_len: usize,
    pub(crate) pred: F,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `deq` | `&''a mut SliceDeque<T>` | TODO: docs |
| `idx` | `usize` | TODO: docs |
| `del` | `usize` | TODO: docs |
| `old_len` | `usize` | TODO: docs |
| `pred` | `F` | TODO: docs |

#### Implementations

##### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
## Traits

### Trait `WrappingOffsetFrom`

Stable implementation of `.wrapping_offset_from` for pointers.

```rust
pub(crate) trait WrappingOffsetFrom {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `wrapping_offset_from_`: Stable implementation of `.wrapping_offset_from` for pointers.

#### Implementations

This trait is implemented for the following types:

- `*const T` with <T: Sized>

### Trait `SpecExtend`

Specialization trait used for `SliceDeque::from_iter` and
`SliceDeque::extend`.

```rust
pub(crate) trait SpecExtend<T, I> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `from_iter`: Specialization for `SliceDeque::from_iter`.
- `spec_extend`: Specialization for `SliceDeque::extend`.

#### Implementations

This trait is implemented for the following types:

- `SliceDeque<T>` with <T, I>
- `SliceDeque<T>` with <''a, T, I>

### Trait `ExtendWith`

This code generalises `extend_with_{element,default}`.

```rust
pub(crate) trait ExtendWith<T> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `next`: TODO: docs
- `last`: TODO: docs

#### Implementations

This trait is implemented for the following types:

- `ExtendElement<T>` with <T: Clone>
- `ExtendDefault` with <T: Default>

### Trait `SpecFromElem`

Specialization trait used for `SliceDeque::from_elem`.

```rust
pub(crate) trait SpecFromElem: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `from_elem`: TODO: docs

#### Implementations

This trait is implemented for the following types:

- `T` with <T: Clone>

## Functions

### Function `in_bounds`

Is `p` in bounds of `s` ?

Does it point to an element of `s` ? That is, one past the end of `s` is
not in bounds.

```rust
pub(crate) fn in_bounds<T>(s: &[T], p: *mut T) -> bool { /* ... */ }
```

### Function `nonnull_raw_slice`

```rust
pub(crate) unsafe fn nonnull_raw_slice<T>(ptr: *mut T, len: usize) -> core::ptr::NonNull<[T]> { /* ... */ }
```

### Function `from_iter_default`

**Attributes:**

- `#[inline(always)]`

Default implementation of `SpecExtend::from_iter`.

```rust
pub(crate) fn from_iter_default<T, I: Iterator<Item = T>>(iterator: I) -> SliceDeque<T> { /* ... */ }
```

## Macros

### Macro `__impl_slice_eq1`

```rust
pub(crate) macro_rules! __impl_slice_eq1 {
    /* macro_rules! __impl_slice_eq1 {
    ($Lhs:ty, $Rhs:ty) => { ... };
    ($Lhs:ty, $Rhs:ty, $Bound:ident) => { ... };
} */
}
```

### Macro `array_impls`

```rust
pub(crate) macro_rules! array_impls {
    /* macro_rules! array_impls {
    ($($N: expr)+) => { ... };
} */
}
```

### Macro `impl_spec_from_elem`

```rust
pub(crate) macro_rules! impl_spec_from_elem {
    /* macro_rules! impl_spec_from_elem {
    ($t:ty, $is_zero:expr) => { ... };
} */
}
```

### Macro `sdeq`

**Attributes:**

- `#[macro_export]`

Creates a [`SliceDeque`] containing the arguments.

`sdeq!` allows `SliceDeque`s to be defined with the same syntax as array
expressions. There are two forms of this macro:

- Create a [`SliceDeque`] containing a given list of elements:

```
# #[macro_use] extern crate slice_deque;
# use slice_deque::SliceDeque;
# fn main() {
let v: SliceDeque<i32> = sdeq![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
# }
```

- Create a [`SliceDeque`] from a given element and size:

```
# #[macro_use] extern crate slice_deque;
# use slice_deque::SliceDeque;
# fn main() {
let v = sdeq![7; 3];
assert_eq!(v, [7, 7, 7]);
# }
```

Note that unlike array expressions this syntax supports all elements
which implement `Clone` and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `sdeq![Rc::new(1); 5]` will create a deque of five references
to the same boxed integer value, not five references pointing to
independently boxed integers.

```
# #[macro_use] extern crate slice_deque;
# use slice_deque::SliceDeque;
# use std::rc::Rc;
# fn main() {
let v = sdeq![Rc::new(1_i32); 5];
let ptr: *const i32 = &*v[0] as *const i32;
for i in v.iter() {
    assert_eq!(Rc::into_raw(i.clone()), ptr);
}
# }
```

[`SliceDeque`]: struct.SliceDeque.html

```rust
pub macro_rules! sdeq {
    /* macro_rules! sdeq {
    ($elem:expr; $n:expr) => { ... };
    () => { ... };
    ($($x:expr),*) => { ... };
    ($($x:expr,)*) => { ... };
} */
}
```

## Re-exports

### Re-export `AllocError`

```rust
pub use mirrored::AllocError;
```

### Re-export `Buffer`

```rust
pub use mirrored::Buffer;
```

