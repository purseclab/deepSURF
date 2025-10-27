use std::mem::uninitialized;
use std::ops::{Index, IndexMut, Deref, DerefMut, Range, RangeFrom, RangeTo, RangeFull};
use std::hash::Hash;
use std::fmt::Debug;

pub unsafe trait Array {
    type Item;
    type Index: ArrayIndex;

    fn len() -> usize;
    fn as_ptr(&self) -> *const Self::Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
    unsafe fn uninitialized() -> Self where Self: Sized { uninitialized() }
}

pub trait ArrayIndex: Sized + Copy + Clone + Default + Hash + Debug + PartialEq + Eq + PartialOrd + Ord {
    fn to_usize(self) -> usize;
    fn from_usize(u: usize) -> Self;
}

impl ArrayIndex for u8 {
    #[inline] fn to_usize(self) -> usize { self as usize }
    #[inline] fn from_usize(u: usize) -> Self { u as Self }
}

impl ArrayIndex for u16 {
    #[inline] fn to_usize(self) -> usize { self as usize }
    #[inline] fn from_usize(u: usize) -> Self { u as Self }
}

impl ArrayIndex for u32 {
    #[inline] fn to_usize(self) -> usize { self as usize }
    #[inline] fn from_usize(u: usize) -> Self { u as Self }
}

pub trait Addressable<T>:
    Index<usize, Output=T> +
    IndexMut<usize> +
    Index<Range<usize>, Output=[<Self as Index<usize>>::Output]> +
    IndexMut<Range<usize>> +
    Index<RangeFrom<usize>, Output=[<Self as Index<usize>>::Output]> +
    IndexMut<RangeFrom<usize>> +
    Index<RangeTo<usize>, Output=[<Self as Index<usize>>::Output]> +
    IndexMut<RangeTo<usize>> +
    Index<RangeFull, Output=[<Self as Index<usize>>::Output]> +
    IndexMut<RangeFull> +
    Deref<Target=[<Self as Index<usize>>::Output]> +
    DerefMut
    where <Self as Index<usize>>::Output: Sized
{
}

impl<T, U> Addressable<U> for T where T:
    Index<usize, Output=U> +
    IndexMut<usize> +
    Index<Range<usize>, Output=[<T as Index<usize>>::Output]> +
    IndexMut<Range<usize>> +
    Index<RangeFrom<usize>, Output=[<T as Index<usize>>::Output]> +
    IndexMut<RangeFrom<usize>> +
    Index<RangeTo<usize>, Output=[<T as Index<usize>>::Output]> +
    IndexMut<RangeTo<usize>> +
    Index<RangeFull, Output=[<T as Index<usize>>::Output]> +
    IndexMut<RangeFull> +
    Deref<Target=[<T as Index<usize>>::Output]> +
    DerefMut,
    <T as Index<usize>>::Output: Sized
{
}

macro_rules! array_impl {
    ($i:ty => $t:expr) => {
        unsafe impl<T> Array for [T; $t] {
            type Item = T;
            type Index = $i;

            fn len() -> usize { $t }
            fn as_ptr(&self) -> *const Self::Item { self[..].as_ptr() }
            fn as_mut_ptr(&mut self) -> *mut Self::Item { self[..].as_mut_ptr() }
        }
    };
    ($i:ty => $($t:expr),+) => {
        $(
            array_impl!($i => $t);
        )+
    };
}

array_impl!(u8 =>
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    0x40, 0x80
);

array_impl!(u16 =>
    0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000
);

array_impl!(u32 =>
    0x10000, 0x20000, 0x40000, 0x80000,
    0x100000, 0x200000, 0x400000, 0x800000,
    0x1000000
);
