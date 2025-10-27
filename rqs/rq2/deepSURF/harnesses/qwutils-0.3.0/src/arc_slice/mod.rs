use super::*;
use std::{ops::{Deref,Range,DerefMut}, sync::Arc, borrow::{Borrow, BorrowMut}, fmt, hash::{self, Hash}, vec::IntoIter, slice, io::{self, Write}};
use hash::Hasher;
use io::IoSlice;

pub mod imp;

/// Slice backed by Arc<Vec<T>>
///
/// Cow mechanisms, minimizes clones
pub struct ArcSlice<T> {
    inner: Arc<Vec<T>>,
    slice: Range<usize>,
}

impl<T> Deref for ArcSlice<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        assert!(self.slice.end >= self.slice.start);
        assert!(self.slice.end <= self.inner.len());
        &self.inner[self.slice.clone()]
    }
}
impl<T> DerefMut for ArcSlice<T> where T: Clone {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(self.slice.end >= self.slice.start);
        assert!(self.slice.end <= self.inner.len());
        let (vec,range) = self._make_mut();
        &mut vec[range.clone()]
    }
}

impl<T> Clone for ArcSlice<T> {
    fn clone(&self) -> Self {
        self.refc()
    }
}

impl<T> RefClonable for ArcSlice<T> {
    fn refc(&self) -> Self {
        Self{
            inner: self.inner.refc(),
            slice: self.slice.clone(),
        }
    }
}

impl<T> From<&[T]> for ArcSlice<T> where T: Clone {
    fn from(v: &[T]) -> Self {
        Self::from(v.to_vec())
    }
}

impl<T> Into<Vec<T>> for &ArcSlice<T> where T: Clone {
    fn into(self) -> Vec<T> {
        self.extract()
    }
}
impl<T> Into<Vec<T>> for ArcSlice<T> where T: Clone {
    fn into(self) -> Vec<T> {
        self.extract()
    }
}
impl<T> Into<Vec<T>> for &mut ArcSlice<T> where T: Clone {
    fn into(self) -> Vec<T> {
        self.extract()
    }
}

impl<T> AsRef<[T]> for ArcSlice<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &**self
    }
}
impl<T> AsMut<[T]> for ArcSlice<T> where T: Clone {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut **self
    }
}

impl<T> Borrow<[T]> for ArcSlice<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        &**self
    }
}
impl<T> BorrowMut<[T]> for ArcSlice<T> where T: Clone {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut **self
    }
}

impl<T> fmt::Debug for ArcSlice<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T,O> PartialEq<O> for ArcSlice<T> where T: PartialEq, O: AsRef<[T]> {
    fn eq(&self, other: &O) -> bool {
        **self == *other.as_ref()
    }
}
impl<T> Eq for ArcSlice<T> where T: Eq {}

impl<T> PartialOrd for ArcSlice<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&**self,&**other)
    }
}
impl<T> Ord for ArcSlice<T> where T: Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&**self,&**other)
    }
}

impl<T> Extend<T> for ArcSlice<T> where T: Clone {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = T> {
        let iter = iter.into_iter();
        let reserve = {
            let (mut lower,upper) = iter.size_hint();
            if let Some(upper) = upper {
                lower = lower.max(upper);
            }
            lower
        };
        let (vec,slice) = self._make_mut_with_capacity(self.len() + reserve);
        vec.truncate(slice.end);
        let reserved = vec.capacity() - slice.end;
        if reserve > reserved {
            vec.reserve(reserve-reserved);
        }
        for i in iter {
            vec.push(i);
            slice.end += 1;
        }
    }
}
impl<'a,T> Extend<&'a T> for ArcSlice<T> where T: Clone + 'a {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = &'a T> {
        let iter = iter.into_iter();
        let reserve = {
            let (mut lower,upper) = iter.size_hint();
            if let Some(upper) = upper {
                lower = lower.max(upper);
            }
            lower
        };
        let (vec,slice) = self._make_mut_with_capacity(self.len() + reserve);
        vec.truncate(slice.end);
        let reserved = vec.capacity() - slice.end;
        if reserve > reserved {
            vec.reserve(reserve-reserved);
        }
        for i in iter {
            vec.push(i.clone());
            slice.end += 1;
        }
    }
}

impl<T> Hash for ArcSlice<T> where T: Hash {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        Hash::hash(&**self, state)
    }
}

impl<T> IntoIterator for ArcSlice<T> where T: Clone {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        let slice = self.slice;
        match Arc::try_unwrap(self.inner) {
            Ok(mut v) => {
                v.truncate(slice.end);
                let mut iter = v.into_iter();
                for _ in 0..slice.start {
                    assert!(iter.next().is_some());
                }
                iter
            }
            Err(v) => {
                let v = v[slice].to_vec();
                v.into_iter()
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a ArcSlice<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self[..].iter()
    }
}
impl<'a, T> IntoIterator for &'a mut ArcSlice<T> where T: Clone {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self[..].iter_mut()
    }
}

impl Write for ArcSlice<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let len = bufs.iter().map(|b| b.len()).sum();
        let (vec,slice) = self._make_mut_with_capacity(self.len() + len);
        vec.truncate(slice.end);
        vec.reserve(len);
        for buf in bufs {
            vec.extend_from_slice(buf);
        }
        slice.end += len;
        Ok(len)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write(buf)?;
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
