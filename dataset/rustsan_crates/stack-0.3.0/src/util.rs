pub trait PointerExt {
    unsafe fn uoffset(self, off: usize) -> Self;
}

impl<T> PointerExt for *const T {
    #[inline]
    unsafe fn uoffset(self, off: usize) -> Self {
        self.offset(off as isize)
    }
}

impl<T> PointerExt for *mut T {
    #[inline]
    unsafe fn uoffset(self, off: usize) -> Self {
        self.offset(off as isize)
    }
}
