// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


//! Fast and lightweight Slab Allocator.


extern crate libc;


use std::{mem, ptr};
use std::ops::{Drop, Index};
use std::iter::{Iterator, IntoIterator};


pub struct Slab<T> {
    capacity: usize,
    len: usize,
    mem: *mut T
}

pub struct SlabIter<'a, T: 'a> {
    slab: &'a Slab<T>,
    current_offset: usize
}

pub struct SlabMutIter<'a, T: 'a> {
    iter: SlabIter<'a, T>
}

impl<T> Slab<T> {
    /// Creates a new Slab
    pub fn new() -> Slab<T> { Slab::with_capacity(1) }

    /// Creates a new, empty Slab with room for `capacity` elems
    ///
    /// # Panics
    ///
    /// * If the host system is out of memory
    pub fn with_capacity(capacity: usize) -> Slab<T> {
        let maybe_ptr = unsafe {
            libc::malloc((mem::size_of::<T>() * capacity)) as *mut T
        };

        // malloc will return NULL if called with zero
        if maybe_ptr.is_null() && capacity != 0 {
            panic!("Unable to allocate requested capacity")
        }

        return Slab {
            capacity: capacity,
            len: 0,
            mem: maybe_ptr
        }
    }

    /// Inserts a new element into the slab, re-allocating if neccessary.
    ///
    /// # Panics
    ///
    /// * If the host system is out of memory.
    #[inline]
    pub fn insert(&mut self, elem: T) {
        if self.len == self.capacity { self.reallocate(); }

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
    pub fn len(&self) -> usize { self.len }

    /// Returns an iterator over the slab
    #[inline]
    pub fn iter(&self) -> SlabIter<T> {
        SlabIter {
            slab: self,
            current_offset: 0
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
        let new_capacity = if self.capacity != 0 {
            self.capacity * 2
        } else { 1 };

        let maybe_ptr = unsafe {
            libc::realloc(self.mem as *mut libc::c_void,
                          (mem::size_of::<T>() * new_capacity)) as *mut T
        };

        assert!(!maybe_ptr.is_null(), "Out of Memory");

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
        //println!("Here: len: {:?}, index: {:?}", self.len(), index);
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
    fn into_iter(self) -> SlabIter<'a, T> { self.iter() }
}

impl<'a, T> IntoIterator for &'a mut Slab<T> {
    type Item = &'a mut T;
    type IntoIter = SlabMutIter<'a, T>;
    fn into_iter(self) -> SlabMutIter<'a, T> { self.iter_mut() }
}
