use super::*;
use std::{slice, io::{self, IoSlice}, hash::{Hasher, Hash}, vec::IntoIter, fmt};
use io::Write;

pub struct PartVec<T> {
    inner: Vec<Vec<T>>,
    factor: usize,
}
pub struct RopeVec<T> {
    inner: Vec<(usize,Vec<T>)>,
}

impl<T> Clone for PartVec<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}


impl<T> From<&[T]> for PartVec<T> where T: Clone {
    fn from(v: &[T]) -> Self {
        todo!()
    }
}

impl<T> Into<Vec<T>> for &PartVec<T> where T: Clone {
    fn into(self) -> Vec<T> {
        todo!()
    }
}
impl<T> Into<Vec<T>> for PartVec<T> where T: Clone {
    fn into(self) -> Vec<T> {
        todo!()
    }
}
impl<T> Into<Vec<T>> for &mut PartVec<T> where T: Clone {
    fn into(self) -> Vec<T> {
        todo!()
    }
}

impl<T> fmt::Debug for PartVec<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<T,O> PartialEq<O> for PartVec<T> where T: PartialEq, O: AsRef<[T]> {
    fn eq(&self, other: &O) -> bool {
        todo!()
    }
}
//impl<T> Eq for PartVec<T> where T: Eq {}

impl<T> PartialOrd for PartVec<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
impl<T> Ord for PartVec<T> where T: Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

impl<T> Extend<T> for PartVec<T> where T: Clone {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = T> {
        todo!()
    }
}
impl<'a,T> Extend<&'a T> for PartVec<T> where T: Clone + 'a {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = &'a T> {
        todo!()
    }
}

impl<T> Hash for PartVec<T> where T: Hash {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        todo!()
    }
}

impl<T> IntoIterator for PartVec<T> where T: Clone {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a PartVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        todo!()
    }
}
impl<'a, T> IntoIterator for &'a mut PartVec<T> where T: Clone {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        todo!()
    }
}

impl Write for PartVec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!();
        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        todo!();
        Ok(0)
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
