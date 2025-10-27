use super::*;
pub trait DivOrNop: Sized + Copy {
    fn div_or_nop(self, d: Self) -> Self;
}
macro_rules! impl_don {
    ($t:ty;$($tt:ty);+) => {
        impl_don! { $t } impl_don! { $($tt);+ }
    };
    ($t:ty) => {
        impl DivOrNop for $t { #[inline] fn div_or_nop(self, d : Self) -> Self { self
        .checked_div(d).unwrap_or(self) } }
    };
}
impl_don! {
    u8; i8; u16; i16; u32; i32; u64; i64; u128; i128; usize; isize
}
#[cfg(test)]
mod tests_rug_201 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        let mut p1: u8 = rug_fuzz_1;
        <u8>::div_or_nop(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_202 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i8 = rug_fuzz_0;
        let p1: i8 = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_203 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u16 = rug_fuzz_0;
        let mut p1: u16 = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_204 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i16 = rug_fuzz_0;
        let mut p1: i16 = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_205 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        u32::div_or_nop(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_206 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i32 = rug_fuzz_0;
        let p1: i32 = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_207 {
    use super::*;
    use crate::imp::numext::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u64 = rug_fuzz_0;
        let mut p1: u64 = rug_fuzz_1;
        <u64 as DivOrNop>::div_or_nop(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_208 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        let mut p1: i64 = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_209 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u128 = rug_fuzz_0;
        let mut p1: u128 = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_210 {
    use super::*;
    use crate::imp::numext::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i128 = rug_fuzz_0;
        let p1: i128 = rug_fuzz_1;
        debug_assert_eq!(< i128 as DivOrNop > ::div_or_nop(p0, p1), 5);
             }
});    }
}
#[cfg(test)]
mod tests_rug_211 {
    use super::*;
    use crate::imp::DivOrNop;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        <usize>::div_or_nop(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_212 {
    use super::*;
    use crate::imp::numext::DivOrNop;
    #[test]
    fn test_div_or_nop() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: isize = rug_fuzz_0;
        let mut p1: isize = rug_fuzz_1;
        p0.div_or_nop(p1);
             }
});    }
}
