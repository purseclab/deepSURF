use std::mem::transmute;
use std::ptr::{read, write, swap, copy};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use util::PointerExt;

pub trait Vector {
    type Item;

    #[inline]
    fn new() -> Self where Self: Sized {
        Self::with_capacity(0)
    }

    fn with_capacity(cap: usize) -> Self where Self: Sized;

    fn capacity(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
    fn into_boxed_slice(self) -> Box<[Self::Item]>;

    fn truncate(&mut self, len: usize) {
        let s_len = self.len();
        assert!(len <= s_len);
        let ptr = self.as_mut_ptr();

        unsafe {
            self.set_len(len);
            for i in len..s_len {
                read(ptr.uoffset(i));
            }
        }
    }

    unsafe fn set_len(&mut self, len: usize);

    fn swap_remove(&mut self, index: usize) -> Self::Item {
        let len = self.len();
        assert!(index < len);
        unsafe {
            let ptr = self.as_mut_ptr();
            let v = read(ptr.uoffset(index));
            if index != len - 1 {
                swap(ptr.uoffset(len - 1), ptr.uoffset(index));
            }
            self.set_len(len - 1);
            v
        }
    }

    fn insert(&mut self, index: usize, element: Self::Item) {
        self.reserve(1);
        unsafe {
            let len = self.len();
            let ptr = self.as_mut_ptr().uoffset(index);
            copy(ptr, ptr.uoffset(1), len - index);
            write(ptr, element);
            self.set_len(len + 1);
        }
    }

    fn remove(&mut self, index: usize) -> Self::Item {
        let len = self.len();
        assert!(index < len);
        unsafe {
            let ptr = self.as_mut_ptr().uoffset(index);
            self.set_len(len - 1);
            let v = read(ptr);
            copy(ptr.uoffset(1), ptr, len - index - 1);
            v
        }
    }

    fn retain<F: FnMut(&Self::Item) -> bool>(&mut self, mut f: F) where Self: Sized {
        let len = self.len();
        let mut del = 0;
        unsafe {
            let v = self.as_mut_ptr();

            for i in 0..len {
                if !f(transmute(v.uoffset(i) as *const _)) {
                    del += 1;
                } else if del > 0 {
                    swap(v.uoffset(i - del), v.uoffset(i));
                }
            }
        }
        if del > 0 {
            self.truncate(len - del);
        }
    }

    fn push(&mut self, value: Self::Item) {
        self.reserve(1);
        let len = self.len();
        unsafe {
            write(self.as_mut_ptr().uoffset(len), value);
            self.set_len(len + 1);
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            Some(self.swap_remove(len - 1))
        }
    }

    #[inline]
    fn clear(&mut self) { self.truncate(0) }

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool { self.len() == 0 }

    fn push_cap(&mut self, value: Self::Item) -> Result<(), Self::Item> {
        if self.len() < self.capacity() {
            self.push(value);
            Ok(())
        } else {
            Err(value)
        }
    }

    fn insert_cap(&mut self, index: usize, element: Self::Item) -> Option<Self::Item> {
        let cap = self.capacity();

        if index >= cap {
            return Some(element);
        }

        let ret = if self.len() < cap {
            None
        } else {
            self.pop()
        };
        self.insert(index, element);
        ret
    }

    fn as_ptr(&self) -> *const Self::Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;

    #[inline]
    fn as_slice(&self) -> &[Self::Item] {
        unsafe { from_raw_parts(self.as_ptr(), self.len()) }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        unsafe { from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }

    unsafe fn uninitialized_resize(&mut self, new_len: usize) {
        let len = self.len();
        if new_len > len {
            self.reserve_exact(new_len - len);
        }
        self.set_len(new_len);
    }
}

impl<T> Vector for Vec<T> {
    type Item = T;

    #[inline] fn new() -> Self { Vec::new() }
    #[inline] fn with_capacity(cap: usize) -> Self { Vec::with_capacity(cap) }
    #[inline] fn capacity(&self) -> usize { Vec::capacity(self) }
    #[inline] fn reserve(&mut self, additional: usize) { Vec::reserve(self, additional) }
    #[inline] fn reserve_exact(&mut self, additional: usize) { Vec::reserve_exact(self, additional) }
    #[inline] fn shrink_to_fit(&mut self) { Vec::shrink_to_fit(self) }
    #[inline] fn into_boxed_slice(self) -> Box<[T]> { Vec::into_boxed_slice(self) }
    #[inline] fn truncate(&mut self, len: usize) { Vec::truncate(self, len) }
    #[inline] unsafe fn set_len(&mut self, len: usize) { Vec::set_len(self, len) }
    #[inline] fn swap_remove(&mut self, index: usize) -> T { Vec::swap_remove(self, index) }
    #[inline] fn insert(&mut self, index: usize, element: T) { Vec::insert(self, index, element) }
    #[inline] fn remove(&mut self, index: usize) -> T { Vec::remove(self, index) }
    #[inline] fn retain<F: FnMut(&T) -> bool>(&mut self, f: F) { Vec::retain(self, f) }
    #[inline] fn push(&mut self, value: T) { Vec::push(self, value) }
    #[inline] fn pop(&mut self) -> Option<T> { Vec::pop(self) }
    #[inline] fn clear(&mut self) { Vec::clear(self) }
    #[inline] fn len(&self) -> usize { Vec::len(self) }
    #[inline] fn is_empty(&self) -> bool { Vec::is_empty(self) }
    #[inline] fn as_ptr(&self) -> *const T { self[..].as_ptr() }
    #[inline] fn as_mut_ptr(&mut self) -> *mut T { self[..].as_mut_ptr() }
    #[inline] fn as_slice(&self) -> &[T] { &self[..] }
    #[inline] fn as_mut_slice(&mut self) -> &mut [T] { &mut self[..] }
}
