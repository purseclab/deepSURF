//! Vector-like class allocated entirely on the stack.
//!
//! Shallow wrapper around an underlying `Array`, which panics if the
//! array bounds are exceeded.
//!
//! # no_std support
//!
//! By default, `smallvec` depends on `libstd`. However, it can be configured to use the unstable
//! `liballoc` API instead, for use on platforms that have `liballoc` but not `libstd`.  This
//! configuration is currently unstable and is not guaranteed to work on all versions of Rust.
//!
//! To depend on `smallvec` without `libstd`, use `default-features = false` in the `smallvec`
//! section of Cargo.toml to disable its `"std"` feature.
//!
//! Adapted from Servo's smallvec:
//!     https://github.com/servo/rust-smallve
//!
//! StackVec is distributed under the same terms as the smallvec and
//! lexical, that is, it is dual licensed under either the MIT or Apache
//! 2.0 license.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

extern crate unreachable;
use unreachable::UncheckedOptionExt;

/// Facade around the core features for name mangling.
pub(crate) mod lib {
    #[cfg(feature = "std")]
    pub(crate) use std::*;

    #[cfg(not(feature = "std"))]
    pub(crate) use core::*;
}

use lib::borrow::{Borrow, BorrowMut};
use lib::{cmp, fmt, hash, iter, mem, ops, ptr, slice};

#[cfg(feature = "std")]
use lib::io;

// POINTER METHODS

// Certain pointer methods aren't implemented below Rustc versions 1.26.
// We implement a dummy version here.

trait PointerMethods {
    // Add to the pointer (use padd to avoid conflict with ptr::add).
    unsafe fn padd(self, count: usize) -> Self;
}

impl<T> PointerMethods for *const T {
    #[inline(always)]
    unsafe fn padd(self, count: usize) -> Self {
        #[cfg(has_pointer_methods)]
        return self.add(count);

        #[cfg(not(has_pointer_methods))]
        return self.offset(count as isize);
    }
}

impl<T> PointerMethods for *mut T {
    #[inline(always)]
    unsafe fn padd(self, count: usize) -> Self {
        #[cfg(has_pointer_methods)]
        return self.add(count);

        #[cfg(not(has_pointer_methods))]
        return self.offset(count as isize);
    }
}

// ARRAY

/// Types that can be used as the backing store for a StackVec
pub unsafe trait Array {
    /// The type of the array's elements.
    type Item;
    /// Returns the number of items the array can hold.
    fn size() -> usize;
    /// Returns a pointer to the first element of the array.
    fn ptr(&self) -> *const Self::Item;
    /// Returns a mutable pointer to the first element of the array.
    fn ptr_mut(&mut self) -> *mut Self::Item;
}

macro_rules! impl_array(
    ($($size:expr),+) => {
        $(
            unsafe impl<T> Array for [T; $size] {
                type Item = T;
                fn size() -> usize { $size }
                fn ptr(&self) -> *const T { self.as_ptr() }
                fn ptr_mut(&mut self) -> *mut T { self.as_mut_ptr() }
            }
        )+
    }
);

impl_array! { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 20, 24, 32, 36, 0x40, 0x80, 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000, 0x20000, 0x40000, 0x80000, 0x100000 }

// VEC LIKE

/// Common operations implemented by both `Vec` and `StackVec`.
///
/// This can be used to write generic code that works with both `Vec` and `StackVec`.
///
/// ## Example
///
/// ```rust
/// use stackvector::{VecLike, StackVec};
///
/// fn initialize<V: VecLike<u8>>(v: &mut V) {
///     for i in 0..5 {
///         v.push(i);
///     }
/// }
///
/// let mut vec = Vec::new();
/// initialize(&mut vec);
///
/// let mut stack_vec = StackVec::<[u8; 8]>::new();
/// initialize(&mut stack_vec);
/// ```
#[deprecated(note = "Use `Extend` and `Deref<[T]>` instead")]
pub trait VecLike<T>:
        ops::Index<usize, Output=T> +
        ops::IndexMut<usize> +
        ops::Index<ops::Range<usize>, Output=[T]> +
        ops::IndexMut<ops::Range<usize>> +
        ops::Index<ops::RangeFrom<usize>, Output=[T]> +
        ops::IndexMut<ops::RangeFrom<usize>> +
        ops::Index<ops::RangeTo<usize>, Output=[T]> +
        ops::IndexMut<ops::RangeTo<usize>> +
        ops::Index<ops::RangeFull, Output=[T]> +
        ops::IndexMut<ops::RangeFull> +
        ops::DerefMut<Target = [T]> +
        Extend<T> {

    /// Append an element to the vector.
    fn push(&mut self, value: T);

    /// Pop an element from the end of the vector.
    fn pop(&mut self) -> Option<T>;
}

#[allow(deprecated)]
impl<T> VecLike<T> for Vec<T> {
    #[inline]
    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }

    #[inline]
    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }
}

// EXTEND FROM SLICE

/// Trait to be implemented by a collection that can be extended from a slice
///
/// ## Example
///
/// ```rust
/// use stackvector::{ExtendFromSlice, StackVec};
///
/// fn initialize<V: ExtendFromSlice<u8>>(v: &mut V) {
///     v.extend_from_slice(b"Test!");
/// }
///
/// let mut vec = Vec::new();
/// initialize(&mut vec);
/// assert_eq!(&vec, b"Test!");
///
/// let mut stack_vec = StackVec::<[u8; 8]>::new();
/// initialize(&mut stack_vec);
/// assert_eq!(&stack_vec as &[_], b"Test!");
/// ```
pub trait ExtendFromSlice<T> {
    /// Extends a collection from a slice of its element type
    fn extend_from_slice(&mut self, other: &[T]);
}

impl<T: Clone> ExtendFromSlice<T> for Vec<T> {
    fn extend_from_slice(&mut self, other: &[T]) {
        Vec::extend_from_slice(self, other)
    }
}

// DRAIN

/// An iterator that removes the items from a `StackVec` and yields them by value.
///
/// Returned from [`StackVec::drain`][1].
///
/// [1]: struct.StackVec.html#method.drain
pub struct Drain<'a, T: 'a> {
    iter: slice::IterMut<'a, T>,
}

impl<'a, T: 'a> Iterator for Drain<'a,T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next().map(|reference| unsafe { ptr::read(reference) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: 'a> DoubleEndedIterator for Drain<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|reference| unsafe { ptr::read(reference) })
    }
}

impl<'a, T> ExactSizeIterator for Drain<'a, T> { }

impl<'a, T: 'a> Drop for Drain<'a,T> {
    fn drop(&mut self) {
        // Destroy the remaining elements.
        for _ in self.by_ref() {}
    }
}

// SET LEN ON DROP

/// Set the length of the vec when the `SetLenOnDrop` value goes out of scope.
///
/// Copied from https://github.com/rust-lang/rust/pull/36355
struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}

impl<'a> SetLenOnDrop<'a> {
    #[inline]
    fn new(len: &'a mut usize) -> Self {
        SetLenOnDrop { local_len: *len, len: len }
    }

    #[inline]
    fn increment_len(&mut self, increment: usize) {
        self.local_len += increment;
    }
}

impl<'a> Drop for SetLenOnDrop<'a> {
    #[inline]
    fn drop(&mut self) {
        *self.len = self.local_len;
    }
}

// STACKVEC

/// A `Vec`-like container that stores elements on the stack.
///
/// The amount of data that a `StackVec` can store inline depends on its backing store. The backing
/// store can be any type that implements the `Array` trait; usually it is a small fixed-sized
/// array.  For example a `StackVec<[u64; 8]>` can hold up to eight 64-bit integers inline.
///
/// ## Example
///
/// ```rust,should_panic
/// use stackvector::StackVec;
/// let mut v = StackVec::<[u8; 4]>::new(); // initialize an empty vector
///
/// // The vector can hold up to 4 items without spilling onto the heap.
/// v.extend(0..4);
/// assert_eq!(v.len(), 4);
///
/// // Pushing another element will force the buffer to spill and panic:
/// v.push(4);
/// ```
pub struct StackVec<A: Array> {
    // The capacity field is used for iteration and other optimizations.
    // Publicly expose the fields, so they may be used in constant
    // initialization.
    pub length: usize,
    pub data: mem::ManuallyDrop<A>,
}

impl<A: Array> StackVec<A> {
    /// Construct an empty vector
    #[inline]
    pub fn new() -> StackVec<A> {
        unsafe {
            StackVec {
                length: 0,
                data: mem::uninitialized(),
            }
        }
    }

    /// Construct a new `StackVec` from a `Vec<A::Item>`.
    ///
    /// Elements will be copied to the inline buffer if vec.len() <= A::size().
    ///
    /// ```rust
    /// use stackvector::StackVec;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let stack_vec: StackVec<[_; 5]> = StackVec::from_vec(vec);
    ///
    /// assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    pub fn from_vec(vec: Vec<A::Item>) -> StackVec<A> {
        assert!(vec.len() <= A::size());
        unsafe { Self::from_vec_unchecked(vec) }
    }

    /// Construct a new `StackVec` from a `Vec<A::Item>` without bounds checking.
    pub unsafe fn from_vec_unchecked(mut vec: Vec<A::Item>) -> StackVec<A> {
        let mut data: A = mem::uninitialized();
        let len = vec.len();
        vec.set_len(0);
        ptr::copy_nonoverlapping(vec.as_ptr(), data.ptr_mut(), len);

        StackVec {
            length: len,
            data: mem::ManuallyDrop::new(data),
        }
    }

    /// Constructs a new `StackVec` on the stack from an `A` without
    /// copying elements.
    ///
    /// ```rust
    /// use stackvector::StackVec;
    ///
    /// let buf = [1, 2, 3, 4, 5];
    /// let stack_vec: StackVec<_> = StackVec::from_buf(buf);
    ///
    /// assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    pub fn from_buf(buf: A) -> StackVec<A> {
        StackVec {
            length: A::size(),
            data: mem::ManuallyDrop::new(buf),
        }
    }

    /// Constructs a new `StackVec` on the stack from an `A` without
    /// copying elements. Also sets the length, which must be less or
    /// equal to the size of `buf`.
    ///
    /// ```rust
    /// use stackvector::StackVec;
    ///
    /// let buf = [1, 2, 3, 4, 5, 0, 0, 0];
    /// let stack_vec: StackVec<_> = StackVec::from_buf_and_len(buf, 5);
    ///
    /// assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    pub fn from_buf_and_len(buf: A, len: usize) -> StackVec<A> {
        assert!(len <= A::size());
        unsafe { StackVec::from_buf_and_len_unchecked(buf, len) }
    }

    /// Constructs a new `StackVec` on the stack from an `A` without
    /// copying elements. Also sets the length. The user is responsible
    /// for ensuring that `len <= A::size()`.
    ///
    /// ```rust
    /// use stackvector::StackVec;
    ///
    /// let buf = [1, 2, 3, 4, 5, 0, 0, 0];
    /// let stack_vec: StackVec<_> = unsafe {
    ///     StackVec::from_buf_and_len_unchecked(buf, 5)
    /// };
    ///
    /// assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    pub unsafe fn from_buf_and_len_unchecked(buf: A, len: usize) -> StackVec<A> {
        StackVec {
            length: len,
            data: mem::ManuallyDrop::new(buf),
        }
    }

    /// Sets the length of a vector.
    ///
    /// This will explicitly set the size of the vector, without actually
    /// modifying its buffers, so it is up to the caller to ensure that the
    /// vector is actually the specified size.
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.length = new_len;
    }

    /// The number of elements stored in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// If the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The number of items the vector can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        A::size()
    }

    /// Empty the vector and return an iterator over its former contents.
    pub fn drain(&mut self) -> Drain<A::Item> {
        unsafe {
            let slice = slice::from_raw_parts_mut(self.as_mut_ptr(), self.len());
            self.set_len(0);

            Drain {
                iter: slice.iter_mut(),
            }
        }
    }

    /// Append an item to the vector.
    #[inline]
    pub fn push(&mut self, value: A::Item) {
        assert!(self.len() < self.capacity());
        unsafe {
            ptr::write(self.as_mut_ptr().padd(self.length), value);
            self.length += 1;
        }
    }

    /// Remove an item from the end of the vector and return it, or None if empty.
    #[inline]
    pub fn pop(&mut self) -> Option<A::Item> {
        unsafe {
            if self.len() == 0 {
                None
            } else {
                self.length -=1;
                Some(ptr::read(self.as_mut_ptr().padd(self.length)))
            }
        }
    }

    /// Shorten the vector, keeping the first `len` elements and dropping the rest.
    ///
    /// If `len` is greater than or equal to the vector's current length, this has no
    /// effect.
    /// `shrink_to_fit` after truncating.
    pub fn truncate(&mut self, len: usize) {
        unsafe {
            while len < self.length {
                self.length -= 1;
                ptr::drop_in_place(self.as_mut_ptr().padd(self.length));
            }
        }
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    #[inline]
    pub fn as_slice(&self) -> &[A::Item] {
        self
    }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [A::Item] {
        self
    }

    /// Remove the element at position `index`, replacing it with the last element.
    ///
    /// This does not preserve ordering, but is O(1).
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> A::Item {
        let len = self.len();
        self.swap(len - 1, index);
        unsafe { self.pop().unchecked_unwrap() }
    }

    /// Remove all elements from the vector.
    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Remove and return the element at position `index`, shifting all elements after it to the
    /// left.
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> A::Item {
        assert!(index < self.len());
        unsafe {
            self.length -= 1;
            let ptr = self.as_mut_ptr().padd(index);
            let item = ptr::read(ptr);
            ptr::copy(ptr.offset(1), ptr, self.length - index);
            item
        }
    }

    /// Insert an element at position `index`, shifting all elements after it to the right.
    ///
    /// Panics if `index` is out of bounds.
    pub fn insert(&mut self, index: usize, element: A::Item) {
        assert!(index < self.len() && self.len() < self.capacity());
        unsafe {
            let ptr = self.as_mut_ptr().padd(index);
            ptr::copy(ptr, ptr.offset(1), self.length - index);
            ptr::write(ptr, element);
            self.length += 1;
        }
    }

    /// Insert multiple elements at position `index`, shifting all following elements toward the
    /// back.
    pub fn insert_many<I: iter::IntoIterator<Item=A::Item>>(&mut self, index: usize, iterable: I) {
        let iter = iterable.into_iter();
        if index == self.len() {
            return self.extend(iter);
        }

        let (lower_size_bound, _) = iter.size_hint();
        assert!(lower_size_bound <= std::isize::MAX as usize);  // Ensure offset is indexable
        assert!(index + lower_size_bound >= index);             // Protect against overflow
        assert!(self.len() + lower_size_bound <= self.capacity());

        unsafe {
            let old_len = self.len();
            assert!(index <= old_len);
            let mut ptr = self.as_mut_ptr().padd(index);

            // Move the trailing elements.
            ptr::copy(ptr, ptr.padd(lower_size_bound), old_len - index);

            // In case the iterator panics, don't double-drop the items we just copied above.
            self.set_len(index);

            let mut num_added = 0;
            for element in iter {
                let mut cur = ptr.padd(num_added);
                if num_added >= lower_size_bound {
                    // Iterator provided more elements than the hint.  Move trailing items again.
                    assert!(self.len() + 1 <= self.capacity());
                    ptr = self.as_mut_ptr().padd(index);
                    cur = ptr.padd(num_added);
                    ptr::copy(cur, cur.padd(1), old_len - index);
                }
                ptr::write(cur, element);
                num_added += 1;
            }
            if num_added < lower_size_bound {
                // Iterator provided fewer elements than the hint
                ptr::copy(ptr.padd(lower_size_bound), ptr.padd(num_added), old_len - index);
            }

            self.set_len(old_len + num_added);
        }
    }

    /// Convert a StackVec to a Vec.
    pub fn into_vec(self) -> Vec<A::Item> {
        self.into_iter().collect()
    }

    /// Convert the StackVec into an `A`.
    pub fn into_inner(self) -> Result<A, Self> {
        if self.len() != A::size() {
            Err(self)
        } else {
            unsafe {
                let data = ptr::read(&self.data);
                mem::forget(self);
                Ok(mem::ManuallyDrop::into_inner(data))
            }
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    /// This method operates in place and preserves the order of the retained
    /// elements.
    pub fn retain<F: FnMut(&mut A::Item) -> bool>(&mut self, mut f: F) {
        let mut del = 0;
        let len = self.len();
        for i in 0..len {
            if !f(&mut self[i]) {
                del += 1;
            } else if del > 0 {
                self.swap(i - del, i);
            }
        }
        self.truncate(len - del);
    }

    /// Removes consecutive duplicate elements.
    pub fn dedup(&mut self) where A::Item: PartialEq<A::Item> {
        self.dedup_by(|a, b| a == b);
    }

    /// Removes consecutive duplicate elements using the given equality relation.
    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
        where F: FnMut(&mut A::Item, &mut A::Item) -> bool
    {
        // See the implementation of Vec::dedup_by in the
        // standard library for an explanation of this algorithm.
        let len = self.len();
        if len <= 1 {
            return;
        }

        let ptr = self.as_mut_ptr();
        let mut w: usize = 1;

        unsafe {
            for r in 1..len {
                let p_r = ptr.offset(r as isize);
                let p_wm1 = ptr.offset((w - 1) as isize);
                if !same_bucket(&mut *p_r, &mut *p_wm1) {
                    if r != w {
                        let p_w = p_wm1.offset(1);
                        mem::swap(&mut *p_r, &mut *p_w);
                    }
                    w += 1;
                }
            }
        }

        self.truncate(w);
    }

    /// Removes consecutive elements that map to the same key.
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
        where F: FnMut(&mut A::Item) -> K,
              K: PartialEq<K>
    {
        self.dedup_by(|a, b| key(a) == key(b));
    }
}

impl<A: Array> StackVec<A> where A::Item: Copy {
    /// Copy the elements from a slice into a new `StackVec`.
    ///
    /// For slices of `Copy` types, this is more efficient than `StackVec::from(slice)`.
    pub fn from_slice(slice: &[A::Item]) -> Self {
        assert!(slice.len() <= A::size());
        StackVec {
            length: slice.len(),
            data: unsafe {
                let mut data: A = mem::uninitialized();
                ptr::copy_nonoverlapping(slice.as_ptr(), data.ptr_mut(), slice.len());
                mem::ManuallyDrop::new(data)
            }
        }
    }

    /// Copy elements from a slice into the vector at position `index`, shifting any following
    /// elements toward the back.
    ///
    /// For slices of `Copy` types, this is more efficient than `insert`.
    pub fn insert_from_slice(&mut self, index: usize, slice: &[A::Item]) {
        assert!(index <= self.len() && self.len() + slice.len() <= self.capacity());
        unsafe {
            let len = self.len();
            let slice_ptr = slice.as_ptr();
            let ptr = self.as_mut_ptr().padd(index);
            ptr::copy(ptr, ptr.padd(slice.len()), len - index);
            ptr::copy_nonoverlapping(slice_ptr, ptr, slice.len());
            self.set_len(len + slice.len());
        }
    }

    /// Copy elements from a slice and append them to the vector.
    ///
    /// For slices of `Copy` types, this is more efficient than `extend`.
    #[inline]
    pub fn extend_from_slice(&mut self, slice: &[A::Item]) {
        let len = self.len();
        self.insert_from_slice(len, slice);
    }
}

impl<A: Array> StackVec<A> where A::Item: Clone {
    /// Resizes the vector so that its length is equal to `len`.
    ///
    /// If `len` is less than the current length, the vector simply truncated.
    ///
    /// If `len` is greater than the current length, `value` is appended to the
    /// vector until its length equals `len`.
    pub fn resize(&mut self, len: usize, value: A::Item) {
        assert!(len <= self.capacity());
        let old_len = self.len();
        if len > old_len {
            self.extend(iter::repeat(value).take(len - old_len));
        } else {
            self.truncate(len);
        }
    }

    /// Creates a `StackVec` with `n` copies of `elem`.
    /// ```
    /// use stackvector::StackVec;
    ///
    /// let v = StackVec::<[char; 128]>::from_elem('d', 2);
    /// assert_eq!(v, StackVec::from_buf(['d', 'd']));
    /// ```
    pub fn from_elem(elem: A::Item, n: usize) -> Self {
        assert!(n <= A::size());
        let mut v = StackVec::<A>::new();
        unsafe {
            let ptr = v.as_mut_ptr();
            let mut local_len = SetLenOnDrop::new(&mut v.length);
            for i in 0..n as isize {
                ptr::write(ptr.offset(i), elem.clone());
                local_len.increment_len(1);
            }
        }
        v
    }
}

impl<A: Array> ops::Deref for StackVec<A> {
    type Target = [A::Item];
    #[inline]
    fn deref(&self) -> &[A::Item] {
        unsafe {
            slice::from_raw_parts(self.data.ptr(), self.len())
        }
    }
}

impl<A: Array> ops::DerefMut for StackVec<A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [A::Item] {
        unsafe {
            slice::from_raw_parts_mut(self.data.ptr_mut(), self.len())
        }
    }
}

impl<A: Array> AsRef<[A::Item]> for StackVec<A> {
    #[inline]
    fn as_ref(&self) -> &[A::Item] {
        self
    }
}

impl<A: Array> AsMut<[A::Item]> for StackVec<A> {
    #[inline]
    fn as_mut(&mut self) -> &mut [A::Item] {
        self
    }
}

impl<A: Array> Borrow<[A::Item]> for StackVec<A> {
    #[inline]
    fn borrow(&self) -> &[A::Item] {
        self
    }
}

impl<A: Array> BorrowMut<[A::Item]> for StackVec<A> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [A::Item] {
        self
    }
}

#[cfg(feature = "std")]
impl<A: Array<Item = u8>> io::Write for StackVec<A> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a, A: Array> From<&'a [A::Item]> for StackVec<A> where A::Item: Clone {
    #[inline]
    fn from(slice: &'a [A::Item]) -> StackVec<A> {
        slice.into_iter().cloned().collect()
    }
}

impl<A: Array> From<Vec<A::Item>> for StackVec<A> {
    #[inline]
    fn from(vec: Vec<A::Item>) -> StackVec<A> {
        StackVec::from_vec(vec)
    }
}

impl<A: Array> From<A> for StackVec<A> {
    #[inline]
    fn from(array: A) -> StackVec<A> {
        StackVec::from_buf(array)
    }
}

macro_rules! impl_index {
    ($index_type: ty, $output_type: ty) => {
        impl<A: Array> ops::Index<$index_type> for StackVec<A> {
            type Output = $output_type;
            #[inline]
            fn index(&self, index: $index_type) -> &$output_type {
                &(&**self)[index]
            }
        }

        impl<A: Array> ops::IndexMut<$index_type> for StackVec<A> {
            #[inline]
            fn index_mut(&mut self, index: $index_type) -> &mut $output_type {
                &mut (&mut **self)[index]
            }
        }
    }
}

impl_index!(usize, A::Item);
impl_index!(ops::Range<usize>, [A::Item]);
impl_index!(ops::RangeFrom<usize>, [A::Item]);
impl_index!(ops::RangeFull, [A::Item]);
impl_index!(ops::RangeTo<usize>, [A::Item]);

#[cfg(has_range_inclusive)]
impl_index!(ops::RangeInclusive<usize>, [A::Item]);

#[cfg(has_range_inclusive)]
impl_index!(ops::RangeToInclusive<usize>, [A::Item]);

impl<A: Array> ExtendFromSlice<A::Item> for StackVec<A> where A::Item: Copy {
    fn extend_from_slice(&mut self, other: &[A::Item]) {
        StackVec::extend_from_slice(self, other)
    }
}

#[allow(deprecated)]
impl<A: Array> VecLike<A::Item> for StackVec<A> {
    #[inline]
    fn push(&mut self, value: A::Item) {
        StackVec::push(self, value);
    }

    #[inline]
    fn pop(&mut self) -> Option<A::Item> {
        StackVec::pop(self)
    }
}

impl<A: Array> iter::FromIterator<A::Item> for StackVec<A> {
    fn from_iter<I: iter::IntoIterator<Item=A::Item>>(iterable: I) -> StackVec<A> {
        let mut v = StackVec::new();
        v.extend(iterable);
        v
    }
}

impl<A: Array> Extend<A::Item> for StackVec<A> {
    fn extend<I: iter::IntoIterator<Item=A::Item>>(&mut self, iterable: I) {
        let mut iter = iterable.into_iter();
        let (lower_bound, upper_bound) = iter.size_hint();
        let upper_bound = upper_bound.expect("iterable must provide upper bound.");
        assert!(self.len() + upper_bound <= self.capacity());

        unsafe {
            let len = self.len();
            let ptr = self.as_mut_ptr().padd(len);
            let mut count = 0;
            while count < lower_bound {
                if let Some(out) = iter.next() {
                    ptr::write(ptr.padd(count), out);
                    count += 1;
                } else {
                    break;
                }
            }
            self.set_len(len + count);
        }

        for elem in iter {
            self.push(elem);
        }
    }
}

impl<A: Array> fmt::Debug for StackVec<A> where A::Item: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<A: Array> Default for StackVec<A> {
    #[inline]
    fn default() -> StackVec<A> {
        StackVec::new()
    }
}

impl<A: Array> Drop for StackVec<A> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(&mut self[..]);
        }
    }
}

impl<A: Array> Clone for StackVec<A> where A::Item: Clone {
    fn clone(&self) -> StackVec<A> {
        let mut new_vector = StackVec::new();
        for element in self.iter() {
            new_vector.push(element.clone())
        }
        new_vector
    }
}

impl<A: Array, B: Array> PartialEq<StackVec<B>> for StackVec<A>
    where A::Item: PartialEq<B::Item>
{
    #[inline]
    fn eq(&self, other: &StackVec<B>) -> bool {
        self[..] == other[..]
    }

    #[inline]
    fn ne(&self, other: &StackVec<B>) -> bool {
        self[..] != other[..]
    }
}

impl<A: Array> Eq for StackVec<A> where A::Item: Eq {
}

impl<A: Array> PartialOrd for StackVec<A> where A::Item: PartialOrd {
    #[inline]
    fn partial_cmp(&self, other: &StackVec<A>) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<A: Array> Ord for StackVec<A> where A::Item: Ord {
    #[inline]
    fn cmp(&self, other: &StackVec<A>) -> cmp::Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<A: Array> hash::Hash for StackVec<A> where A::Item: hash::Hash {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

unsafe impl<A: Array> Send for StackVec<A> where A::Item: Send {
}

/// An iterator that consumes a `StackVec` and yields its items by value.
///
/// Returned from [`StackVec::into_iter`][1].
///
/// [1]: struct.StackVec.html#method.into_iter
pub struct IntoIter<A: Array> {
    data: StackVec<A>,
    current: usize,
    end: usize,
}

impl<A: Array> Drop for IntoIter<A> {
    fn drop(&mut self) {
        for _ in self {
        }
    }
}

impl<A: Array> Iterator for IntoIter<A> {
    type Item = A::Item;

    #[inline]
    fn next(&mut self) -> Option<A::Item> {
        if self.current == self.end {
            None
        }
        else {
            unsafe {
                let current = self.current;
                self.current += 1;
                Some(ptr::read(self.data.as_ptr().padd(current)))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end - self.current;
        (size, Some(size))
    }
}

impl<A: Array> DoubleEndedIterator for IntoIter<A> {
    #[inline]
    fn next_back(&mut self) -> Option<A::Item> {
        if self.current == self.end {
            None
        }
        else {
            unsafe {
                self.end -= 1;
                Some(ptr::read(self.data.as_ptr().padd(self.end)))
            }
        }
    }
}

impl<A: Array> ExactSizeIterator for IntoIter<A> {
}

impl<A: Array> IntoIterator for StackVec<A> {
    type IntoIter = IntoIter<A>;
    type Item = A::Item;
    fn into_iter(mut self) -> Self::IntoIter {
        unsafe {
            // Set StackVec len to zero as `IntoIter` drop handles dropping of the elements
            let len = self.len();
            self.set_len(0);
            IntoIter {
                data: self,
                current: 0,
                end: len,
            }
        }
    }
}

impl<'a, A: Array> IntoIterator for &'a StackVec<A> {
    type IntoIter = slice::Iter<'a, A::Item>;
    type Item = &'a A::Item;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Array> IntoIterator for &'a mut StackVec<A> {
    type IntoIter = slice::IterMut<'a, A::Item>;
    type Item = &'a mut A::Item;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// STACKVEC MACRO

/// Creates a [`StackVec`] containing the arguments.
///
/// `stackvec!` allows `StackVec`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// - Create a [`StackVec`] containing a given list of elements:
///
/// ```
/// # #[macro_use] extern crate stackvector;
/// # use stackvector::StackVec;
/// # fn main() {
/// let v: StackVec<[_; 128]> = stackvec![1, 2, 3];
/// assert_eq!(v[0], 1);
/// assert_eq!(v[1], 2);
/// assert_eq!(v[2], 3);
/// # }
/// ```
///
/// - Create a [`StackVec`] from a given element and size:
///
/// ```
/// # #[macro_use] extern crate stackvector;
/// # use stackvector::StackVec;
/// # fn main() {
/// let v: StackVec<[_; 0x8000]> = stackvec![1; 3];
/// assert_eq!(v, StackVec::from_buf([1, 1, 1]));
/// # }
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the number of elements doesn't have to be
/// a constant.
///
/// This will use `clone` to duplicate an expression, so one should be careful
/// using this with types having a nonstandard `Clone` implementation. For
/// example, `stackvec![Rc::new(1); 5]` will create a vector of five references
/// to the same boxed integer value, not five references pointing to independently
/// boxed integers.
#[macro_export]
macro_rules! stackvec {
    // count helper: transform any expression into 1
    (@one $x:expr) => (1usize);
    ($elem:expr; $n:expr) => ({
        $crate::StackVec::from_elem($elem, $n)
    });
    ($($x:expr),*$(,)*) => ({
        // Allow an unused mut variable, since if the sequence is empty,
        // the vec will never be mutated.
        #[allow(unused_mut)] {
            let mut vec = $crate::StackVec::new();
            $(vec.push($x);)*
            vec
        }
    });
}

// TESTS
// -----

#[cfg(test)]
mod test {
    use super::*;
    use super::lib::iter::FromIterator;
    use super::lib::rc::Rc;

    #[test]
    pub fn test_zero() {
        let v = StackVec::<[usize; 0]>::new();
        assert_eq!(v.len(), 0);
    }

    #[test]
    #[should_panic]
    pub fn test_panic() {
        let mut v = StackVec::<[usize; 0]>::new();
        v.push(0);
    }

    // We heap allocate all these strings so that double frees will show up under valgrind.

    #[test]
    pub fn test_inline() {
        let mut v = StackVec::<[_; 16]>::new();
        v.push("hello".to_owned());
        v.push("there".to_owned());
        assert_eq!(&*v, &[
            "hello".to_owned(),
            "there".to_owned(),
        ][..]);
    }

    #[test]
    #[should_panic]
    pub fn test_spill() {
        let mut v = StackVec::<[_; 2]>::new();
        v.push("hello".to_owned());
        assert_eq!(v[0], "hello");
        v.push("there".to_owned());
        v.push("burma".to_owned());
        assert_eq!(v[0], "hello");
        v.push("shave".to_owned());
        assert_eq!(&*v, &[
            "hello".to_owned(),
            "there".to_owned(),
            "burma".to_owned(),
            "shave".to_owned(),
        ][..]);
    }

    #[test]
    #[should_panic]
    pub fn test_double_spill() {
        let mut v = StackVec::<[_; 2]>::new();
        v.push("hello".to_owned());
        v.push("there".to_owned());
        v.push("burma".to_owned());
        v.push("shave".to_owned());
        v.push("hello".to_owned());
        v.push("there".to_owned());
        v.push("burma".to_owned());
        v.push("shave".to_owned());
        assert_eq!(&*v, &[
            "hello".to_owned(),
            "there".to_owned(),
            "burma".to_owned(),
            "shave".to_owned(),
            "hello".to_owned(),
            "there".to_owned(),
            "burma".to_owned(),
            "shave".to_owned(),
        ][..]);
    }

    /// https://github.com/servo/rust-smallvec/issues/4
    #[test]
    fn issue_4() {
        StackVec::<[Box<u32>; 2]>::new();
    }

    /// https://github.com/servo/rust-smallvec/issues/5
    #[test]
    fn issue_5() {
        assert!(Some(StackVec::<[&u32; 2]>::new()).is_some());
    }

    #[test]
    fn drain_test() {
        let mut v: StackVec<[u8; 2]> = StackVec::new();
        v.push(3);
        assert_eq!(v.drain().collect::<Vec<_>>(), &[3]);
    }

    #[test]
    fn drain_rev_test() {
        let mut v: StackVec<[u8; 2]> = StackVec::new();
        v.push(3);
        assert_eq!(v.drain().rev().collect::<Vec<_>>(), &[3]);
    }

    #[test]
    fn into_iter() {
        let mut v: StackVec<[u8; 2]> = StackVec::new();
        v.push(3);
        assert_eq!(v.into_iter().collect::<Vec<_>>(), &[3]);
    }

    #[test]
    fn into_iter_rev() {
        let mut v: StackVec<[u8; 2]> = StackVec::new();
        v.push(3);
        assert_eq!(v.into_iter().rev().collect::<Vec<_>>(), &[3]);
    }

    #[test]
    fn into_iter_drop() {
        use lib::cell::Cell;

        struct DropCounter<'a>(&'a Cell<i32>);

        impl<'a> Drop for DropCounter<'a> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        {
            let cell = Cell::new(0);
            let mut v: StackVec<[DropCounter; 2]> = StackVec::new();
            v.push(DropCounter(&cell));
            v.into_iter();
            assert_eq!(cell.get(), 1);
        }

        {
            let cell = Cell::new(0);
            let mut v: StackVec<[DropCounter; 2]> = StackVec::new();
            v.push(DropCounter(&cell));
            v.push(DropCounter(&cell));
            assert!(v.into_iter().next().is_some());
            assert_eq!(cell.get(), 2);
        }
    }

    #[test]
    fn test_capacity() {
        let v: StackVec<[u8; 2]> = StackVec::new();
        assert_eq!(v.capacity(), 2);
    }

    #[test]
    fn test_truncate() {
        let mut v: StackVec<[Box<u8>; 8]> = StackVec::new();

        for x in 0..8 {
            v.push(Box::new(x));
        }
        v.truncate(4);

        assert_eq!(v.len(), 4);

        assert_eq!(*v.swap_remove(1), 1);
        assert_eq!(*v.remove(1), 3);
        v.insert(1, Box::new(3));

        assert_eq!(&v.iter().map(|v| **v).collect::<Vec<_>>(), &[0, 3, 2]);
    }

    #[test]
    fn test_insert_many() {
        let mut v: StackVec<[u8; 8]> = StackVec::new();
        for x in 0..4 {
            v.push(x);
        }
        assert_eq!(v.len(), 4);
        v.insert_many(1, [5, 6].iter().cloned());
        assert_eq!(&v.iter().map(|v| *v).collect::<Vec<_>>(), &[0, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn test_insert_from_slice() {
        let mut v: StackVec<[u8; 8]> = StackVec::new();
        for x in 0..4 {
            v.push(x);
        }
        assert_eq!(v.len(), 4);
        v.insert_from_slice(1, &[5, 6]);
        assert_eq!(&v.iter().map(|v| *v).collect::<Vec<_>>(), &[0, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn test_extend_from_slice() {
        let mut v: StackVec<[u8; 8]> = StackVec::new();
        for x in 0..4 {
            v.push(x);
        }
        assert_eq!(v.len(), 4);
        v.extend_from_slice(&[5, 6]);
        assert_eq!(&v.iter().map(|v| *v).collect::<Vec<_>>(), &[0, 1, 2, 3, 5, 6]);
    }

    #[test]
    #[should_panic]
    fn test_drop_panic_smallvec() {
        // This test should only panic once, and not double panic,
        // which would mean a double drop
        struct DropPanic;

        impl Drop for DropPanic {
            fn drop(&mut self) {
                panic!("drop");
            }
        }

        let mut v = StackVec::<[_; 1]>::new();
        v.push(DropPanic);
    }

    #[test]
    fn test_eq() {
        let mut a: StackVec<[u32; 2]> = StackVec::new();
        let mut b: StackVec<[u32; 2]> = StackVec::new();
        let mut c: StackVec<[u32; 2]> = StackVec::new();
        // a = [1, 2]
        a.push(1);
        a.push(2);
        // b = [1, 2]
        b.push(1);
        b.push(2);
        // c = [3, 4]
        c.push(3);
        c.push(4);

        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn test_ord() {
        let mut a: StackVec<[u32; 2]> = StackVec::new();
        let mut b: StackVec<[u32; 2]> = StackVec::new();
        let mut c: StackVec<[u32; 2]> = StackVec::new();
        // a = [1]
        a.push(1);
        // b = [1, 1]
        b.push(1);
        b.push(1);
        // c = [1, 2]
        c.push(1);
        c.push(2);

        assert!(a < b);
        assert!(b > a);
        assert!(b < c);
        assert!(c > b);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_hash() {
        use std::hash::Hash;
        use std::collections::hash_map::DefaultHasher;

        {
            let mut a: StackVec<[u32; 2]> = StackVec::new();
            let b = [1, 2];
            a.extend(b.iter().cloned());
            let mut hasher = DefaultHasher::new();
            assert_eq!(a.hash(&mut hasher), b.hash(&mut hasher));
        }
        {
            let mut a: StackVec<[u32; 4]> = StackVec::new();
            let b = [1, 2, 11, 12];
            a.extend(b.iter().cloned());
            let mut hasher = DefaultHasher::new();
            assert_eq!(a.hash(&mut hasher), b.hash(&mut hasher));
        }
    }

    #[test]
    fn test_as_ref() {
        let mut a: StackVec<[u32; 3]> = StackVec::new();
        a.push(1);
        assert_eq!(a.as_ref(), [1]);
        a.push(2);
        assert_eq!(a.as_ref(), [1, 2]);
        a.push(3);
        assert_eq!(a.as_ref(), [1, 2, 3]);
    }

    #[test]
    fn test_as_mut() {
        let mut a: StackVec<[u32; 3]> = StackVec::new();
        a.push(1);
        assert_eq!(a.as_mut(), [1]);
        a.push(2);
        assert_eq!(a.as_mut(), [1, 2]);
        a.push(3);
        assert_eq!(a.as_mut(), [1, 2, 3]);
        a.as_mut()[1] = 4;
        assert_eq!(a.as_mut(), [1, 4, 3]);
    }

    #[test]
    fn test_borrow() {
        use std::borrow::Borrow;

        let mut a: StackVec<[u32; 3]> = StackVec::new();
        a.push(1);
        assert_eq!(a.borrow(), [1]);
        a.push(2);
        assert_eq!(a.borrow(), [1, 2]);
        a.push(3);
        assert_eq!(a.borrow(), [1, 2, 3]);
    }

    #[test]
    fn test_borrow_mut() {
        use std::borrow::BorrowMut;

        let mut a: StackVec<[u32; 3]> = StackVec::new();
        a.push(1);
        assert_eq!(a.borrow_mut(), [1]);
        a.push(2);
        assert_eq!(a.borrow_mut(), [1, 2]);
        a.push(3);
        assert_eq!(a.borrow_mut(), [1, 2, 3]);
        BorrowMut::<[u32]>::borrow_mut(&mut a)[1] = 4;
        assert_eq!(a.borrow_mut(), [1, 4, 3]);
    }

    #[test]
    fn test_from() {
        assert_eq!(&StackVec::<[u32; 2]>::from(&[1][..])[..], [1]);
        assert_eq!(&StackVec::<[u32; 3]>::from(&[1, 2, 3][..])[..], [1, 2, 3]);

        let vec = vec![];
        let stack_vec: StackVec<[u8; 3]> = StackVec::from(vec);
        assert_eq!(&*stack_vec, &[]);
        drop(stack_vec);

        let vec = vec![1, 2, 3, 4, 5];
        let stack_vec: StackVec<[u8; 5]> = StackVec::from(vec);
        assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
        drop(stack_vec);

        let vec = vec![1, 2, 3, 4, 5];
        let stack_vec: StackVec<[u8; 5]> = StackVec::from(vec);
        assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
        drop(stack_vec);

        let array = [1];
        let stack_vec: StackVec<[u8; 1]> = StackVec::from(array);
        assert_eq!(&*stack_vec, &[1]);
        drop(stack_vec);

        let array = [99; 128];
        let stack_vec: StackVec<[u8; 128]> = StackVec::from(array);
        assert_eq!(&*stack_vec, vec![99u8; 128].as_slice());
        drop(stack_vec);
    }

    #[test]
    fn test_from_slice() {
        assert_eq!(&StackVec::<[u32; 2]>::from_slice(&[1][..])[..], [1]);
        assert_eq!(&StackVec::<[u32; 3]>::from_slice(&[1, 2, 3][..])[..], [1, 2, 3]);
    }

    #[test]
    fn test_exact_size_iterator() {
        let mut vec = StackVec::<[u32; 3]>::from(&[1, 2, 3][..]);
        assert_eq!(vec.clone().into_iter().len(), 3);
        assert_eq!(vec.drain().len(), 3);
    }

    #[test]
    #[allow(deprecated)]
    fn veclike_deref_slice() {
        use super::VecLike;

        fn test<T: VecLike<i32>>(vec: &mut T) {
            assert!(!vec.is_empty());
            assert_eq!(vec.len(), 3);

            vec.sort();
            assert_eq!(&vec[..], [1, 2, 3]);
        }

        let mut vec = StackVec::<[i32; 3]>::from(&[3, 1, 2][..]);
        test(&mut vec);
    }

    #[test]
    fn test_into_vec() {
        let vec = StackVec::<[u8; 2]>::from_iter(0..2);
        assert_eq!(vec.into_vec(), vec![0, 1]);

        let vec = StackVec::<[u8; 3]>::from_iter(0..3);
        assert_eq!(vec.into_vec(), vec![0, 1, 2]);
    }

    #[test]
    fn test_into_inner() {
        let vec = StackVec::<[u8; 2]>::from_iter(0..2);
        assert_eq!(vec.into_inner(), Ok([0, 1]));

        let vec = StackVec::<[u8; 2]>::from_iter(0..1);
        assert_eq!(vec.clone().into_inner(), Err(vec));

        let vec = StackVec::<[u8; 3]>::from_iter(0..3);
        assert_eq!(vec.clone().into_inner(), Ok([0, 1, 2]));

        let vec = StackVec::<[u8; 4]>::from_iter(0..3);
        assert_eq!(vec.clone().into_inner(), Err(vec));
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![];
        let stack_vec: StackVec<[u8; 3]> = StackVec::from_vec(vec);
        assert_eq!(&*stack_vec, &[]);
        drop(stack_vec);

        let vec = vec![];
        let stack_vec: StackVec<[u8; 1]> = StackVec::from_vec(vec);
        assert_eq!(&*stack_vec, &[]);
        drop(stack_vec);

        let vec = vec![1];
        let stack_vec: StackVec<[u8; 3]> = StackVec::from_vec(vec);
        assert_eq!(&*stack_vec, &[1]);
        drop(stack_vec);

        let vec = vec![1, 2, 3];
        let stack_vec: StackVec<[u8; 3]> = StackVec::from_vec(vec);
        assert_eq!(&*stack_vec, &[1, 2, 3]);
        drop(stack_vec);

        let vec = vec![1, 2, 3, 4, 5];
        let stack_vec: StackVec<[u8; 5]> = StackVec::from_vec(vec);
        assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
        drop(stack_vec);
    }

    #[test]
    fn test_retain() {
        let mut sv: StackVec<[i32; 5]> = StackVec::from_slice(&[1, 2, 3, 3, 4]);
        sv.retain(|&mut i| i != 3);
        assert_eq!(sv.pop(), Some(4));
        assert_eq!(sv.pop(), Some(2));
        assert_eq!(sv.pop(), Some(1));
        assert_eq!(sv.pop(), None);

        // Test that drop implementations are called for inline.
        let one = Rc::new(1);
        let mut sv: StackVec<[Rc<i32>; 3]> = StackVec::new();
        sv.push(Rc::clone(&one));
        assert_eq!(Rc::strong_count(&one), 2);
        sv.retain(|_| false);
        assert_eq!(Rc::strong_count(&one), 1);
    }

    #[test]
    fn test_dedup() {
        let mut dupes: StackVec<[i32; 5]> = StackVec::from_slice(&[1, 1, 2, 3, 3]);
        dupes.dedup();
        assert_eq!(&*dupes, &[1, 2, 3]);

        let mut empty: StackVec<[i32; 5]> = StackVec::new();
        empty.dedup();
        assert!(empty.is_empty());

        let mut all_ones: StackVec<[i32; 5]> = StackVec::from_slice(&[1, 1, 1, 1, 1]);
        all_ones.dedup();
        assert_eq!(all_ones.len(), 1);

        let mut no_dupes: StackVec<[i32; 5]> = StackVec::from_slice(&[1, 2, 3, 4, 5]);
        no_dupes.dedup();
        assert_eq!(no_dupes.len(), 5);
    }

    #[test]
    fn test_resize() {
        let mut v: StackVec<[i32; 8]> = StackVec::new();
        v.push(1);
        v.resize(5, 0);
        assert_eq!(v[..], [1, 0, 0, 0, 0][..]);

        v.resize(2, -1);
        assert_eq!(v[..], [1, 0][..]);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_write() {
        use io::Write;

        let data = [1, 2, 3, 4, 5];

        let mut small_vec: StackVec<[u8; 5]> = StackVec::new();
        let len = small_vec.write(&data[..]).unwrap();
        assert_eq!(len, 5);
        assert_eq!(small_vec.as_ref(), data.as_ref());

        let mut small_vec: StackVec<[u8; 5]> = StackVec::new();
        small_vec.write_all(&data[..]).unwrap();
        assert_eq!(small_vec.as_ref(), data.as_ref());
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        assert_eq!(<[usize; 0] as Array>::size(), 0);
        assert_eq!(<[i32; 0] as Array>::size(), 0);
        assert_eq!(<[String; 0] as Array>::size(), 0);
    }
}#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr() {
        let mut p0: [i32; 0] = [];

        <[i32; 0]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_5 {
    use super::*;

    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 0] = [];

        <[i32; 0]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 1]>::size();
    }
}#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let p0: [i32; 1] = [10];

        <[i32; 1]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 1] = [0];

        <[i32; 1]>::ptr_mut(&mut p0);

        // Add assertions or further tests if needed
    }
}#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        assert_eq!(<[usize; 2] as Array>::size(), 2);
    }
}#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 2] = [1, 2];       

        <[i32; 2] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 2] = [1, 2];

        <[i32; 2]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[i32; 3]>::size();
    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::Array;
    #[test]
    fn test_rug() {
        let mut p0: [i32; 3] = [1, 2, 3];

        p0.ptr();
    }
}#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 3] = [1, 2, 3];

        <[i32; 3]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 4]>::size();
    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 4] = [1, 2, 3, 4];
        
        <[i32; 4] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 4] = [1, 2, 3, 4];

        <[i32; 4]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        let size = <[i32; 5]>::size();
        assert_eq!(size, 5);
    }
}#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 5] = [1, 2, 3, 4, 5];

        <[i32; 5] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 5] = [1, 2, 3, 4, 5];

        <[i32; 5]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let _: [u32; 6] = [1, 2, 3, 4, 5, 6];
        <[u32; 6]>::size();
    }
}#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let mut p0: [i32; 6] = [1, 2, 3, 4, 5, 6];

        <[i32; 6] as Array>::ptr(&p0);
    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 6] = [1, 2, 3, 4, 5, 6];
        
        <[i32; 6]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 7]>::size();
    }
}#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];

        <[u32; 7] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];

        <[i32; 7] as Array>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        assert_eq!(<[(); 8] as Array>::size(), 8);
    }
}#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let mut p0: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

        p0.ptr();
    }
}#[cfg(test)]
mod tests_rug_29 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        
        <[i32; 8]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let size: usize = 9;  // Sample data
        
        <[i32; 9]>::size();  // Sample data
    }
}#[cfg(test)]
mod tests_rug_31 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        <[i32; 9]>::ptr(&p0);
    }
}
#[cfg(test)]
mod tests_rug_32 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        <[i32; 9]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_34 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr() {
        let mut p0: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        <[i32; 10]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 10] = [0; 10];

        <[i32; 10]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        <[usize; 11]>::size();
    }
}#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        
        <[i32; 11]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        <[i32; 11]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_size() {
        let size = <[u32; 12]>::size();
        assert_eq!(size, 12);
    }
}#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr() {
        let mut p0: [i32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        <[i32; 12]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_41 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 12] = [0; 12];
        
        <[i32; 12]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_42 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let size = <[i32; 13] as Array>::size();
        assert_eq!(size, 13);
    }
}
#[cfg(test)]
mod tests_rug_43 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr() {
        let mut p0: [i32; 13] = [0; 13];

        <[i32; 13] as Array>::ptr(&p0);
    }
}

#[cfg(test)]
mod tests_rug_44 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [u32; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

        <[u32; 13] as Array>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_45 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let size = <[usize; 14]>::size();
        assert_eq!(size, 14);
    }
}#[cfg(test)]
mod tests_rug_46 {
    use super::*;
    use crate::Array;
    #[test]
    fn test_rug() {
        let mut p0: [i32; 14] = [0; 14];

        p0.ptr();
    }
}#[cfg(test)]
mod tests_rug_47 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 14] = Default::default();

        <[i32; 14]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_48 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 15]>::size();
    }
}
#[cfg(test)]
mod tests_rug_49 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr() {
        let mut p0: [i32; 15] = [0; 15];
        
        <[i32; 15]>::ptr(&p0);
    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 15] = [0; 15];

        <[i32; 15]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_51 {
    use super::*;

    use crate::Array;

    #[test]
    fn test_rug() {
        <[i32; 16]>::size();
    }
}#[cfg(test)]
mod tests_rug_52 {
    use super::*;
    use crate::Array;
    #[test]
    fn test_rug() {
        let mut p0: [i32; 16] = [0; 16];

        <[i32; 16]>::ptr(&p0);

    }
}#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 16] = [0; 16];

        <[i32; 16]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_54 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 20]>::size();
    }
}
#[cfg(test)]
mod tests_rug_55 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 20] = [0; 20];
        
        p0.ptr();
    }
}

#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 20] = [0; 20];

        <[i32; 20] as Array>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_57 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_stackvector_size() {
        let size = <[i32; 24]>::size();
        assert_eq!(size, 24);
    }
}#[cfg(test)]
mod tests_rug_58 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let mut p0: [i32; 24] = [0; 24];

        <[i32; 24] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_59 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 24] = [0; 24];
        
        <[i32; 24]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let size = <[usize; 32]>::size();
        assert_eq!(size, 32);
    }
}#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 32] = [0; 32];
        
        <[i32; 32]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::Array;

    use std::ptr;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 32] = [0; 32];

        <[i32; 32]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_63 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let size = 36;
        <[usize; 36]>::size();
    }
}#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let mut p0: [i32; 36] = [0; 36];

        <[i32; 36]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_65 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 36] = [0; 36];
        
        <[i32; 36]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_66 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        <[u32; 64]>::size();
    }
}#[cfg(test)]
mod tests_rug_67 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 64] = [0; 64];
        
        <[i32; 64]>::ptr(&p0);

    }
}
#[cfg(test)]
mod tests_rug_68 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 64] = [0; 64];

        <[i32; 64]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_69 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_stackvector_size() {
        <[usize; 128] as Array>::size();
    }
}#[cfg(test)]
mod tests_rug_70 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 128] = [0; 128];

        <[i32; 128] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_71 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [u32; 128] = [0; 128];
        
        <[u32; 128]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        let size = <[usize; 256]>::size();
        assert_eq!(size, 256);
    }
}#[cfg(test)]
mod tests_rug_73 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 256] = [0; 256];

        <[i32; 256] as Array>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_74 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 256] = [0; 256];

        <[i32; 256]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_75 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 512] as Array>::size();
    }
}
#[cfg(test)]
mod tests_rug_76 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 512] = [0; 512];

        <[i32; 512]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_77 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 512] = [0; 512];

        <[i32; 512] as Array>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_78 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let size_result = <[i32; 1024]>::size();
        assert_eq!(size_result, 1024);
    }
}#[cfg(test)]
mod tests_rug_79 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 1024] = [0; 1024];
                
        p0.ptr();
    }
}
#[cfg(test)]
mod tests_rug_80 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 1024] = [0; 1024];

        <[i32; 1024] as Array>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_81 {
    use super::*;

    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 2048]>::size();
    }
}#[cfg(test)]
mod tests_rug_82 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let mut p0: [u32; 2048] = [0; 2048];

        p0.ptr();
    }
}#[cfg(test)]
mod tests_rug_83 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 2048] = [0; 2048];
        
        <[i32; 2048] as Array>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_84 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[u32; 4096]>::size();
    }
}#[cfg(test)]
mod tests_rug_85 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 4096] = [0; 4096];

        <[i32; 4096]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_86 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut arr: [i32; 4096] = [0; 4096];
        
        <[i32; 4096]>::ptr_mut(&mut arr);
    }
}#[cfg(test)]
mod tests_rug_87 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[u32; 8192]>::size();
    }
}#[cfg(test)]
mod tests_rug_88 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr() {
        let mut p0: [i32; 8192] = [0; 8192];

        <[i32; 8192]>::ptr(&p0);
    }
}#[cfg(test)]
mod tests_rug_89 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 8192] = [0; 8192];

        <[i32; 8192] as Array>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_90 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        <[usize; 16384]>::size();
    }
}
#[cfg(test)]
mod tests_rug_91 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr() {
        let mut p0: [i32; 16384] = [0; 16384];

        <[i32; 16384] as Array>::ptr(&p0);
    }
}
#[cfg(test)]
mod tests_rug_92 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        // Create a StackVector with capacity 16384 containing u32 elements
        let mut p0 = [0u32; 16384];
        
        <[u32; 16384]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_size() {
        let _ = <[i32; 32768]>::size(); // Sample: Using i32 as the type and 32768 as the size
    }
}
#[cfg(test)]
mod tests_rug_94 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 32768] = [0; 32768];

        <[i32; 32768] as Array>::ptr(&p0);
    }
}
#[cfg(test)]
mod tests_rug_95 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 32768] = [0; 32768];
        
        <[i32; 32768]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_96 {
    use super::*;
    use crate::Array;
    #[test]
    fn test_rug() {
        <[usize; 65536] as Array>::size();
    }
}#[cfg(test)]
mod tests_rug_97 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 65536] = [0; 65536];

        <[i32; 65536]>::ptr(&p0);
    }
}
#[cfg(test)]
mod tests_rug_98 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 65536] = [0; 65536];

        <[i32; 65536]>::ptr_mut(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_99 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[u32; 131072] as Array>::size();
    }
}
#[cfg(test)]
mod tests_rug_100 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 131072] = [0; 131072];

        p0.ptr();
    }
}#[cfg(test)]
mod tests_rug_101 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 131072] = [0; 131072];
        
        <[i32; 131072]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_102 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_size() {
        let size_result: usize = <[u8; 262144]>::size();
        assert_eq!(262144, size_result);
    }
}#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 262144] = [0; 262144];
        
        p0.ptr();
    }
}#[cfg(test)]
mod tests_rug_104 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 262144] = [0; 262144];

        <[i32; 262144]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_105 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[u32; 524288]>::size();
    }
}#[cfg(test)]
mod tests_rug_106 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        let mut p0: [i32; 524288] = [0; 524288];
        
        <[i32; 524288] as Array>::ptr(&p0);

    }
}#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 524288] = [0; 524288];

        <[i32; 524288]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_108 {
    use super::*;
    use crate::Array;

    #[test]
    fn test_rug() {
        <[usize; 1048576]>::size();
    }
}        
#[cfg(test)]
mod tests_rug_109 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_rug() {
        let mut p0: [i32; 1048576] = [0; 1048576];
        
        <[i32; 1048576] as Array>::ptr(&p0);
    }
}
                    #[cfg(test)]
mod tests_rug_110 {
    use super::*;
    use crate::Array;
    
    #[test]
    fn test_ptr_mut() {
        let mut p0: [i32; 1048576] = [0; 1048576];

        <[i32; 1048576]>::ptr_mut(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    use crate::VecLike;
    use std::vec::Vec;
    
    #[test]
    fn test_rug() {
        let mut p0: Vec<i32> = Vec::new();
        let p1: i32 = 42;

        p0.push(p1);
    }
}#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    use crate::VecLike;
  
    #[test]
    fn test_rug() {
        let mut p0: Vec<i32> = Vec::new();

        <std::vec::Vec<i32> as VecLike<i32>>::pop(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;

    use crate::ExtendFromSlice;
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0: Vec<i32> = Vec::new();
        let p1: &[i32] = &[1, 2, 3];

        <Vec<i32> as ExtendFromSlice<i32>>::extend_from_slice(&mut p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::std::iter::Iterator;

    use crate::Drain;

    #[test]
    fn test_rug() {
        let mut p0: Drain<'static, i32> = unimplemented!();

        p0.size_hint();
    }
}#[cfg(test)]
mod tests_rug_118 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut len: usize = 5;
        let mut p0: &mut usize = &mut len;

        <SetLenOnDrop>::new(p0);
    }
}#[cfg(test)]
mod tests_rug_122 {
    use super::*;
    use crate::StackVec;

    #[test]
    fn test_from_vec() {
        let mut p0 = {
            let mut v = Vec::new();
            v.push(1);
            v.push(2);
            v.push(3);
            v
        };

        StackVec::<[_; 5]>::from_vec(p0);
    }
}#[cfg(test)]
mod tests_rug_124 {
    use super::*;
    use crate::{StackVec, Array};
    use std::mem;

    struct MyArray<T> {
        array: [T; 5],
    }

    unsafe impl<T: Copy> Array for MyArray<T> {
        type Item = T;

        fn size() -> usize {
            5
        }

        fn ptr(&self) -> *const Self::Item {
            self.array.as_ptr()
        }

        fn ptr_mut(&mut self) -> *mut Self::Item {
            self.array.as_mut_ptr()
        }
    }

    #[test]
    fn test_rug() {
        let mut p0 = MyArray { array: [1, 2, 3, 4, 5] };

        let stack_vec: StackVec<MyArray<i32>> = StackVec::from_buf(p0);

        assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
    }
}#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::{StackVec, Array};

    struct ConcreteArray<T> {
        data: [T; 8],
    }

    unsafe impl<T> Array for ConcreteArray<T> {
        type Item = T;

        fn size() -> usize {
            8
        }

        fn ptr(&self) -> *const Self::Item {
            self.data.as_ptr()
        }

        fn ptr_mut(&mut self) -> *mut Self::Item {
            self.data.as_mut_ptr()
        }
    }

    #[test]
    fn test_rug() {
        let mut p0 = ConcreteArray { data: [1, 2, 3, 4, 5, 0, 0, 0] };
        let p1: usize = 5;

        
        <StackVec<ConcreteArray<_>>>::from_buf_and_len(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_126 {
    use super::*;
    use crate::{StackVec, Array};
    use std::{mem, ptr};

    struct MyArray {
        data: [i32; 8],
    }

    unsafe impl Array for MyArray {
        type Item = i32;

        fn size() -> usize {
            8
        }

        fn ptr(&self) -> *const Self::Item {
            self.data.as_ptr()
        }

        fn ptr_mut(&mut self) -> *mut Self::Item {
            self.data.as_mut_ptr()
        }
    }

    #[test]
    fn test_rug() {
        let mut p0 = MyArray { data: [1, 2, 3, 4, 5, 0, 0, 0] };
        let p1: usize = 5;

        let stack_vec: StackVec<MyArray> = unsafe {
            StackVec::from_buf_and_len_unchecked(p0, p1)
        };
        
        assert_eq!(&*stack_vec, &[1, 2, 3, 4, 5]);
    }
}#[cfg(test)]
mod tests_rug_132 {
    use super::*;
    use crate::StackVec;

    #[test]
    fn test_rug() {
        let mut p0: StackVec<[_; 2]> = StackVec::new(); // Sample
        p0.push(1); // Sample

        let mut p1: i32 = 2; // Sample

        p0.push(p1);
    }
}#[cfg(test)]
mod tests_rug_133 {
    use super::*;

    #[test]
    fn test_pop() {
        let mut p0 = StackVec::<[u32; 4]>::new();
        p0.push(1);
        p0.push(2);
        
        assert_eq!(p0.pop(), Some(2));
        assert_eq!(p0.pop(), Some(1));
        assert_eq!(p0.pop(), None);
    }
}
#[cfg(test)]
mod tests_rug_139 {
    use super::*;

    use crate::StackVec;

    #[test]
    fn test_rug() {
        let mut data = [1, 2, 3, 4];
        let mut stack_vec = StackVec::<[i32; 4]>::from(data);
        let index = 1;

        let removed_item = stack_vec.remove(index);

        assert_eq!(removed_item, 2);
        assert_eq!(stack_vec.as_slice(), [1, 3, 4]);
    }
}#[cfg(test)]
mod tests_rug_141 {
    use super::*;
    use crate::{StackVec, Array};

    #[test]
    fn test_rug() {
        let mut p0: StackVec<[i32; 5]> = StackVec::from_elem(0, 5);
        let p1: usize = 2;
        let mut p2: StackVec<[i32; 5]> = StackVec::from_elem(1, 2);
                
        p0.insert_many(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_151 {
    use super::*;
    use crate::{StackVec, Array};
    use std::iter;
    
    #[test]
    fn test_rug() {
        let mut p0: StackVec<[i32; 5]> = StackVec::from_elem(0, 5);
        let p1: usize = 8;
        let mut p2: i32 = 3;
        
        p0.resize(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_152 {
    use super::*;
    use crate::{StackVec, SetLenOnDrop};

    #[test]
    fn test_rug() {
        let mut p0: StackVec<[char; 128]> = StackVec::from_elem('d', 2);
        let mut p1: usize = 2;

        StackVec::<[char; 128]>::from_elem('d', 2);
    }
}#[cfg(test)]
mod tests_rug_164 {
    use super::*;
    use crate::StackVec;
    use crate::std::convert::From;


    struct ConcreteArray<T> {
        data: [T; 5],
    }

    unsafe impl<T> Array for ConcreteArray<T> {
        type Item = T;

        fn size() -> usize {
            5
        }

        fn ptr(&self) -> *const Self::Item {
            self.data.as_ptr()
        }

        fn ptr_mut(&mut self) -> *mut Self::Item {
            self.data.as_mut_ptr()
        }
    }

    #[test]
    fn test_rug() {
        let mut p0 = ConcreteArray { data: [1, 2, 3, 4, 5] };

        <StackVec<ConcreteArray<i32>> as std::convert::From<ConcreteArray<i32>>>::from(p0);
    }
}#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    use crate::std::iter::FromIterator;
    use crate::{StackVec, Array};

    #[test]
    fn test_rug() {
        let mut p0: StackVec<[i32; 5]> = StackVec::from_elem(0, 5);
                
        <StackVec<[i32; 5]>>::from_iter(p0);
    }
}#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    use crate::std::iter::Extend;
    use crate::{StackVec, Array};
    
    #[test]
    fn test_rug() {
        // Prepare the StackVec variable
        let mut p0: StackVec<[i32; 5]> = StackVec::from_elem(0, 5);
        
        // Prepare the iterable variable
        let mut p1 = [1, 2, 3, 4, 5];

        <StackVec<[i32; 5]>>::extend(&mut p0, &mut p1.iter().cloned());

        // Add assertions here
    }
}#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use crate::std::iter::Iterator;

    struct Data {
        data: Vec<i32>,
        current: usize,
        end: usize,
    }

    impl Data {
        fn new(data: Vec<i32>, current: usize, end: usize) -> Self {
            Data { data, current, end }
        }
    }

    impl Iterator for Data {
        type Item = i32;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.current == self.end {
                None
            } else {
                unsafe {
                    let current = self.current;
                    self.current += 1;
                    Some(std::ptr::read(self.data.as_ptr().add(current)))
                }
            }
        }
    }

    #[test]
    fn test_rug() {
        let mut p0 = Data::new(vec![1, 2, 3], 0, 3);

        p0.next();
    }
}