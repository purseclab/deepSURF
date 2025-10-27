use Error;
use header::{HeaderName, HeaderValue, HeaderMap};
use method::Method;
use sealed::Sealed;
use status::StatusCode;
use uri::{Scheme, Authority, PathAndQuery, Uri};
/// Private trait for the `http` crate to have generic methods with fallible
/// conversions.
///
/// This trait is similar to the `TryFrom` trait proposed in the standard
/// library, except this is specialized for the `http` crate and isn't intended
/// for general consumption.
///
/// This trait cannot be implemented types outside of the `http` crate, and is
/// only intended for use as a generic bound on methods in the `http` crate.
pub trait HttpTryFrom<T>: Sized + Sealed {
    /// Associated error with the conversion this implementation represents.
    type Error: Into<Error>;
    #[doc(hidden)]
    fn try_from(t: T) -> Result<Self, Self::Error>;
}
pub(crate) trait HttpTryInto<T>: Sized {
    fn http_try_into(self) -> Result<T, Error>;
}
#[doc(hidden)]
impl<T, U> HttpTryInto<U> for T
where
    U: HttpTryFrom<T>,
    T: Sized,
{
    fn http_try_into(self) -> Result<U, Error> {
        HttpTryFrom::try_from(self).map_err(|e: U::Error| e.into())
    }
}
macro_rules! reflexive {
    ($($t:ty,)*) => {
        $(impl HttpTryFrom <$t > for $t { type Error = Error; fn try_from(t : Self) ->
        Result < Self, Self::Error > { Ok(t) } } impl Sealed for $t {})*
    };
}
reflexive! {
    Uri, Method, StatusCode, HeaderName, HeaderValue, Scheme, Authority, PathAndQuery,
}
impl<T> HttpTryFrom<HeaderMap<T>> for HeaderMap<T> {
    type Error = Error;
    fn try_from(t: Self) -> Result<Self, Self::Error> {
        Ok(t)
    }
}
impl<T> Sealed for HeaderMap<T> {}
#[cfg(test)]
mod tests_rug_439 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::Uri;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Uri::builder()
            .scheme(rug_fuzz_0)
            .authority(rug_fuzz_1)
            .path_and_query(rug_fuzz_2)
            .build()
            .unwrap();
        <Uri as HttpTryFrom<Uri>>::try_from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_440 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::Method;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_440_rrrruuuugggg_test_rug = 0;
        let mut p0 = Method::GET;
        <Method as HttpTryFrom<Method>>::try_from(p0);
        let _rug_ed_tests_rug_440_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_445 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::uri::Authority;
    use bytes::Bytes;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        <Authority as HttpTryFrom<Authority>>::try_from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_446 {
    use super::*;
    use crate::HttpTryFrom;
    use uri::PathAndQuery;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_446_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/test";
        let p0 = PathAndQuery::from_static(rug_fuzz_0);
        <PathAndQuery as HttpTryFrom<PathAndQuery>>::try_from(p0).unwrap();
        let _rug_ed_tests_rug_446_rrrruuuugggg_test_rug = 0;
    }
}
