use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, size_of};
use std::ptr::{read, drop_in_place};
use std::ops::{Deref, DerefMut};
use std::slice::{self, from_raw_parts, from_raw_parts_mut};
use std::iter::FromIterator;
use vector::Vector;
use array::{Array, ArrayIndex};
use util::PointerExt;

pub struct ArrayVec<T: Array> {
    array: ManuallyDrop<T>,
    len: T::Index,
}

impl<T: Array> fmt::Debug for ArrayVec<T> where T::Item: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self[..], fmt)
    }
}

impl<T: Array> Hash for ArrayVec<T> where T::Item: Hash {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self[..].hash(h)
    }
}

impl<T: Array> ArrayVec<T> {
    pub fn into_inner(mut self) -> Result<T, Self> {
        if self.len() == self.capacity() {
            self.len = Default::default();
            Ok(unsafe { read(&*self.array) })
        } else {
            Err(self)
        }
    }
}

impl<T: Array> Vector for ArrayVec<T> {
    type Item = T::Item;

    #[inline]
    fn with_capacity(cap: usize) -> Self {
        assert!(cap <= T::len());

        ArrayVec {
            array: ManuallyDrop::new(unsafe { T::uninitialized() }),
            len: Default::default(),
        }
    }

    #[inline]
    fn capacity(&self) -> usize { T::len() }
    #[inline]
    fn reserve(&mut self, additional: usize) { self.reserve_exact(additional); }
    #[inline]
    fn reserve_exact(&mut self, additional: usize) { assert!(T::len() - self.len() >= additional) }
    #[inline]
    fn shrink_to_fit(&mut self) { }

    fn into_boxed_slice(self) -> Box<[T::Item]> { unimplemented!() }

    #[inline]
    unsafe fn set_len(&mut self, len: usize) { self.len = ArrayIndex::from_usize(len); }

    #[inline]
    fn len(&self) -> usize { ArrayIndex::to_usize(self.len) }

    #[inline]
    fn as_ptr(&self) -> *const T::Item {
        self.array.as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T::Item {
        self.array.as_mut_ptr()
    }
}

impl<T: Array> Drop for ArrayVec<T> {
    fn drop(&mut self) {
        let len = ArrayIndex::to_usize(self.len);
        if len > 0 {
            let ptr = self.array.as_mut_ptr();

            unsafe {
                self.set_len(0);
                for i in 0..len {
                    drop_in_place(ptr.uoffset(i));
                }
            }
        }
    }
}

impl<T: Array> From<T> for ArrayVec<T> {
    fn from(array: T) -> Self {
        ArrayVec {
            array: ManuallyDrop::new(array),
            len: ArrayIndex::from_usize(T::len()),
        }
    }
}

pub struct ArrayVecIntoIter<T: Array> {
    inner: ArrayVec<T>,
    start: T::Index,
    end: T::Index,
}

impl<T: Array> ArrayVecIntoIter<T> {
    fn new(mut inner: ArrayVec<T>) -> Self {
        let (start, end) = unsafe {
            let len = inner.len();
            inner.set_len(0);
            (ArrayIndex::from_usize(0), ArrayIndex::from_usize(len))
        };

        ArrayVecIntoIter {
            inner: inner,
            start: start,
            end: end,
        }
    }

    fn ptr_size() -> usize {
        let size = size_of::<<Self as Iterator>::Item>();
        if size == 0 {
            1
        } else {
            size
        }
    }

    fn ptr(&mut self, index: T::Index) -> *mut <Self as Iterator>::Item {
        (self.inner.as_mut_ptr() as usize + ArrayIndex::to_usize(index) * Self::ptr_size()) as *mut _
    }
}

impl<T: Array> Iterator for ArrayVecIntoIter<T> {
    type Item = <ArrayVec<T> as Vector>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            unsafe {
                let start = self.start;
                self.start = ArrayIndex::from_usize(ArrayIndex::to_usize(start) + 1);
                Some(read(self.ptr(start)))
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = ArrayIndex::to_usize(self.end) - ArrayIndex::to_usize(self.start);
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<T: Array> DoubleEndedIterator for ArrayVecIntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            unsafe {
                self.end = ArrayIndex::from_usize(ArrayIndex::to_usize(self.end) - 1);
                let end = self.end;
                Some(read(self.ptr(end)))
            }
        } else {
            None
        }
    }
}

impl<T: Array> ExactSizeIterator for ArrayVecIntoIter<T> { }

impl<T: Array> IntoIterator for ArrayVec<T> {
    type Item = T::Item;
    type IntoIter = ArrayVecIntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ArrayVecIntoIter::new(self)
    }
}

impl<T: Array, R, RHS: Deref<Target=[R]>> PartialEq<RHS> for ArrayVec<T> where T::Item: PartialEq<R>
{
    fn eq(&self, other: &RHS) -> bool {
        **self == *other.deref()
    }
}

impl<T: Array> Eq for ArrayVec<T> where T::Item: Eq { }

impl<T: Array> Drop for ArrayVecIntoIter<T> {
    fn drop(&mut self) {
        for _ in self { }
    }
}

impl<T: Array> Clone for ArrayVec<T> where T::Item: Clone {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: Array> Default for ArrayVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Array> Extend<T::Item> for ArrayVec<T> {
    fn extend<I: IntoIterator<Item=T::Item>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for v in iter {
            self.push(v);
        }
    }
}

impl<T: Array> FromIterator<T::Item> for ArrayVec<T> {
    fn from_iter<I: IntoIterator<Item=T::Item>>(iter: I) -> Self {
        let mut s = Self::new();
        s.extend(iter);
        s
    }
}

impl<'a, T: Array> IntoIterator for &'a ArrayVec<T> {
    type Item = &'a T::Item;
    type IntoIter = slice::Iter<'a, T::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Array> IntoIterator for &'a mut ArrayVec<T> {
    type Item = &'a mut T::Item;
    type IntoIter = slice::IterMut<'a, T::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Array> Deref for ArrayVec<T> {
    type Target = [T::Item];

    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.array.as_ptr(), ArrayIndex::to_usize(self.len)) }
    }
}

impl<T: Array> DerefMut for ArrayVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.array.as_mut_ptr(), ArrayIndex::to_usize(self.len)) }
    }
}
