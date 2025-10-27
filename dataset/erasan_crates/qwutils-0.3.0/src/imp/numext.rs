use super::*;

pub trait DivOrNop: Sized + Copy {
    fn div_or_nop(self, d: Self) -> Self;
}

macro_rules! impl_don {
    ($t:ty;$($tt:ty);+) => {
        impl_don!{$t}
        impl_don!{$($tt);+}
    };
    ($t:ty) => {
        impl DivOrNop for $t {
            #[inline]
            fn div_or_nop(self, d: Self) -> Self {
                self.checked_div(d).unwrap_or(self)
            }
        }
    };
}

impl_don!{
    u8;i8;
    u16;i16;
    u32;i32;
    u64;i64;
    u128;i128;
    usize;isize
}