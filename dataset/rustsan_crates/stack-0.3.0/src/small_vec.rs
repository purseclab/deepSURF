use std::fmt;
use std::mem::replace;
use std::ptr::copy_nonoverlapping;
use std::ops::{Deref, DerefMut};
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};
use unreachable::unreachable;
use coalesce::Coalesce2;
use array_vec::ArrayVec;
use vector::Vector;
use array::Array;

#[path = "index_impls.rs"]
mod index_impls;

pub trait Spilled<T> {
    fn spill(v: T) -> Self;
}

impl<T: Array> Spilled<ArrayVec<T>> for Vec<T::Item> {
    fn spill(mut v: ArrayVec<T>) -> Self {
        let len = v.len();

        let mut s = Self::with_capacity(len);
        unsafe {
            v.set_len(0);
            copy_nonoverlapping(v.as_ptr(), s.as_mut_ptr(), len);
            s.set_len(len);
        }
        s
    }
}

pub struct SmallVec<T: Array, S = Vec<<T as Array>::Item>>(Coalesce2<ArrayVec<T>, S>);

impl<T: Array, S: Clone> Clone for SmallVec<T, S> where ArrayVec<T>: Clone {
    fn clone(&self) -> Self {
        SmallVec(self.0.clone())
    }
}

impl<T: Array, S: Vector + Spilled<ArrayVec<T>>> SmallVec<T, S> {
    #[inline]
    pub fn spill(&mut self) {
        if !self.is_spilled() {
            match replace(&mut self.0, Coalesce2::B(S::new())) {
                Coalesce2::A(v) => { replace(&mut self.0, Coalesce2::B(S::spill(v))); },
                _ => unsafe { unreachable() },
            }
        }
    }

    #[inline]
    pub fn is_spilled(&self) -> bool {
        match self.0.as_ref() {
            Coalesce2::A(..) => false,
            Coalesce2::B(..) => true,
        }
    }
}

impl<T: Array, S> SmallVec<T, S> {
    #[inline]
    pub fn into_inner(self) -> Coalesce2<ArrayVec<T>, S> {
        self.0
    }
}

impl<T: Array, S> From<ArrayVec<T>> for SmallVec<T, S> {
    fn from(v: ArrayVec<T>) -> Self {
        SmallVec(Coalesce2::A(v))
    }
}

impl<T: Array, S: Vector<Item=T::Item> + Spilled<ArrayVec<T>>> Vector for SmallVec<T, S> {
    type Item = T::Item;

    #[inline]
    fn with_capacity(cap: usize) -> Self {
        SmallVec(if cap > T::len() {
            Coalesce2::B(S::with_capacity(cap))
        } else {
            Coalesce2::A(Default::default())
        })
    }

    #[inline]
    fn capacity(&self) -> usize {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v.capacity())
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        let cap = self.capacity() - self.len();
        if cap < additional {
            self.spill();
        } else if cap >= additional {
            return
        }

        let v = self.0.as_mut();
        coalesce!(2 => |v| v.reserve(additional))
    }

    #[inline]
    fn reserve_exact(&mut self, additional: usize) {
        let cap = self.capacity() - self.len();
        if cap < additional {
            self.spill();
        } else if cap > additional {
            return
        }

        let v = self.0.as_mut();
        coalesce!(2 => |v| v.reserve_exact(additional))
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let v = self.0.as_mut();
        coalesce!(2 => |v| v.shrink_to_fit())
    }

    #[inline]
    fn into_boxed_slice(self) -> Box<[T::Item]> {
        let v = self.into_inner();
        coalesce!(2 => |v| v.into_boxed_slice())
    }

    #[inline]
    unsafe fn set_len(&mut self, len: usize) {
        let v = self.0.as_mut();
        coalesce!(2 => |v| v.set_len(len))
    }

    #[inline]
    fn len(&self) -> usize {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v.len())
    }

    #[inline]
    fn as_ptr(&self) -> *const T::Item {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v.as_ptr())
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T::Item {
        let v = self.0.as_mut();
        coalesce!(2 => |v| v.as_mut_ptr())
    }
}

impl<T: Array, S: Extend<T::Item>> Extend<T::Item> for SmallVec<T, S> where SmallVec<T, S>: Vector {
    fn extend<I: IntoIterator<Item=T::Item>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        let v = self.0.as_mut();
        coalesce!(2 => |v| v.extend(iter))
    }
}

impl<T: Array, S: Extend<T::Item>> FromIterator<T::Item> for SmallVec<T, S> where SmallVec<T, S>: Vector {
    fn from_iter<I: IntoIterator<Item=T::Item>>(iter: I) -> Self {
        let mut s = Self::new();
        s.extend(iter);
        s
    }
}

pub struct SmallVecIter<T1, T2>(Coalesce2<T1, T2>);

impl<T1: Iterator, T2: Iterator<Item=T1::Item>> Iterator for SmallVecIter<T1, T2> {
    type Item = T1::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.0.as_mut();
        coalesce!(2 => |v| v.next())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v.size_hint())
    }

    #[inline]
    fn count(self) -> usize {
        let v = self.0;
        coalesce!(2 => |v| v.count())
    }
}

impl<T: Array, S: IntoIterator<Item=<ArrayVec<T> as IntoIterator>::Item>> IntoIterator for SmallVec<T, S> where ArrayVec<T>: IntoIterator {
    type Item = <S as IntoIterator>::Item;
    type IntoIter = SmallVecIter<<ArrayVec<T> as IntoIterator>::IntoIter, <S as IntoIterator>::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SmallVecIter(match self.0 {
            Coalesce2::A(v) => Coalesce2::A(v.into_iter()),
            Coalesce2::B(v) => Coalesce2::B(v.into_iter()),
        })
    }
}

impl<T: Array, S: Deref> Deref for SmallVec<T, S> where ArrayVec<T>: Deref<Target=S::Target>  {
    type Target = S::Target;

    fn deref(&self) -> &Self::Target {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v.deref())
    }
}

impl<T: Array, S: DerefMut> DerefMut for SmallVec<T, S> where ArrayVec<T>: DerefMut + Deref<Target=S::Target> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let v = self.0.as_mut();
        coalesce!(2 => |v| v.deref_mut())
    }
}

impl<T: Array, S> Default for SmallVec<T, S> {
    fn default() -> Self {
        SmallVec(Coalesce2::A(Default::default()))
    }
}

impl<T: Array, S: Hash> Hash for SmallVec<T, S> where ArrayVec<T>: Hash {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v.hash(h))
    }
}

impl<T: Array, S: fmt::Debug + Vector<Item=T::Item> + Spilled<ArrayVec<T>>> fmt::Debug for SmallVec<T, S> where ArrayVec<T>: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let v = self.0.as_ref();
        fmt.debug_struct("SmallVec")
            .field("is_spilled", &self.is_spilled())
            .field("capacity", &self.capacity())
            .field("data", &coalesce!(2 => |v| v as &fmt::Debug))
            .finish()
    }
}

impl<'a, T: Array + 'a, S: Vector<Item=T::Item> + 'a> AsRef<Vector<Item=T::Item> + 'a> for SmallVec<T, S> where T::Item: 'a, T::Index: 'a {
    fn as_ref(&self) -> &(Vector<Item=T::Item> + 'a) {
        let v = self.0.as_ref();
        coalesce!(2 => |v| v as &_)
    }
}
