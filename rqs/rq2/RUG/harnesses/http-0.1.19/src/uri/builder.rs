use {Uri, Result};
use convert::{HttpTryFrom, HttpTryInto};
use super::{Authority, Scheme, Parts, PathAndQuery};
/// A builder for `Uri`s.
///
/// This type can be used to construct an instance of `Uri`
/// through a builder pattern.
#[derive(Debug)]
pub struct Builder {
    parts: Option<Result<Parts>>,
}
impl Builder {
    /// Creates a new default instance of `Builder` to construct a `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let uri = uri::Builder::new()
    ///     .scheme("https")
    ///     .authority("hyper.rs")
    ///     .path_and_query("/")
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }
    /// Set the `Scheme` for this URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let mut builder = uri::Builder::new();
    /// builder.scheme("https");
    /// ```
    pub fn scheme<T>(&mut self, scheme: T) -> &mut Self
    where
        Scheme: HttpTryFrom<T>,
    {
        self.map(|parts| {
            parts.scheme = Some(scheme.http_try_into()?);
            Ok(())
        })
    }
    /// Set the `Authority` for this URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let uri = uri::Builder::new()
    ///     .authority("tokio.rs")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn authority<T>(&mut self, auth: T) -> &mut Self
    where
        Authority: HttpTryFrom<T>,
    {
        self.map(|parts| {
            parts.authority = Some(auth.http_try_into()?);
            Ok(())
        })
    }
    /// Set the `PathAndQuery` for this URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let uri = uri::Builder::new()
    ///     .path_and_query("/hello?foo=bar")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn path_and_query<T>(&mut self, p_and_q: T) -> &mut Self
    where
        PathAndQuery: HttpTryFrom<T>,
    {
        self.map(|parts| {
            parts.path_and_query = Some(p_and_q.http_try_into()?);
            Ok(())
        })
    }
    /// Consumes this builder, and tries to construct a valid `Uri` from
    /// the configured pieces.
    ///
    /// # Errors
    ///
    /// This function may return an error if any previously configured argument
    /// failed to parse or get converted to the internal representation. For
    /// example if an invalid `scheme` was specified via `scheme("!@#%/^")`
    /// the error will be returned when this function is called rather than
    /// when `scheme` was called.
    ///
    /// Additionally, the various forms of URI require certain combinations of
    /// parts to be set to be valid. If the parts don't fit into any of the
    /// valid forms of URI, a new error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let uri = Uri::builder()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(&mut self) -> Result<Uri> {
        self.parts
            .take()
            .expect("cannot reuse Uri builder")
            .and_then(|parts| parts.http_try_into())
    }
    fn map<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Parts) -> Result<()>,
    {
        let res = if let Some(Ok(ref mut parts)) = self.parts {
            f(parts)
        } else {
            return self;
        };
        if let Err(err) = res {
            self.parts = Some(Err(err));
        }
        self
    }
}
impl Default for Builder {
    #[inline]
    fn default() -> Builder {
        Builder {
            parts: Some(Ok(Parts::default())),
        }
    }
}
#[cfg(test)]
mod tests_rug_449 {
    use super::*;
    use crate::uri;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_449_rrrruuuugggg_test_rug = 0;
        let uri_builder: uri::Builder = uri::Builder::new();
        let _rug_ed_tests_rug_449_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_450 {
    use super::*;
    use crate::uri;
    #[test]
    fn test_scheme() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = uri::Builder::new();
        let p1: &str = rug_fuzz_0;
        p0.scheme(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_451 {
    use super::*;
    use crate::uri;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = uri::Builder::new();
        let p1: &str = rug_fuzz_0;
        p0.authority(p1);
             }
});    }
}
use crate::uri;
#[cfg(test)]
mod tests_rug_452 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = uri::Builder::new();
        let p1: &str = rug_fuzz_0;
        p0.path_and_query(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_453 {
    use super::*;
    use crate::Uri;
    use crate::uri;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_453_rrrruuuugggg_test_rug = 0;
        let mut p0 = uri::Builder::new();
        p0.build();
        let _rug_ed_tests_rug_453_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_455 {
    use super::*;
    use crate::uri::builder::Builder;
    use crate::uri::Parts;
    #[test]
    fn test_default() {
        let _rug_st_tests_rug_455_rrrruuuugggg_test_default = 0;
        let builder: Builder = Default::default();
        let _rug_ed_tests_rug_455_rrrruuuugggg_test_default = 0;
    }
}
