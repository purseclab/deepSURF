use std::{cmp, fmt, str};
use std::str::FromStr;
use bytes::Bytes;
use byte_str::ByteStr;
use convert::HttpTryFrom;
use super::{ErrorKind, InvalidUri, InvalidUriBytes};
/// Represents the path component of a URI
#[derive(Clone)]
pub struct PathAndQuery {
    pub(super) data: ByteStr,
    pub(super) query: u16,
}
const NONE: u16 = ::std::u16::MAX;
impl PathAndQuery {
    /// Attempt to convert a `PathAndQuery` from `Bytes`.
    ///
    /// This function will be replaced by a `TryFrom` implementation once the
    /// trait lands in stable.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate http;
    /// # use http::uri::*;
    /// extern crate bytes;
    ///
    /// use bytes::Bytes;
    ///
    /// # pub fn main() {
    /// let bytes = Bytes::from("/hello?world");
    /// let path_and_query = PathAndQuery::from_shared(bytes).unwrap();
    ///
    /// assert_eq!(path_and_query.path(), "/hello");
    /// assert_eq!(path_and_query.query(), Some("world"));
    /// # }
    /// ```
    pub fn from_shared(mut src: Bytes) -> Result<Self, InvalidUriBytes> {
        let mut query = NONE;
        let mut fragment = None;
        #[allow(warnings)]
        {
            let mut iter = src.as_ref().iter().enumerate();
            for (i, &b) in &mut iter {
                match b {
                    b'?' => {
                        debug_assert_eq!(query, NONE);
                        query = i as u16;
                        break;
                    }
                    b'#' => {
                        fragment = Some(i);
                        break;
                    }
                    0x21
                    | 0x24..=0x3B
                    | 0x3D
                    | 0x40..=0x5F
                    | 0x61..=0x7A
                    | 0x7C
                    | 0x7E => {}
                    _ => return Err(ErrorKind::InvalidUriChar.into()),
                }
            }
            if query != NONE {
                #[allow(warnings)]
                for (i, &b) in iter {
                    match b {
                        0x21 | 0x24..=0x3B | 0x3D | 0x3F..=0x7E => {}
                        b'#' => {
                            fragment = Some(i);
                            break;
                        }
                        _ => return Err(ErrorKind::InvalidUriChar.into()),
                    }
                }
            }
        }
        if let Some(i) = fragment {
            src.truncate(i);
        }
        Ok(PathAndQuery {
            data: unsafe { ByteStr::from_utf8_unchecked(src) },
            query: query,
        })
    }
    /// Convert a `PathAndQuery` from a static string.
    ///
    /// This function will not perform any copying, however the string is
    /// checked to ensure that it is valid.
    ///
    /// # Panics
    ///
    /// This function panics if the argument is an invalid path and query.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::uri::*;
    /// let v = PathAndQuery::from_static("/hello?world");
    ///
    /// assert_eq!(v.path(), "/hello");
    /// assert_eq!(v.query(), Some("world"));
    /// ```
    #[inline]
    pub fn from_static(src: &'static str) -> Self {
        let src = Bytes::from_static(src.as_bytes());
        PathAndQuery::from_shared(src).unwrap()
    }
    pub(super) fn empty() -> Self {
        PathAndQuery {
            data: ByteStr::new(),
            query: NONE,
        }
    }
    pub(super) fn slash() -> Self {
        PathAndQuery {
            data: ByteStr::from_static("/"),
            query: NONE,
        }
    }
    pub(super) fn star() -> Self {
        PathAndQuery {
            data: ByteStr::from_static("*"),
            query: NONE,
        }
    }
    /// Returns the path component
    ///
    /// The path component is **case sensitive**.
    ///
    /// ```notrust
    /// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
    ///                                        |--------|
    ///                                             |
    ///                                           path
    /// ```
    ///
    /// If the URI is `*` then the path component is equal to `*`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::uri::*;
    ///
    /// let path_and_query: PathAndQuery = "/hello/world".parse().unwrap();
    ///
    /// assert_eq!(path_and_query.path(), "/hello/world");
    /// ```
    #[inline]
    pub fn path(&self) -> &str {
        let ret = if self.query == NONE {
            &self.data[..]
        } else {
            &self.data[..self.query as usize]
        };
        if ret.is_empty() {
            return "/";
        }
        ret
    }
    /// Returns the query string component
    ///
    /// The query component contains non-hierarchical data that, along with data
    /// in the path component, serves to identify a resource within the scope of
    /// the URI's scheme and naming authority (if any). The query component is
    /// indicated by the first question mark ("?") character and terminated by a
    /// number sign ("#") character or by the end of the URI.
    ///
    /// ```notrust
    /// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
    ///                                                   |-------------------|
    ///                                                             |
    ///                                                           query
    /// ```
    ///
    /// # Examples
    ///
    /// With a query string component
    ///
    /// ```
    /// # use http::uri::*;
    /// let path_and_query: PathAndQuery = "/hello/world?key=value&foo=bar".parse().unwrap();
    ///
    /// assert_eq!(path_and_query.query(), Some("key=value&foo=bar"));
    /// ```
    ///
    /// Without a query string component
    ///
    /// ```
    /// # use http::uri::*;
    /// let path_and_query: PathAndQuery = "/hello/world".parse().unwrap();
    ///
    /// assert!(path_and_query.query().is_none());
    /// ```
    #[inline]
    pub fn query(&self) -> Option<&str> {
        if self.query == NONE {
            None
        } else {
            let i = self.query + 1;
            Some(&self.data[i as usize..])
        }
    }
    /// Returns the path and query as a string component.
    ///
    /// # Examples
    ///
    /// With a query string component
    ///
    /// ```
    /// # use http::uri::*;
    /// let path_and_query: PathAndQuery = "/hello/world?key=value&foo=bar".parse().unwrap();
    ///
    /// assert_eq!(path_and_query.as_str(), "/hello/world?key=value&foo=bar");
    /// ```
    ///
    /// Without a query string component
    ///
    /// ```
    /// # use http::uri::*;
    /// let path_and_query: PathAndQuery = "/hello/world".parse().unwrap();
    ///
    /// assert_eq!(path_and_query.as_str(), "/hello/world");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        let ret = &self.data[..];
        if ret.is_empty() {
            return "/";
        }
        ret
    }
    /// Converts this `PathAndQuery` back to a sequence of bytes
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}
impl HttpTryFrom<Bytes> for PathAndQuery {
    type Error = InvalidUriBytes;
    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        PathAndQuery::from_shared(bytes)
    }
}
impl<'a> HttpTryFrom<&'a [u8]> for PathAndQuery {
    type Error = InvalidUri;
    #[inline]
    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        PathAndQuery::from_shared(s.into()).map_err(|e| e.0)
    }
}
impl<'a> HttpTryFrom<&'a str> for PathAndQuery {
    type Error = InvalidUri;
    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        HttpTryFrom::try_from(s.as_bytes())
    }
}
impl FromStr for PathAndQuery {
    type Err = InvalidUri;
    #[inline]
    fn from_str(s: &str) -> Result<Self, InvalidUri> {
        HttpTryFrom::try_from(s)
    }
}
impl From<PathAndQuery> for Bytes {
    fn from(src: PathAndQuery) -> Bytes {
        src.data.into()
    }
}
impl fmt::Debug for PathAndQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl fmt::Display for PathAndQuery {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if !self.data.is_empty() {
            match self.data.as_bytes()[0] {
                b'/' | b'*' => write!(fmt, "{}", & self.data[..]),
                _ => write!(fmt, "/{}", & self.data[..]),
            }
        } else {
            write!(fmt, "/")
        }
    }
}
impl PartialEq for PathAndQuery {
    #[inline]
    fn eq(&self, other: &PathAndQuery) -> bool {
        self.data == other.data
    }
}
impl Eq for PathAndQuery {}
impl PartialEq<str> for PathAndQuery {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl<'a> PartialEq<PathAndQuery> for &'a str {
    #[inline]
    fn eq(&self, other: &PathAndQuery) -> bool {
        self == &other.as_str()
    }
}
impl<'a> PartialEq<&'a str> for PathAndQuery {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}
impl PartialEq<PathAndQuery> for str {
    #[inline]
    fn eq(&self, other: &PathAndQuery) -> bool {
        self == other.as_str()
    }
}
impl PartialEq<String> for PathAndQuery {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<PathAndQuery> for String {
    #[inline]
    fn eq(&self, other: &PathAndQuery) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialOrd for PathAndQuery {
    #[inline]
    fn partial_cmp(&self, other: &PathAndQuery) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}
impl PartialOrd<str> for PathAndQuery {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}
impl PartialOrd<PathAndQuery> for str {
    #[inline]
    fn partial_cmp(&self, other: &PathAndQuery) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}
impl<'a> PartialOrd<&'a str> for PathAndQuery {
    #[inline]
    fn partial_cmp(&self, other: &&'a str) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}
impl<'a> PartialOrd<PathAndQuery> for &'a str {
    #[inline]
    fn partial_cmp(&self, other: &PathAndQuery) -> Option<cmp::Ordering> {
        self.partial_cmp(&other.as_str())
    }
}
impl PartialOrd<String> for PathAndQuery {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}
impl PartialOrd<PathAndQuery> for String {
    #[inline]
    fn partial_cmp(&self, other: &PathAndQuery) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_to_self_of_same_path() {
        let p1: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        let p2: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        assert_eq!(p1, p2);
        assert_eq!(p2, p1);
    }
    #[test]
    fn not_equal_to_self_of_different_path() {
        let p1: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        let p2: PathAndQuery = "/world&foo=bar".parse().unwrap();
        assert_ne!(p1, p2);
        assert_ne!(p2, p1);
    }
    #[test]
    fn equates_with_a_str() {
        let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        assert_eq!(& path_and_query, "/hello/world&foo=bar");
        assert_eq!("/hello/world&foo=bar", & path_and_query);
        assert_eq!(path_and_query, "/hello/world&foo=bar");
        assert_eq!("/hello/world&foo=bar", path_and_query);
    }
    #[test]
    fn not_equal_with_a_str_of_a_different_path() {
        let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        assert_ne!(& path_and_query, "/hello&foo=bar");
        assert_ne!("/hello&foo=bar", & path_and_query);
        assert_ne!(path_and_query, "/hello&foo=bar");
        assert_ne!("/hello&foo=bar", path_and_query);
    }
    #[test]
    fn equates_with_a_string() {
        let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        assert_eq!(path_and_query, "/hello/world&foo=bar".to_string());
        assert_eq!("/hello/world&foo=bar".to_string(), path_and_query);
    }
    #[test]
    fn not_equal_with_a_string_of_a_different_path() {
        let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
        assert_ne!(path_and_query, "/hello&foo=bar".to_string());
        assert_ne!("/hello&foo=bar".to_string(), path_and_query);
    }
    #[test]
    fn compares_to_self() {
        let p1: PathAndQuery = "/a/world&foo=bar".parse().unwrap();
        let p2: PathAndQuery = "/b/world&foo=bar".parse().unwrap();
        assert!(p1 < p2);
        assert!(p2 > p1);
    }
    #[test]
    fn compares_with_a_str() {
        let path_and_query: PathAndQuery = "/b/world&foo=bar".parse().unwrap();
        assert!(& path_and_query < "/c/world&foo=bar");
        assert!("/c/world&foo=bar" > & path_and_query);
        assert!(& path_and_query > "/a/world&foo=bar");
        assert!("/a/world&foo=bar" < & path_and_query);
        assert!(path_and_query < "/c/world&foo=bar");
        assert!("/c/world&foo=bar" > path_and_query);
        assert!(path_and_query > "/a/world&foo=bar");
        assert!("/a/world&foo=bar" < path_and_query);
    }
    #[test]
    fn compares_with_a_string() {
        let path_and_query: PathAndQuery = "/b/world&foo=bar".parse().unwrap();
        assert!(path_and_query < "/c/world&foo=bar".to_string());
        assert!("/c/world&foo=bar".to_string() > path_and_query);
        assert!(path_and_query > "/a/world&foo=bar".to_string());
        assert!("/a/world&foo=bar".to_string() < path_and_query);
    }
    #[test]
    fn ignores_valid_percent_encodings() {
        assert_eq!("/a%20b", pq("/a%20b?r=1").path());
        assert_eq!("qr=%31", pq("/a/b?qr=%31").query().unwrap());
    }
    #[test]
    fn ignores_invalid_percent_encodings() {
        assert_eq!("/a%%b", pq("/a%%b?r=1").path());
        assert_eq!("/aaa%", pq("/aaa%").path());
        assert_eq!("/aaa%", pq("/aaa%?r=1").path());
        assert_eq!("/aa%2", pq("/aa%2").path());
        assert_eq!("/aa%2", pq("/aa%2?r=1").path());
        assert_eq!("qr=%3", pq("/a/b?qr=%3").query().unwrap());
    }
    fn pq(s: &str) -> PathAndQuery {
        s.parse().expect(&format!("parsing {}", s))
    }
}
#[cfg(test)]
mod tests_rug_456 {
    use super::*;
    use bytes::Bytes;
    use crate::uri::PathAndQuery;
    use crate::uri::InvalidUriBytes;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let mut p0: Bytes = bytes;
        let _ = PathAndQuery::from_shared(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_457 {
    use super::*;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_from_static() {
        let _rug_st_tests_rug_457_rrrruuuugggg_test_from_static = 0;
        let rug_fuzz_0 = "/hello?world";
        let p0: &str = rug_fuzz_0;
        let v = PathAndQuery::from_static(&p0);
        debug_assert_eq!(v.path(), "/hello");
        debug_assert_eq!(v.query(), Some("world"));
        let _rug_ed_tests_rug_457_rrrruuuugggg_test_from_static = 0;
    }
}
#[cfg(test)]
mod tests_rug_458 {
    use super::*;
    use crate::uri::path::{PathAndQuery, ByteStr, NONE};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_458_rrrruuuugggg_test_rug = 0;
        let empty_path = PathAndQuery::empty();
        let _rug_ed_tests_rug_458_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_459 {
    use super::*;
    use uri::path::PathAndQuery;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_459_rrrruuuugggg_test_rug = 0;
        PathAndQuery::slash();
        let _rug_ed_tests_rug_459_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_460 {
    use super::*;
    use uri::path::PathAndQuery;
    use uri::ByteStr;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_460_rrrruuuugggg_test_rug = 0;
        PathAndQuery::star();
        let _rug_ed_tests_rug_460_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_461 {
    use super::*;
    use crate::uri::{PathAndQuery, InvalidUri};
    #[test]
    fn test_path() {
        let _rug_st_tests_rug_461_rrrruuuugggg_test_path = 0;
        let rug_fuzz_0 = "/hello/world";
        let p0 = PathAndQuery::from_static(rug_fuzz_0);
        debug_assert_eq!(p0.path(), "/hello/world");
        let _rug_ed_tests_rug_461_rrrruuuugggg_test_path = 0;
    }
}
#[cfg(test)]
mod tests_rug_462 {
    use super::*;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_query_with_query_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: PathAndQuery = rug_fuzz_0.parse().unwrap();
        debug_assert_eq!(p0.query(), Some("key=value&foo=bar"));
             }
});    }
    #[test]
    fn test_query_without_query_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: PathAndQuery = rug_fuzz_0.parse().unwrap();
        debug_assert!(p0.query().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_rug_463 {
    use super::*;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: PathAndQuery = rug_fuzz_0.parse().unwrap();
        debug_assert_eq!(PathAndQuery::as_str(& p0), "/hello/world?key=value&foo=bar");
        let p1: PathAndQuery = rug_fuzz_1.parse().unwrap();
        debug_assert_eq!(PathAndQuery::as_str(& p1), "/hello/world");
             }
});    }
}
#[cfg(test)]
mod tests_rug_464 {
    use super::*;
    use crate::uri::path::PathAndQuery;
    use bytes::Bytes;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_464_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/test/path";
        let p0 = PathAndQuery::from_static(rug_fuzz_0);
        PathAndQuery::into_bytes(p0);
        let _rug_ed_tests_rug_464_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_465 {
    use super::*;
    use crate::HttpTryFrom;
    use uri::path::PathAndQuery;
    use bytes::Bytes;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_465_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"sample_path";
        let mut p0: bytes::Bytes = bytes::Bytes::from_static(rug_fuzz_0);
        <PathAndQuery as HttpTryFrom<bytes::Bytes>>::try_from(p0).unwrap();
        let _rug_ed_tests_rug_465_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_466 {
    use super::*;
    use crate::HttpTryFrom;
    use uri::path::PathAndQuery;
    #[test]
    fn test_try_from() {
        let _rug_st_tests_rug_466_rrrruuuugggg_test_try_from = 0;
        let rug_fuzz_0 = b"/example/path";
        let p0: &[u8] = rug_fuzz_0;
        PathAndQuery::try_from(p0);
        let _rug_ed_tests_rug_466_rrrruuuugggg_test_try_from = 0;
    }
}
#[cfg(test)]
mod tests_rug_467 {
    use super::*;
    use crate::HttpTryFrom;
    use uri::path::PathAndQuery;
    #[test]
    fn test_try_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        let _ = <PathAndQuery as HttpTryFrom<&str>>::try_from(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_468 {
    use super::*;
    use crate::std::str::FromStr;
    use uri::path::PathAndQuery;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        <PathAndQuery as FromStr>::from_str(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_469 {
    use super::*;
    use bytes::Bytes;
    use uri::path::PathAndQuery;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_469_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/example/path";
        let p0 = PathAndQuery::from_static(rug_fuzz_0);
        let _ = <Bytes>::from(p0);
        let _rug_ed_tests_rug_469_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_470 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use uri::{PathAndQuery, path};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_470_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/test";
        let rug_fuzz_1 = "/test";
        let p0 = path::PathAndQuery::from_static(rug_fuzz_0);
        let p1 = path::PathAndQuery::from_static(rug_fuzz_1);
        debug_assert_eq!(p0.eq(& p1), true);
        let _rug_ed_tests_rug_470_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_471 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::Uri;
    use crate::uri;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = uri::PathAndQuery::slash();
        let p1 = rug_fuzz_0;
        debug_assert_eq!(< uri::PathAndQuery > ::eq(& p0, & p1), false);
             }
});    }
}
#[cfg(test)]
mod tests_rug_472 {
    use super::*;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_eq() {
        let _rug_st_tests_rug_472_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = "example/path";
        let rug_fuzz_1 = "/example/path";
        let p0: &'static str = rug_fuzz_0;
        let p1 = PathAndQuery::from_str(rug_fuzz_1).unwrap();
        debug_assert!(p0.eq(& p1));
        let _rug_ed_tests_rug_472_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_rug_473 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_473_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/example/path?query=1";
        let rug_fuzz_1 = "example/path?query=1";
        let p0 = PathAndQuery::from_static(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        debug_assert!(< PathAndQuery as PartialEq < & str > > ::eq(& p0, & & p1));
        let _rug_ed_tests_rug_473_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_474 {
    use super::*;
    use uri::path::PathAndQuery;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        let mut p1: PathAndQuery = PathAndQuery::from_str(rug_fuzz_1).unwrap();
        <str>::eq(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_475 {
    use super::*;
    use std::cmp::PartialEq;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_475_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/test";
        let rug_fuzz_1 = "/test";
        use crate::uri;
        use std::string::String;
        let p0 = uri::PathAndQuery::from_static(rug_fuzz_0);
        let p1 = String::from(rug_fuzz_1);
        debug_assert_eq!(p0.eq(& p1), true);
        let _rug_ed_tests_rug_475_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_476 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::uri::path::PathAndQuery;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: std::string::String = rug_fuzz_0.to_string();
        let p1: PathAndQuery = PathAndQuery::from_str(rug_fuzz_1).unwrap();
        debug_assert!(< std::string::String > ::eq(& p0, & p1));
             }
});    }
}
#[cfg(test)]
mod tests_rug_477 {
    use super::*;
    use crate::uri::path::PathAndQuery;
    use crate::std::cmp::PartialOrd;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_477_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/";
        let rug_fuzz_1 = "/test";
        let mut p0 = PathAndQuery::from_static(rug_fuzz_0);
        let mut p1 = PathAndQuery::from_static(rug_fuzz_1);
        p0.partial_cmp(&p1);
        let _rug_ed_tests_rug_477_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_478 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use uri::PathAndQuery;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_478_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/";
        let rug_fuzz_1 = "/example/path";
        let p0 = PathAndQuery::from_static(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        PathAndQuery::partial_cmp(&p0, &p1);
        let _rug_ed_tests_rug_478_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_479 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_rug_479_rrrruuuugggg_test_partial_cmp = 0;
        let rug_fuzz_0 = "/example/path";
        let rug_fuzz_1 = "/test/path";
        let p0: &str = rug_fuzz_0;
        let p1: PathAndQuery = PathAndQuery::from_static(rug_fuzz_1);
        <str>::partial_cmp(&p0, &p1);
        let _rug_ed_tests_rug_479_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_rug_480 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::uri;
        let p0 = uri::PathAndQuery::empty();
        let p1 = rug_fuzz_0;
        <uri::path::PathAndQuery as std::cmp::PartialOrd<&str>>::partial_cmp(&p0, &&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_481 {
    use super::*;
    use crate::uri::PathAndQuery;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_rug_481_rrrruuuugggg_test_partial_cmp = 0;
        let rug_fuzz_0 = "example_path";
        let p0: &'static str = rug_fuzz_0;
        let p1 = PathAndQuery::empty();
        <&'static str>::partial_cmp(&p0, &p1);
        let _rug_ed_tests_rug_481_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_rug_482 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use uri::{PathAndQuery, path};
    use std::cmp;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = path::PathAndQuery::slash();
        let mut p1 = String::from(rug_fuzz_0);
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_483 {
    use super::*;
    use std::cmp::Ordering;
    use crate::uri::path::PathAndQuery;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_rug_483_rrrruuuugggg_test_partial_cmp = 0;
        let rug_fuzz_0 = "/path1";
        let rug_fuzz_1 = "/path2";
        let p0 = String::from(rug_fuzz_0);
        let p1 = PathAndQuery::from_static(rug_fuzz_1);
        p0.partial_cmp(&p1);
        let _rug_ed_tests_rug_483_rrrruuuugggg_test_partial_cmp = 0;
    }
}
