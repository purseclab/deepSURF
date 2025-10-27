use std::iter::Sum;
use std::ops::*;
pub trait AsTuple {
    type Dest;
    fn as_tuple(self) -> Self::Dest;
}
pub trait AsArray {
    type Dest;
    fn as_array(self) -> Self::Dest;
}
pub trait TupleFns<T>
where
    T: 'static,
{
    fn avg<U>(&self) -> U
    where
        T: Clone,
        U: From<T> + From<u8> + Sum<U> + DivAssign<U>;
}
macro_rules! impl_arr {
    {$n:expr;$t:ident $($ts:ident)*;$l : ident $($ls:ident)*} => {
        impl < T > AsTuple for [T; $n] { type Dest = ($t,$($ts),*); #[inline] fn
        as_tuple(self) -> Self::Dest { let [$l,$($ls),*] = self; ($l,$($ls),*) } } impl <
        T > AsArray for ($t,$($ts),*) { type Dest = [T; $n]; #[inline] fn as_array(self)
        -> Self::Dest { let ($l,$($ls),*) = self; [$l,$($ls),*] } } impl < T > TupleFns <
        T > for [T; $n] where T : 'static { #[inline] fn avg < U > (& self) -> U where T
        : Clone, U : From < T > + From < u8 > + Sum < U > + DivAssign < U > { let mut
        dest : U = self.iter().cloned().map(U::from).sum(); dest /= U::from($n); dest } }
        impl < T > TupleFns < T > for ($t,$($ts),*) where Self : Clone, T : 'static {
        #[inline] fn avg < U > (& self) -> U where T : Clone, U : From < T > + From < u8
        > + Sum < U > + DivAssign < U > { (* self).clone().as_array().avg() } } impl_arr!
        { ($n - 1);$($ts)*;$($ls) * }
    };
    {$n:expr;;} => {};
}
impl_arr! {
    32; T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T; a b c d e f g h
    i j k l m n o p q r s t u v w x y z aa ab ac ad ae af
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_it() {
        let tuple = (1u32, 2u32, 3u32, 4u32);
        let arr = tuple.clone().as_array();
        assert_eq!(arr, [1u32, 2, 3, 4]);
        let t = arr.as_tuple();
        assert_eq!(t, tuple);
        assert_eq!((3u32, 6u32, 8u32, 3u32, 5u32, 2u32).avg::< u32 > (), 4u32);
        assert_eq!((3u32, 6u32, 8u32, 3u32, 5u32, 2u32).avg::< f64 > (), 4.5);
    }
}
#[cfg(test)]
mod tests_rug_73 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 32] = [rug_fuzz_0; 32];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_75 {
    use super::*;
    use crate::imp::TupleFns;
    use std::ops::{DivAssign, AddAssign};
    use std::iter::Sum;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 32] = [rug_fuzz_0; 32];
        p0.avg::<u8>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_77 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_79 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use std::iter::FromIterator;
    use std::ops::{DivAssign, AddAssign};
    use std::convert::TryInto;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<f64>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_81 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_83 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use std::ops::{DivAssign, AddAssign};
    use std::iter::Sum;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<u32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_85 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [usize; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        <[usize; 3]>::as_tuple(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_87 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use std::iter::Sum;
    use std::ops::DivAssign;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        <[u8; 4]>::avg::<u16>(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_89 {
    use super::*;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_90 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_91 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use std::ops::DivAssign;
    use std::iter::Sum;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.avg::<u8>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_95 {
    use super::*;
    use crate::imp::TupleFns;
    use std::iter::FromIterator;
    use std::ops::{DivAssign, AddAssign};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<u32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_97 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_99 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<f32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_101 {
    use super::*;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<i32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_105 {
    use super::*;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.avg::<u16>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_109 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_110 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ((
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
        ));
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::imp::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        p0.avg::<i32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_117 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_118 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_121 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_122 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
        );
        <(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        )>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_123 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.avg::<u8>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_126 {
    use super::*;
    use crate::imp::tuple::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_127 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        <[u8; 3]>::avg::<f64>(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_129 {
    use super::*;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_130 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<i32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_133 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_134 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_137 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_138 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_141 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_142 {
    use super::*;
    use crate::imp::tuple::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_143 {
    use super::*;
    use crate::imp::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.avg::<f32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_145 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_146 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_147 {
    use super::*;
    use crate::imp::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        p0.avg::<f64>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_149 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_150 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_153 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_154 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_155 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<i32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_157 {
    use super::*;
    use crate::imp::AsTuple;
    use crate::imp;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_158 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_159 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<i32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_161 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_162 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_163 {
    use super::*;
    use crate::imp::TupleFns;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        p0.avg::<u8>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_165 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_166 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32, i32, i32, i32, i32, i32, i32, i32, i32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
        );
        <(i32, i32, i32, i32, i32, i32, i32, i32, i32)>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_167 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        <[i32; 4]>::avg::<i32>(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_169 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_170 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32, i32, i32, i32, i32, i32, i32, i32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        );
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_171 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array: [u8; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let p0 = array;
        p0.avg::<u8>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_173 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_174 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
        );
        <(i32, i32, i32, i32, i32, i32, i32)>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_177 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_178 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
        );
        <(i32, i32, i32, i32, i32, i32)>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [u32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_as_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: (i32, i32, i32, i32, i32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        );
        <(i32, i32, i32, i32, i32)>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [u8; 5] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        p0.avg::<f32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_185 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_186 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32, i32, i32, i32) = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        <(i32, i32, i32, i32)>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use std::iter::once;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.avg::<f64>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_as_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_190 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32, i32, i32) = (rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        <(i32, i32, i32)>::as_array(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_191 {
    use super::*;
    use crate::imp::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.avg::<i32>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_194 {
    use super::*;
    use crate::imp::tuple::AsArray;
    use crate::imp::tuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32, i32) = (rug_fuzz_0, rug_fuzz_1);
        p0.as_array();
             }
});    }
}
#[cfg(test)]
mod tests_rug_195 {
    use super::*;
    use crate::imp::tuple::TupleFns;
    use crate::imp::tuple::AsTuple;
    #[test]
    fn test_avg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        p0.avg::<f64>();
             }
});    }
}
#[cfg(test)]
mod tests_rug_197 {
    use super::*;
    use crate::imp::AsTuple;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.as_tuple();
             }
});    }
}
#[cfg(test)]
mod tests_rug_198 {
    use super::*;
    use crate::imp::AsArray;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: (i32,) = (rug_fuzz_0,);
        p0.as_array();
             }
});    }
}
