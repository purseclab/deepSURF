//! This is meant to be API compatible drop in replacement for std [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html),
//! but made compact by cramming capacity and length into a single 64bit word.
//!
//! ```rust
//! use std::mem::size_of;
//!
//! const WORD: usize = size_of::<usize>();
//!
//! assert_eq!(size_of::<Vec<u8>>(), WORD * 3);
//! assert_eq!(size_of::<ordnung::compact::Vec<u8>>(), WORD * 2);
//! ```
use alloc::vec::{Vec as StdVec, IntoIter};
use core::fmt;
use core::iter::FromIterator;
use core::mem::ManuallyDrop;
use core::ptr::{NonNull, slice_from_raw_parts_mut, slice_from_raw_parts};
use core::ops::{Deref, DerefMut};

/// A contiguous growable array type, written `Vec<T>` but pronounced 'vector'.
pub struct Vec<T> {
    ptr: NonNull<[T]>,
}

impl<T> Vec<T> {
    /// Constructs a new, empty Vec<T>.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    pub fn new() -> Self {
        Self::from_stdvec_unchecked(StdVec::new())
    }

    /// Constructs a new, empty Vec<T> with the specified capacity.
    ///
    /// The vector will be able to hold exactly capacity elements without reallocating. If capacity is 0, the vector will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_stdvec_unchecked(StdVec::with_capacity(capacity))
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `u32`.
    pub fn push(&mut self, val: T) {
        let ptr = self.as_mut_ptr();
        let (len, cap) = self.parts();

        if len == cap {
            let new_cap = match cap {
                0 => 1,
                n => n * 2,
            };

            if new_cap > MASK_LO {
                panic!("compact Vec capacity out of bounds");
            }

            // Create a new bigger buffer
            let mut stdvec = ManuallyDrop::new(StdVec::with_capacity(new_cap));

            unsafe {
                // Copy contents
                core::ptr::copy_nonoverlapping(ptr, stdvec.as_mut_ptr(), len);

                // Drop old buffer, len 0 (we don't want to drop content)
                core::mem::drop(StdVec::from_raw_parts(ptr, 0, cap));
            }

            self.ptr = unsafe { pack_unchecked(stdvec.as_mut_ptr(), len, stdvec.capacity()) }
        }
        unsafe { self.as_mut_ptr().add(len).write(val) }
        self.set_len(len + 1);
    }

    /// Removes the last element from a vector and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        let len = self.len().checked_sub(1)?;

        self.set_len(len);

        Some(unsafe {
            self.as_mut_ptr().add(len).read()
        })
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    pub fn clear(&mut self) {
        self.with(move |v| v.clear())
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        let (len, _) = self.parts();

        len
    }

    /// Returns the number of elements the vector can hold without reallocating.
    pub fn capacity(&self) -> usize {
        let (_, cap) = self.parts();

        cap
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    pub fn remove(&mut self, index: usize) -> T {
        self.with(move |v| v.remove(index))
    }

    /// Returns a raw pointer to the vector's buffer.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.ptr.cast().as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.cast().as_ptr()
    }

    fn set_len(&mut self, len: usize) {
        let (_, cap) = self.parts();

        self.ptr = unsafe {
            pack_unchecked(
                self.as_mut_ptr(),
                len,
                cap,
            )
        };
    }

    #[inline]
    fn parts(&self) -> (usize, usize) {
        let parts = unsafe { &*(self.ptr.as_ptr() as *const [()]) }.len();

        (parts & MASK_LO, (parts & MASK_HI) >> 32)
    }

    fn with<'a, R: 'a, F: FnOnce(&mut StdVec<T>) -> R>(&mut self, f: F) -> R {
        let (len, cap) = self.parts();

        let mut stdvec = unsafe {
            StdVec::from_raw_parts(self.as_mut_ptr(), len, cap)
        };

        let r = f(&mut stdvec);

        ManuallyDrop::new(core::mem::replace(self, Self::from_stdvec_unchecked(stdvec)));

        r
    }

    fn from_stdvec_unchecked(stdvec: StdVec<T>) -> Self {
        let mut stdvec = ManuallyDrop::new(stdvec);

        let ptr = stdvec.as_mut_ptr();
        let len = stdvec.len();
        let cap = stdvec.capacity();

        let ptr = slice_from_raw_parts_mut(
            ptr,
            len & MASK_LO | (cap & MASK_LO) << 32,
        );

        Vec {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }
}

impl<T> core::ops::Drop for Vec<T> {
    fn drop(&mut self) {
        let (len, cap) = self.parts();

        unsafe {
            StdVec::from_raw_parts(self.as_mut_ptr(), len, cap);
        }
    }
}

impl<T> From<StdVec<T>> for Vec<T> {
    fn from(stdvec: StdVec<T>) -> Self {
        let mut stdvec = ManuallyDrop::new(stdvec);

        let ptr = stdvec.as_mut_ptr();
        let len = stdvec.len();
        let cap = stdvec.capacity();

        Vec {
            ptr: unsafe { pack(ptr, len, cap) },
        }
    }
}

impl<T> From<Vec<T>> for StdVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut vec = ManuallyDrop::new(vec);
        let ptr = vec.as_mut_ptr();
        let (len, cap) = vec.parts();

        unsafe {
            StdVec::from_raw_parts(ptr, len, cap)
        }
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Vec<T> {
        Vec::from_stdvec_unchecked((&**self).to_vec())
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        let (len, _) = self.parts();

        unsafe {
            &*slice_from_raw_parts(self.as_ptr() as *mut T, len)
        }
    }
}

impl<T> DerefMut for Vec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        let (len, _) = self.parts();

        unsafe {
            &mut *slice_from_raw_parts_mut(self.as_mut_ptr() as *mut T, len)
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        StdVec::from(self).into_iter()
    }
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I>(iter: I) -> Vec<T>
    where
        I: IntoIterator<Item = T>,
    {
        Self::from(StdVec::from_iter(iter))
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.deref() == other.deref()
    }
}

unsafe impl<T: Sync> Sync for Vec<T> {}
unsafe impl<T: Send> Send for Vec<T> {}


const MASK_LO: usize = core::u32::MAX as usize;
const MASK_HI: usize = !(core::u32::MAX as usize);


#[inline]
unsafe fn pack<T>(ptr: *mut T, len: usize, capacity: usize) -> NonNull<[T]> {
    if (capacity & MASK_HI) != 0 {
        panic!("compact Vec capacity out of bounds");
    }

    pack_unchecked(ptr, len, capacity)
}


#[inline]
unsafe fn pack_unchecked<T>(ptr: *mut T, len: usize, capacity: usize) -> NonNull<[T]> {
    NonNull::new_unchecked(
        slice_from_raw_parts_mut(
            ptr as *mut T,
            (len & MASK_LO) | ((capacity & MASK_LO) << 32)
        )
    )
}
