//! Provides two types, `CSemiBox` and `DisposeRef`
extern crate libc;
use libc::{free, c_char, c_void, size_t};
use std::borrow::Borrow;
use std::ffi::{CString, CStr};
use std::{fmt, mem, str};
use std::ops::{Deref, DerefMut, Drop};
use std::cmp::PartialEq;
use std::marker::PhantomData;
/// Implemented by any type of which its reference represents a C pointer that can be disposed.
pub trait DisposeRef {
    /// What a reference to this type represents as a C pointer.
    type RefTo;
    /// Destroy the contents at the pointer's location.
    ///
    /// This should run some variant of `libc::free(ptr)`
    unsafe fn dispose(ptr: *mut Self::RefTo) {
        free(ptr as *mut c_void);
    }
}
/// A wrapper for pointers made by C that are now partially owned in Rust.
///
/// This is necessary to allow owned and borrowed representations of C types
/// to be represented by the same type as they are in C with little overhead
pub struct CSemiBox<'a, D: ?Sized>
where
    D: DisposeRef + 'a,
{
    ptr: *mut D::RefTo,
    marker: PhantomData<&'a ()>,
}
impl<'a, D: ?Sized> CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
{
    #[inline(always)]
    /// Wrap the pointer in a `CSemiBox`
    pub fn new(ptr: *mut D::RefTo) -> Self {
        CSemiBox {
            ptr: ptr,
            marker: PhantomData,
        }
    }
    #[inline(always)]
    /// Returns the internal pointer
    pub unsafe fn as_ptr(&self) -> *mut D::RefTo {
        self.ptr
    }
    #[inline(always)]
    /// Returns the internal pointer
    pub unsafe fn unwrap(self) -> *mut D::RefTo {
        let ptr = self.ptr;
        mem::forget(self);
        ptr
    }
}
impl<'a, D: ?Sized> From<*mut D::RefTo> for CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
{
    #[inline(always)]
    fn from(ptr: *mut D::RefTo) -> Self {
        CSemiBox::new(ptr)
    }
}
impl<'a, D: ?Sized> Drop for CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
{
    #[inline(always)]
    /// Run the destructor
    fn drop(&mut self) {
        unsafe { <D as DisposeRef>::dispose(self.ptr) }
    }
}
impl<'a, D> Deref for CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
    *mut D::RefTo: Into<&'a D>,
{
    type Target = D;
    fn deref(&self) -> &D {
        self.ptr.into()
    }
}
impl<'a, D> Borrow<D> for CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
    *mut D::RefTo: Into<&'a D>,
{
    fn borrow(&self) -> &D {
        self.ptr.into()
    }
}
impl<'a, D> DerefMut for CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
    *mut D::RefTo: Into<&'a D>,
    *mut D::RefTo: Into<&'a mut D>,
{
    fn deref_mut(&mut self) -> &mut D {
        self.ptr.into()
    }
}
impl<'a, T> fmt::Display for CSemiBox<'a, T>
where
    T: fmt::Display + DisposeRef + 'a,
    *mut T::RefTo: Into<&'a T>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self as &T, fmt)
    }
}
impl<'a, T> fmt::Debug for CSemiBox<'a, T>
where
    T: fmt::Debug + DisposeRef + 'a,
    *mut T::RefTo: Into<&'a T>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self as &T, fmt)
    }
}
impl<'a, T> PartialEq<T> for CSemiBox<'a, T>
where
    T: 'a + DisposeRef + PartialEq,
    *mut T::RefTo: Into<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        (self as &T).eq(other)
    }
}
impl<'a> From<&'a CStr> for CSemiBox<'a, str> {
    fn from(text: &'a CStr) -> CSemiBox<'a, str> {
        CSemiBox::new(text.as_ptr() as *mut c_char)
    }
}
impl DisposeRef for str {
    type RefTo = c_char;
}
/// A wrapper for pointers made by C that are now completely owned by Rust, so
/// they are not limited by any lifetimes.
///
/// This is necessary to allow owned and borrowed representations of C types
/// to be represented by the same type as they are in C with little overhead.
pub struct CBox<D: ?Sized>
where
    D: DisposeRef,
{
    ptr: *mut D::RefTo,
}
impl<D: ?Sized> CBox<D>
where
    D: DisposeRef,
{
    #[inline(always)]
    /// Wrap the pointer in a `CBox`.
    pub fn new(ptr: *mut D::RefTo) -> Self {
        CBox { ptr: ptr }
    }
    #[inline(always)]
    /// Returns the internal pointer.
    pub unsafe fn as_ptr(&self) -> *mut D::RefTo {
        self.ptr
    }
    #[inline(always)]
    /// Returns the internal pointer.
    pub unsafe fn unwrap(self) -> *mut D::RefTo {
        let ptr = self.ptr;
        mem::forget(self);
        ptr
    }
    /// Returns the box as a 'CSemiBox'.
    pub fn as_semi<'a>(&'a self) -> &CSemiBox<'a, D> {
        unsafe { mem::transmute(self) }
    }
    /// Returns the box as a 'CSemiBox'.
    pub fn as_semi_mut<'a>(&'a mut self) -> &mut CSemiBox<'a, D> {
        unsafe { mem::transmute(self) }
    }
}
impl<'a> From<&'a str> for CBox<str> {
    /// Copy this text using malloc and strcpy.
    fn from(text: &'a str) -> CBox<str> {
        unsafe {
            let cstr = CString::new(text).unwrap();
            let ptr = libc::malloc(text.len() as size_t + 1) as *mut c_char;
            libc::strcpy(ptr, cstr.as_ptr());
            CBox::new(ptr)
        }
    }
}
impl<'a> Deref for CBox<str> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            let text = CStr::from_ptr(self.ptr);
            str::from_utf8_unchecked(text.to_bytes())
        }
    }
}
impl Clone for CBox<str> {
    fn clone(&self) -> CBox<str> {
        unsafe {
            let ptr = libc::malloc(self.len() as size_t + 1) as *mut c_char;
            libc::strcpy(ptr, self.ptr);
            CBox::new(ptr)
        }
    }
}
impl fmt::Display for CBox<str> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.deref())
    }
}
impl fmt::Debug for CBox<str> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.deref())
    }
}
impl<T> Deref for CBox<T>
where
    T: DisposeRef,
{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { mem::transmute(self.ptr) }
    }
}
impl<T> Borrow<T> for CBox<T>
where
    T: DisposeRef,
{
    fn borrow(&self) -> &T {
        unsafe { mem::transmute(self.ptr) }
    }
}
impl<T> DerefMut for CBox<T>
where
    T: DisposeRef,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { mem::transmute(self.ptr) }
    }
}
impl<'a, T> PartialEq<T> for CBox<T>
where
    T: 'a + DisposeRef + PartialEq,
    *mut T::RefTo: Into<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        unsafe { mem::transmute::<_, &T>(self.ptr) == other }
    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::std::ops::Deref;
    struct CSemiBox<'a, D>(std::ptr::NonNull<D>, std::marker::PhantomData<&'a D>);
    impl<'a, D> CSemiBox<'a, D> {
        fn as_ptr(&self) -> *const D {
            self.0.as_ptr()
        }
    }
    impl<'a, D> std::ops::Deref for CSemiBox<'a, D> {
        type Target = D;
        fn deref(&self) -> &D {
            unsafe { &*self.0.as_ptr() }
        }
    }
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = 0;
        let mut p0 = CSemiBox(
            std::ptr::NonNull::<i32>::dangling(),
            std::marker::PhantomData,
        );
        p0.deref();
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::std::convert::From;
    use std::ffi::CStr;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_11_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"hello\0";
        let text = CStr::from_bytes_with_nul(rug_fuzz_0).unwrap();
        <CSemiBox<'static, str>>::from(text);
        let _rug_ed_tests_rug_11_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::std::ops::Deref;
    use crate::{CBox, CStr, str};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ptr = rug_fuzz_0.as_ptr() as *mut i8;
        let cbox_str = CBox::<str> { ptr };
        cbox_str.deref();
             }
});    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use crate::std::ops::Deref;
    struct CBox<T> {
        ptr: *const T,
    }
    impl<T> Deref for CBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            unsafe { std::mem::transmute(self.ptr) }
        }
    }
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: i32 = rug_fuzz_0;
        let p0 = CBox { ptr: &value };
        <CBox<i32> as std::ops::Deref>::deref(&p0);
             }
});    }
}
