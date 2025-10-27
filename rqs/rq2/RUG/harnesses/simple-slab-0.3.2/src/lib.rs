//! Fast and lightweight Slab Allocator.
extern crate libc;
use std::{mem, ptr};
use std::ops::{Drop, Index};
use std::iter::{Iterator, IntoIterator};
pub struct Slab<T> {
    capacity: usize,
    len: usize,
    mem: *mut T,
}
pub struct SlabIter<'a, T: 'a> {
    slab: &'a Slab<T>,
    current_offset: usize,
}
pub struct SlabMutIter<'a, T: 'a> {
    iter: SlabIter<'a, T>,
}
impl<T> Slab<T> {
    /// Creates a new Slab
    pub fn new() -> Slab<T> {
        Slab::with_capacity(1)
    }
    /// Creates a new, empty Slab with room for `capacity` elems
    ///
    /// # Panics
    ///
    /// * If the host system is out of memory
    pub fn with_capacity(capacity: usize) -> Slab<T> {
        let maybe_ptr = unsafe {
            libc::malloc((mem::size_of::<T>() * capacity)) as *mut T
        };
        if maybe_ptr.is_null() && capacity != 0 {
            panic!("Unable to allocate requested capacity")
        }
        return Slab {
            capacity: capacity,
            len: 0,
            mem: maybe_ptr,
        };
    }
    /// Inserts a new element into the slab, re-allocating if neccessary.
    ///
    /// # Panics
    ///
    /// * If the host system is out of memory.
    #[inline]
    pub fn insert(&mut self, elem: T) {
        if self.len == self.capacity {
            self.reallocate();
        }
        unsafe {
            let ptr = self.mem.offset(self.len as isize);
            ptr::write(ptr, elem);
        }
        self.len += 1;
    }
    /// Removes the element at `offset`.
    ///
    /// # Panics
    ///
    /// * If `offset` is out of bounds.
    #[inline]
    pub fn remove(&mut self, offset: usize) -> T {
        assert!(offset < self.len, "Offset out of bounds");
        let elem: T;
        let last_elem: T;
        let elem_ptr: *mut T;
        let last_elem_ptr: *mut T;
        unsafe {
            elem_ptr = self.mem.offset(offset as isize);
            last_elem_ptr = self.mem.offset(self.len as isize);
            elem = ptr::read(elem_ptr);
            last_elem = ptr::read(last_elem_ptr);
            ptr::write(elem_ptr, last_elem);
        }
        self.len -= 1;
        return elem;
    }
    /// Returns the number of elements in the slab
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns an iterator over the slab
    #[inline]
    pub fn iter(&self) -> SlabIter<T> {
        SlabIter {
            slab: self,
            current_offset: 0,
        }
    }
    /// Returns a mutable iterator over the slab
    #[inline]
    pub fn iter_mut(&mut self) -> SlabMutIter<T> {
        SlabMutIter { iter: self.iter() }
    }
    /// Reserves capacity * 2 extra space in this slab
    ///
    /// # Panics
    ///
    /// Panics if the host system is out of memory
    #[inline]
    fn reallocate(&mut self) {
        let new_capacity = if self.capacity != 0 { self.capacity * 2 } else { 1 };
        let maybe_ptr = unsafe {
            libc::realloc(
                self.mem as *mut libc::c_void,
                (mem::size_of::<T>() * new_capacity),
            ) as *mut T
        };
        assert!(! maybe_ptr.is_null(), "Out of Memory");
        self.capacity = new_capacity;
        self.mem = maybe_ptr;
    }
}
impl<T> Drop for Slab<T> {
    fn drop(&mut self) {
        for x in 0..self.len() {
            unsafe {
                let elem_ptr = self.mem.offset(x as isize);
                ptr::drop_in_place(elem_ptr);
            }
        }
        unsafe { libc::free(self.mem as *mut _ as *mut libc::c_void) };
    }
}
impl<T> Index<usize> for Slab<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &(*(self.mem.offset(index as isize))) }
    }
}
impl<'a, T> Iterator for SlabIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        while self.current_offset < self.slab.len() {
            let offset = self.current_offset;
            self.current_offset += 1;
            unsafe {
                return Some(&(*self.slab.mem.offset(offset as isize)));
            }
        }
        None
    }
}
impl<'a, T> Iterator for SlabMutIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        unsafe { mem::transmute(self.iter.next()) }
    }
}
impl<'a, T> IntoIterator for &'a Slab<T> {
    type Item = &'a T;
    type IntoIter = SlabIter<'a, T>;
    fn into_iter(self) -> SlabIter<'a, T> {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Slab<T> {
    type Item = &'a mut T;
    type IntoIter = SlabMutIter<'a, T>;
    fn into_iter(self) -> SlabMutIter<'a, T> {
        self.iter_mut()
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::Slab;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_new = 0;
        let slab: Slab<i32> = Slab::new();
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use crate::Slab;
    use std::ptr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Slab::<String> {
            mem: std::ptr::null_mut(),
            capacity: rug_fuzz_0,
            len: rug_fuzz_1,
        };
        let p1 = rug_fuzz_2.to_string();
        p0.insert(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::Slab;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut slab: Slab<i32> = Slab::new();
        let offset: usize = rug_fuzz_0;
        slab.remove(offset);
             }
});    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::Slab;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut slab: Slab<i32> = Slab::new();
        slab.insert(rug_fuzz_0);
        debug_assert_eq!(< Slab < i32 > > ::len(& slab), 1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::Slab;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_6_rrrruuuugggg_test_rug = 0;
        let mut p0: Slab<u32> = Slab::new();
        <Slab<u32>>::iter(&p0);
        let _rug_ed_tests_rug_6_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::Slab;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = 0;
        let mut slab: Slab<i32> = Slab::new();
        slab.iter_mut();
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    use crate::Slab;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_8_rrrruuuugggg_test_rug = 0;
        let mut p0: Slab<u32> = Slab::new();
        Slab::<u32>::reallocate(&mut p0);
        let _rug_ed_tests_rug_8_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::std::ops::Index;
    #[test]
    fn test_index() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut slab: Slab<i32> = Slab::new();
        let idx: usize = rug_fuzz_0;
        slab.index(idx);
             }
});    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::std::iter::Iterator;
    use crate::SlabIter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let slab = Slab::<i32>::new();
        let slab_iter = SlabIter {
            slab: &slab,
            current_offset: rug_fuzz_0,
        };
        let mut p0 = slab_iter;
        p0.next();
             }
});    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::std::iter::IntoIterator;
    use crate::Slab;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_13_rrrruuuugggg_test_rug = 0;
        let mut p0: Slab<u32> = Slab::new();
        p0.into_iter();
        let _rug_ed_tests_rug_13_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::std::iter::IntoIterator;
    use crate::{Slab, SlabMutIter};
    #[test]
    fn test_into_iter() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut slab: Slab<i32> = Slab::new();
        slab.insert(rug_fuzz_0);
        slab.insert(rug_fuzz_1);
        slab.into_iter();
             }
});    }
}
