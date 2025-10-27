pub trait ResultNonDebugUnwrap<T, E> {
    fn expect_nodebug(self, msg: &str) -> T;
    fn expect_err_nodebug(self, msg: &str) -> E;
    fn unwrap_nodebug(self) -> T;
    fn unwrap_err_nodebug(self) -> E;
}
impl<T, E> ResultNonDebugUnwrap<T, E> for Result<T, E> {
    #[inline]
    fn unwrap_nodebug(self) -> T {
        match self {
            Ok(t) => t,
            Err(_) => unwrap_failed::<E>("called `Result::unwrap()` on an `Err` value"),
        }
    }
    #[inline]
    fn expect_nodebug(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => unwrap_failed::<E>(msg),
        }
    }
    #[inline]
    fn unwrap_err_nodebug(self) -> E {
        match self {
            Ok(_) => unwrap_failed::<T>("called `Result::unwrap_err()` on an `Ok` value"),
            Err(e) => e,
        }
    }
    #[inline]
    fn expect_err_nodebug(self, msg: &str) -> E {
        match self {
            Ok(_) => unwrap_failed::<T>(msg),
            Err(e) => e,
        }
    }
}
#[inline(never)]
#[cold]
fn unwrap_failed<E>(msg: &str) -> ! {
    panic!("{}: {}", msg, std::any::type_name::< E > ())
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = rug_fuzz_0;
        crate::imp::result::unwrap_failed::<u32>(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    use crate::imp::ResultNonDebugUnwrap;
    use std::result::Result;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Result<i32, &str> = Ok(rug_fuzz_0);
        <Result<i32, &str>>::unwrap_nodebug(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use crate::imp::ResultNonDebugUnwrap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: std::result::Result<i32, String> = Ok(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        <std::result::Result<i32, String>>::expect_nodebug(p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::imp::ResultNonDebugUnwrap;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "error message";
        let mut p0: Result<(), &'static str> = Err(rug_fuzz_0);
        <std::result::Result<(), &'static str>>::unwrap_err_nodebug(p0);
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::imp::ResultNonDebugUnwrap;
    #[test]
    fn test_result_expect_err_nodebug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: Result<i32, &str> = Ok(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        p0.expect_err_nodebug(p1);
             }
});    }
}
