/*! Implement `Endian` on mutable slices.
!*/
use super::Endian;
use std::ptr;
/// Traverse a slice, performing the `Endian` method on each item in place.
impl<'a, T: Endian> Endian for &'a mut [T] {
    fn from_be(self) -> Self {
        for elt in self.iter_mut() {
            unsafe {
                ptr::write(elt, ptr::read(elt).from_be());
            }
        }
        self
    }
    fn from_le(self) -> Self {
        for elt in self.iter_mut() {
            unsafe {
                ptr::write(elt, ptr::read(elt).from_le());
            }
        }
        self
    }
    fn to_be(self) -> Self {
        for elt in self.iter_mut() {
            unsafe {
                ptr::write(elt, ptr::read(elt).to_be());
            }
        }
        self
    }
    fn to_le(self) -> Self {
        for elt in self.iter_mut() {
            unsafe {
                ptr::write(elt, ptr::read(elt).to_le());
            }
        }
        self
    }
}
#[cfg(test)]
mod tests_rug_57 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut [i32] = &mut [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        <&mut [i32]>::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_58 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut [u32] = &mut [rug_fuzz_0, rug_fuzz_1];
        p0 = <&mut [u32]>::from_le(p0);
        debug_assert_eq!(p0, & mut [67305985, 3704098005]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_59 {
    use super::*;
    use crate::Endian;
    use std::ptr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut [u16] = &mut [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.to_be();
        debug_assert_eq!(p0, & [256, 512, 768, 1024]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut [u8] = &mut [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        <&mut [u8]>::to_le(p0);
        let expected_result: &[u8] = &[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        debug_assert_eq!(p0, expected_result);
             }
});    }
}
