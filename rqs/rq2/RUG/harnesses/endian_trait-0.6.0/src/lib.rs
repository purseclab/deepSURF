/*! Endian conversion trait

This trait declares methods that perform endian conversions on data types. For
the primitives, which are essentially atomic in structure, this conversion is
simple: flip all their bytes around. This conversion is also defined as inherent
methods on the integral primitives, so `Endian::from_be(n: i32)` is equivalent
to `::std::i32::from_be(n: i32)`
!*/
#![deny(missing_docs)]
#[allow(unused_imports)]
#[macro_use]
extern crate endian_trait_derive;
pub use endian_trait_derive::*;
/// Convert a type from one endian order to another.
///
/// The standard implementation of this trait is simply to call the methods on
/// the component members of a data type which are themselves `Endian`, until the
/// call stack bottoms out at one of Rust's primitives.
pub trait Endian {
    /// Converts from host endian to big-endian order.
    ///
    /// On big-endian platforms, this is a no-op and should be compiled out.
    fn to_be(self) -> Self;
    /// Converts from host endian to little-endian order.
    ///
    /// On little-endian platforms, this is a no-op and should be compiled out.
    fn to_le(self) -> Self;
    /// Converts from big-endian order to host endian.
    ///
    /// On big-endian platforms, this is a no-op and should be compiled out.
    fn from_be(self) -> Self;
    /// Converts from little-endian order to host endian.
    ///
    /// On little-endian platforms, this is a no-op and should be compiled out.
    fn from_le(self) -> Self;
}
/// Implementing Endian on the integer primitives just means delegating to their
/// inherent methods. As there are many integer primitives, this macro kills the
/// needless code duplication.
macro_rules! implendian {
    ($($t:tt),*) => {
        $(impl Endian for $t { #[inline(always)] fn from_be(self) -> Self { $t
        ::from_be(self) } #[inline(always)] fn from_le(self) -> Self { $t ::from_le(self)
        } #[inline(always)] fn to_be(self) -> Self { $t ::to_be(self) } #[inline(always)]
        fn to_le(self) -> Self { $t ::to_le(self) } })*
    };
}
/// Implement Endian on the floats by flipping their byte repr.
///
/// The to_ conversions use bare transmute, as the result may wind up looking
/// invalid on the host architecture when used in floating-point contexts. The
/// from_ conversions use Rust's from_bits functions, as the final result must
/// be a valid local floating-point number.
///
/// The to/from _bits() APIs on f32/64 were stabilized in Rust 1.20, and thus
/// this code cannot be run on Rust version below that.
macro_rules! implendian_f {
    ($($t:tt),*) => {
        $(impl Endian for $t { fn from_be(self) -> Self { Self::from_bits(self.to_bits()
        .from_be()) } fn from_le(self) -> Self { Self::from_bits(self.to_bits()
        .from_le()) } fn to_be(self) -> Self { Self::from_bits(self.to_bits().to_be()) }
        fn to_le(self) -> Self { Self::from_bits(self.to_bits().to_le()) } })*
    };
}
/// Implement on `bool`.
///
/// `bool` is always one byte, and single bytes don`t have endian order.
impl Endian for bool {
    fn from_be(self) -> Self {
        self
    }
    fn from_le(self) -> Self {
        self
    }
    fn to_be(self) -> Self {
        self
    }
    fn to_le(self) -> Self {
        self
    }
}
/// Implement on `char`.
///
/// `char` is four bytes wide. Delegate to `u32`'s implementation and transmute.
/// This is safe ONLY IF THE CONVERSION MAKES LOGICAL SENSE
/// `char` is Unicode codepoints, NOT integers, so not all values of `u32` are
/// valid values of `char`.
/// The `to_` functions will emit potentially invalid `char` values, and this is
/// to be expected. The `from_` functions, however, will panic if they are about
/// to emit an invalid `char` byte value.
impl Endian for char {
    /// Attempts to create a local `char` from a big-endian value.
    ///
    /// This function WILL panic if the local value exceeds the maximum Unicode
    /// Scalar Value permissible.
    fn from_be(self) -> Self {
        let flip: u32 = (self as u32).from_be();
        if flip > ::std::char::MAX as u32 {
            panic!("A `char` cannot have a value of {:X}", flip);
        }
        unsafe { ::std::mem::transmute(flip) }
    }
    /// Attempts to create a local `char` from a little-endian value.
    ///
    /// This function WILL panic if the local value exceeds the maximum Unicode
    /// Scalar Value permissible.
    fn from_le(self) -> Self {
        let flip: u32 = (self as u32).from_le();
        if flip > ::std::char::MAX as u32 {
            panic!("A `char` cannot have a value of {:X}", flip);
        }
        unsafe { ::std::mem::transmute(flip) }
    }
    /// Converts a local `char` to big-endian.
    ///
    /// This may result in a byte value that is not a valid Unicode Scalar Value
    /// and the result of this transform should be passed into a `from_be()`
    /// before using it in anything that requires `char` semantics.
    fn to_be(self) -> Self {
        unsafe { ::std::mem::transmute((self as u32).to_be()) }
    }
    /// Converts a local `char` to little-endian.
    ///
    /// This may result in a byte value that is not a valid Unicode Scalar Value
    /// and the result of this transform should be passed into a `from_le()`
    /// before using it in anything that requires `char` semantics.
    fn to_le(self) -> Self {
        unsafe { ::std::mem::transmute((self as u32).to_le()) }
    }
}
implendian!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
implendian_f!(f32, f64);
#[cfg(feature = "arrays")]
mod arrays;
mod slices;
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: bool = rug_fuzz_0;
        debug_assert_eq!(p0.from_be(), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: bool = rug_fuzz_0;
        <bool as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: bool = rug_fuzz_0;
        debug_assert_eq!(< bool as Endian > ::to_be(p0), false);
             }
});    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: bool = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: char = rug_fuzz_0;
        debug_assert_eq!(p0.from_be(), 'A');
             }
});    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: char = rug_fuzz_0;
        <char as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_char_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: char = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 'A');
             }
});    }
}
#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: char = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i8 = rug_fuzz_0;
        let result = <i8 as Endian>::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i8 = rug_fuzz_0;
        <i8 as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i8 = rug_fuzz_0;
        let result = p0.to_be();
        debug_assert_eq!(result, 42.to_be());
             }
});    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i8 = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(p0.from_be(), 0xAB);
             }
});    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(p0, u8::from_le(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 0xAC.to_be());
             }
});    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i16 = rug_fuzz_0;
        debug_assert_eq!(p0.from_be(), i16::from_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i16 = rug_fuzz_0;
        debug_assert_eq!(p0.from_le(), 255);
             }
});    }
}
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i16 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 12345.to_be());
             }
});    }
}
#[cfg(test)]
mod tests_rug_20 {
    use crate::Endian;
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i16 = rug_fuzz_0;
        <i16 as Endian>::to_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u16 = rug_fuzz_0;
        p0.from_be();
             }
});    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u16 = rug_fuzz_0;
        let result = p0.from_le();
        debug_assert_eq!(result, 0xCDAB);
             }
});    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u16 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 0xCDAB);
             }
});    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u16 = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i32 = rug_fuzz_0;
        debug_assert_eq!(rug_fuzz_1, < i32 as Endian > ::from_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(p0, < i32 as Endian > ::from_le(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), i32::to_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        <i32 as Endian>::to_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_29 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u32 = rug_fuzz_0;
        debug_assert_eq!(p0, u32::from_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        <u32 as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_31 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        debug_assert_eq!(rug_fuzz_1.to_be(), < u32 as Endian > ::to_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_32 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u32 = rug_fuzz_0;
        let result = p0.to_le();
        debug_assert_eq!(result, 42.to_le());
             }
});    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        <i64>::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_34 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        <i64 as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 12345i64.to_be());
             }
});    }
}
#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), u64::from_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u64 = rug_fuzz_0;
        <u64 as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 123456789.to_be());
             }
});    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u64 = rug_fuzz_0;
        debug_assert_eq!(p0.to_le(), 9814072354875320320);
             }
});    }
}
#[cfg(test)]
mod tests_rug_41 {
    use super::*;
    use crate::Endian;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i128 = Wrapping(rug_fuzz_0).0;
        let _result = i128::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_42 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i128 = rug_fuzz_0;
        p0 = i128::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_43 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i128 = rug_fuzz_0;
        <i128 as Endian>::to_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_44 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i128 = rug_fuzz_0;
        let result = p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_45 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u128 = rug_fuzz_0;
        let result = u128::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_46 {
    use super::*;
    use crate::Endian;
    use std::mem;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u128 = rug_fuzz_0;
        debug_assert_eq!(p0, u128::from_le(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_47 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u128 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), 128356735960003310900434392399436715170);
             }
});    }
}
#[cfg(test)]
mod tests_rug_48 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u128 = rug_fuzz_0;
        let result = <u128 as Endian>::to_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_49 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f32 = rug_fuzz_0;
        <f32 as Endian>::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f32 = rug_fuzz_0;
        <f32 as Endian>::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_51 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f32 = rug_fuzz_0;
        debug_assert_eq!(p0.to_be(), f32::to_be(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_52 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f32 = rug_fuzz_0;
        p0.to_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        <f64>::from_be(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_54 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        let result = f64::from_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_55 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_to_be() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        p0.to_be();
             }
});    }
}
#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::Endian;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f64 = rug_fuzz_0;
        <f64 as Endian>::to_le(p0);
             }
});    }
}
