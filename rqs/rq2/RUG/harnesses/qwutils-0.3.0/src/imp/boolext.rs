pub trait BoolExtOption {
    fn option(&self) -> Option<()>;
    fn result(&self) -> Result<(), ()>;
    #[inline]
    fn map<U>(&self, f: impl FnOnce() -> U) -> Option<U> {
        self.option().map(#[inline] |_| f())
    }
    #[inline]
    fn map_or<U>(&self, default: U, f: impl FnOnce() -> U) -> U {
        self.option().map_or(default, #[inline] |_| f())
    }
    #[inline]
    fn map_or_else<U>(&self, default: impl FnOnce() -> U, f: impl FnOnce() -> U) -> U {
        self.option().map_or_else(default, #[inline] |_| f())
    }
    #[inline]
    fn map_or_err<T, E>(
        &self,
        f: impl FnOnce() -> T,
        e: impl FnOnce() -> E,
    ) -> Result<T, E> {
        self.result().map(#[inline] |_| f()).map_err(#[inline] |_| e())
    }
}
impl BoolExtOption for bool {
    #[inline]
    fn option(&self) -> Option<()> {
        if *self { Some(()) } else { None }
    }
    #[inline]
    fn result(&self) -> Result<(), ()> {
        if *self { Ok(()) } else { Err(()) }
    }
}
#[cfg(test)]
mod tests_rug_71 {
    use super::*;
    use crate::imp::BoolExtOption;
    #[test]
    fn test_option() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(bool, bool, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: bool = rug_fuzz_0;
        debug_assert_eq!(rug_fuzz_1.option(), Some(()));
        let p1: bool = rug_fuzz_2;
        debug_assert_eq!(rug_fuzz_3.option(), None);
             }
});    }
}
#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    use crate::imp::BoolExtOption;
    #[test]
    fn test_result() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: bool = rug_fuzz_0;
        let result = <bool as BoolExtOption>::result(&p0);
        debug_assert_eq!(result, Ok(()));
             }
});    }
}
