use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr::{copy_nonoverlapping};
use std::mem::{uninitialized, transmute, forget, size_of, align_of};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::marker;
use ::{ArrayVec, Vector};

#[macro_export]
macro_rules! small_dst {
    ($x:expr) => {
        {
            let v = $x;
            let dst = unsafe { $crate::SmallDST::new_ref(&v as &_, &v) };
            ::std::mem::forget(v);
            dst
        }
    };
}

pub struct SmallDST<T: ?Sized, A: Vector<Item=usize> = ArrayVec<[usize; 9]>> {
    data: A,
    _phantom: marker::PhantomData<T>,
}

impl<T: ?Sized + Clone, A: Vector<Item=usize>> Clone for SmallDST<T, A> {
    fn clone(&self) -> Self {
        small_dst!((&**self).clone())
    }
}

impl<T: ?Sized, A: Vector<Item=usize>> SmallDST<T, A> {
    fn ptr_len() -> usize {
        assert_eq!(size_of::<*const T>() % size_of::<usize>(), 0);
        size_of::<*const T>() / size_of::<usize>()
    }

    fn data_len() -> usize {
        Self::ptr_len() - 1
    }

    fn ref_data<'a>(v: &'a *const T) -> &'a [usize] {
        unsafe { from_raw_parts(v as *const *const T as *const usize, Self::ptr_len()) }
    }

    fn mut_data<'a>(v: &'a mut *const T) -> &'a mut [usize] {
        unsafe { from_raw_parts_mut(v as *mut *const T as *mut usize, Self::ptr_len()) }
    }

    #[cfg(feature = "unstable")]
    pub fn new<U: marker::Unsize<T>>(v: U) -> Self {
        let ret = unsafe { Self::new_ref(&v, &v) };
        forget(v);
        ret
    }

    #[inline]
    pub unsafe fn new_ref<U>(v: &T, u: &U) -> Self {
        let t = v as *const T;
        let data = Self::ref_data(&t);
        assert_eq!(data[0], u as *const U as usize);

        Self::from_ref(u, &data[1..])
    }

    #[inline]
    pub unsafe fn data(&self) -> &A {
        &self.data
    }

    unsafe fn from_value<U>(v: U, data: &[usize]) -> Self {
        let ret = Self::from_ref(&v, data);
        forget(v);
        ret
    }

    unsafe fn from_ref<U>(v: &U, data: &[usize]) -> Self {
        assert!(align_of::<U>() <= align_of::<A::Item>()); // TODO: support arbitrary alignments
        assert_eq!(data.len(), Self::data_len());

        let size = (size_of::<U>() + size_of::<usize>() - 1) / size_of::<usize>();

        let mut a = A::with_capacity(Self::data_len() + size);
        a.set_len(Self::data_len() + size);
        copy_nonoverlapping(data.as_ptr(), a.as_mut_ptr(), Self::data_len());
        copy_nonoverlapping(v as *const U as *const u8, a.as_mut_ptr().offset(Self::data_len() as isize) as *mut u8, size_of::<U>());

        SmallDST {
            data: a,
            _phantom: marker::PhantomData,
        }
    }
}

impl<T: ?Sized, A: Vector<Item=usize>> Drop for SmallDST<T, A> {
    #[inline]
    fn drop(&mut self) {
        use std::ptr;

        unsafe {
            if self.data.len() > 0 {
                self.data.set_len(0);
                ptr::drop_in_place(&mut **self);
            }
        }
    }
}

impl<T: Sized, A: Vector<Item=usize>> From<T> for SmallDST<T, A> {
    #[inline]
    fn from(t: T) -> Self {
        unsafe { Self::from_value(t, &[]) }
    }
}

impl<T: ?Sized, A: Vector<Item=usize>> Deref for SmallDST<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            let mut v: *const T = uninitialized();
            {
                let v = Self::mut_data(&mut v);
                v[0] = self.data.as_ptr().offset(Self::data_len() as isize) as usize;
                copy_nonoverlapping(self.data.as_ptr(), v[1..].as_mut_ptr(), Self::data_len());
            }
            transmute(v)
        }
    }
}

impl<T: ?Sized, A: Vector<Item=usize>> DerefMut for SmallDST<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let mut v: *const T = uninitialized();
            {
                let v = Self::mut_data(&mut v);
                v[0] = self.data.as_ptr().offset(Self::data_len() as isize) as usize;
                copy_nonoverlapping(self.data.as_ptr(), v[1..].as_mut_ptr(), Self::data_len());
            }
            transmute(v as *mut T)
        }
    }
}

impl<T: ?Sized + fmt::Debug, A: Vector<Item=usize>> fmt::Debug for SmallDST<T, A> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, fmt)
    }
}
