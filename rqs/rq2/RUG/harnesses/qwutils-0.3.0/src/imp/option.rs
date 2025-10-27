use std::ops::*;
pub trait OptionExt<T> {
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R>;
    fn with_mut<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> Option<R>;
    fn with_if<R, U>(&self, o: &Option<U>, f: impl FnOnce(&T, &U) -> R) -> Option<R>;
    fn with_mut_if<R, U>(
        &mut self,
        o: &Option<U>,
        f: impl FnOnce(&mut T, &U) -> R,
    ) -> Option<R>;
    fn with_mut_if_saturating<R>(
        &mut self,
        o: Option<T>,
        f: impl FnOnce(&mut T, &T) -> R,
    ) -> Option<R>;
    fn add_to<V>(&mut self, v: V)
    where
        T: AddAssign<V>;
    fn sub_to<V>(&mut self, v: V)
    where
        T: SubAssign<V>;
    fn mul_to<V>(&mut self, v: V)
    where
        T: MulAssign<V>;
    fn div_to<V>(&mut self, v: V)
    where
        T: DivAssign<V>;
    fn add_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: AddAssign<V>;
    fn sub_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: SubAssign<V>;
    fn mul_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: MulAssign<V>;
    fn div_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: DivAssign<V>;
    fn add_to_if<V>(&mut self, v: Option<V>)
    where
        T: AddAssign<V>;
    fn sub_to_if<V>(&mut self, v: Option<V>)
    where
        T: SubAssign<V>;
    fn mul_to_if<V>(&mut self, v: Option<V>)
    where
        T: MulAssign<V>;
    fn div_to_if<V>(&mut self, v: Option<V>)
    where
        T: DivAssign<V>;
}
impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.as_ref().map(f)
    }
    #[inline]
    fn with_mut<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.as_mut().map(f)
    }
    #[inline]
    fn with_if<R, U>(&self, o: &Option<U>, f: impl FnOnce(&T, &U) -> R) -> Option<R> {
        flatten(self.with(#[inline] |s| o.with(|o| f(s, o))))
    }
    #[inline]
    fn with_mut_if<R, U>(
        &mut self,
        o: &Option<U>,
        f: impl FnOnce(&mut T, &U) -> R,
    ) -> Option<R> {
        flatten(self.with_mut(#[inline] |s| o.with(|o| f(s, o))))
    }
    #[inline]
    fn with_mut_if_saturating<R>(
        &mut self,
        o: Option<T>,
        f: impl FnOnce(&mut T, &T) -> R,
    ) -> Option<R> {
        if let Some(s) = self {
            o.with(#[inline] |o| f(s, o))
        } else {
            *self = o;
            None
        }
    }
    #[inline]
    fn add_to<V>(&mut self, v: V)
    where
        T: AddAssign<V>,
    {
        self.with_mut(#[inline] |s| AddAssign::add_assign(s, v));
    }
    #[inline]
    fn sub_to<V>(&mut self, v: V)
    where
        T: SubAssign<V>,
    {
        self.with_mut(#[inline] |s| SubAssign::sub_assign(s, v));
    }
    #[inline]
    fn mul_to<V>(&mut self, v: V)
    where
        T: MulAssign<V>,
    {
        self.with_mut(#[inline] |s| MulAssign::mul_assign(s, v));
    }
    #[inline]
    fn div_to<V>(&mut self, v: V)
    where
        T: DivAssign<V>,
    {
        self.with_mut(#[inline] |s| DivAssign::div_assign(s, v));
    }
    #[inline]
    fn add_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: AddAssign<V>,
    {
        if let Some(v) = v {
            self.add_to(v)
        } else {
            *self = None;
        }
    }
    #[inline]
    fn sub_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: SubAssign<V>,
    {
        if let Some(v) = v {
            self.sub_to(v)
        } else {
            *self = None;
        }
    }
    #[inline]
    fn mul_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: MulAssign<V>,
    {
        if let Some(v) = v {
            self.mul_to(v)
        } else {
            *self = None;
        }
    }
    #[inline]
    fn div_to_lossy<V>(&mut self, v: Option<V>)
    where
        T: DivAssign<V>,
    {
        if let Some(v) = v {
            self.div_to(v)
        } else {
            *self = None;
        }
    }
    #[inline]
    fn add_to_if<V>(&mut self, v: Option<V>)
    where
        T: AddAssign<V>,
    {
        if let Some(v) = v {
            self.add_to(v)
        }
    }
    #[inline]
    fn sub_to_if<V>(&mut self, v: Option<V>)
    where
        T: SubAssign<V>,
    {
        if let Some(v) = v {
            self.sub_to(v)
        }
    }
    #[inline]
    fn mul_to_if<V>(&mut self, v: Option<V>)
    where
        T: MulAssign<V>,
    {
        if let Some(v) = v {
            self.mul_to(v)
        }
    }
    #[inline]
    fn div_to_if<V>(&mut self, v: Option<V>)
    where
        T: DivAssign<V>,
    {
        if let Some(v) = v {
            self.div_to(v)
        }
    }
}
#[inline]
fn flatten<T>(i: Option<Option<T>>) -> Option<T> {
    match i {
        Some(j) => j,
        None => None,
    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<Option<u32>> = Some(Some(rug_fuzz_0));
        crate::imp::option::flatten(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::imp::OptionExt;
    use std::option::Option;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<i32> = Some(rug_fuzz_0);
        let mut p1 = |x: &i32| x + rug_fuzz_1;
        p0.with(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Some(rug_fuzz_0);
        let p1 = Some(rug_fuzz_1);
        let p2 = |a: &mut i32, b: &i32| *a += b;
        p0.with_mut_if_saturating(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::imp::OptionExt;
    use std::ops::AddAssign;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Some(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        p0.add_to(p1);
        debug_assert_eq!(p0, Some(8));
             }
});    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::imp::OptionExt;
    use std::option::Option;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<i32> = Some(rug_fuzz_0);
        let mut p1: i32 = rug_fuzz_1;
        p0.sub_to(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<i32> = Some(rug_fuzz_0);
        let p1: i32 = rug_fuzz_1;
        p0.mul_to(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::imp::OptionExt;
    use std::option::Option;
    #[test]
    fn test_div_to() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<i32> = Some(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        p0.div_to(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<i32> = Some(rug_fuzz_0);
        let p1: Option<i32> = Some(rug_fuzz_1);
        p0.add_to_lossy(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Some(rug_fuzz_0);
        let p1 = Some(rug_fuzz_1);
        p0.sub_to_lossy(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Some(rug_fuzz_0);
        let p1 = Some(rug_fuzz_1);
        p0.mul_to_lossy(p1);
        debug_assert_eq!(p0, Some(15));
             }
});    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = std::option::Option::<i32>::None;
        let p1 = std::option::Option::<i32>::Some(rug_fuzz_0);
        p0.add_to_if(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::imp::OptionExt;
    struct ExampleType;
    impl SubAssign<i32> for ExampleType {
        fn sub_assign(&mut self, _rhs: i32) {}
    }
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<ExampleType> = Some(ExampleType);
        let p1: Option<i32> = Some(rug_fuzz_0);
        <std::option::Option<ExampleType>>::sub_to_if(&mut p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Some(rug_fuzz_0);
        let p1 = Some(rug_fuzz_1);
        p0.mul_to_if(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::imp::OptionExt;
    #[test]
    fn test_div_to_if() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<i32> = Some(rug_fuzz_0);
        let p1: Option<i32> = Some(rug_fuzz_1);
        p0.div_to_if(p1);
             }
});    }
}
