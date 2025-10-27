#[allow(unused, deprecated)]
use std::ascii::AsciiExt;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use bytes::Bytes;
use byte_str::ByteStr;
use convert::HttpTryFrom;
use super::{ErrorKind, InvalidUri, InvalidUriBytes};
/// Represents the scheme component of a URI
#[derive(Clone)]
pub struct Scheme {
    pub(super) inner: Scheme2,
}
#[derive(Clone, Debug)]
pub(super) enum Scheme2<T = Box<ByteStr>> {
    None,
    Standard(Protocol),
    Other(T),
}
#[derive(Copy, Clone, Debug)]
pub(super) enum Protocol {
    Http,
    Https,
}
impl Scheme {
    /// HTTP protocol scheme
    pub const HTTP: Scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Http),
    };
    /// HTTP protocol over TLS.
    pub const HTTPS: Scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Https),
    };
    /// Attempt to convert a `Scheme` from `Bytes`
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
    /// let bytes = Bytes::from("http");
    /// let scheme = Scheme::from_shared(bytes).unwrap();
    ///
    /// assert_eq!(scheme.as_str(), "http");
    /// # }
    /// ```
    pub fn from_shared(s: Bytes) -> Result<Self, InvalidUriBytes> {
        use self::Scheme2::*;
        match Scheme2::parse_exact(&s[..]).map_err(InvalidUriBytes)? {
            None => Err(ErrorKind::InvalidScheme.into()),
            Standard(p) => Ok(Standard(p).into()),
            Other(_) => {
                let b = unsafe { ByteStr::from_utf8_unchecked(s) };
                Ok(Other(Box::new(b)).into())
            }
        }
    }
    pub(super) fn empty() -> Self {
        Scheme { inner: Scheme2::None }
    }
    /// Return a str representation of the scheme
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::uri::*;
    /// let scheme: Scheme = "http".parse().unwrap();
    /// assert_eq!(scheme.as_str(), "http");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        use self::Scheme2::*;
        use self::Protocol::*;
        match self.inner {
            Standard(Http) => "http",
            Standard(Https) => "https",
            Other(ref v) => &v[..],
            None => unreachable!(),
        }
    }
    /// Converts this `Scheme` back to a sequence of bytes
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}
impl HttpTryFrom<Bytes> for Scheme {
    type Error = InvalidUriBytes;
    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Scheme::from_shared(bytes)
    }
}
impl<'a> HttpTryFrom<&'a [u8]> for Scheme {
    type Error = InvalidUri;
    #[inline]
    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        use self::Scheme2::*;
        match Scheme2::parse_exact(s)? {
            None => Err(ErrorKind::InvalidScheme.into()),
            Standard(p) => Ok(Standard(p).into()),
            Other(_) => {
                Ok(
                    Other(Box::new(unsafe { ByteStr::from_utf8_unchecked(s.into()) }))
                        .into(),
                )
            }
        }
    }
}
impl<'a> HttpTryFrom<&'a str> for Scheme {
    type Error = InvalidUri;
    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        HttpTryFrom::try_from(s.as_bytes())
    }
}
impl FromStr for Scheme {
    type Err = InvalidUri;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HttpTryFrom::try_from(s)
    }
}
impl From<Scheme> for Bytes {
    #[inline]
    fn from(src: Scheme) -> Self {
        use self::Scheme2::*;
        use self::Protocol::*;
        match src.inner {
            None => Bytes::new(),
            Standard(Http) => Bytes::from_static(b"http"),
            Standard(Https) => Bytes::from_static(b"https"),
            Other(v) => (*v).into(),
        }
    }
}
impl fmt::Debug for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}
impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for Scheme {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl PartialEq for Scheme {
    fn eq(&self, other: &Scheme) -> bool {
        use self::Protocol::*;
        use self::Scheme2::*;
        match (&self.inner, &other.inner) {
            (&Standard(Http), &Standard(Http)) => true,
            (&Standard(Https), &Standard(Https)) => true,
            (&Other(ref a), &Other(ref b)) => a.eq_ignore_ascii_case(b),
            (&None, _) | (_, &None) => unreachable!(),
            _ => false,
        }
    }
}
impl Eq for Scheme {}
/// Case-insensitive equality
///
/// # Examples
///
/// ```
/// # use http::uri::Scheme;
/// let scheme: Scheme = "HTTP".parse().unwrap();
/// assert_eq!(scheme, *"http");
/// ```
impl PartialEq<str> for Scheme {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}
/// Case-insensitive equality
impl PartialEq<Scheme> for str {
    fn eq(&self, other: &Scheme) -> bool {
        other == self
    }
}
/// Case-insensitive hashing
impl Hash for Scheme {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self.inner {
            Scheme2::None => {}
            Scheme2::Standard(Protocol::Http) => state.write_u8(1),
            Scheme2::Standard(Protocol::Https) => state.write_u8(2),
            Scheme2::Other(ref other) => {
                other.len().hash(state);
                for &b in other.as_bytes() {
                    state.write_u8(b.to_ascii_lowercase());
                }
            }
        }
    }
}
impl<T> Scheme2<T> {
    pub(super) fn is_none(&self) -> bool {
        match *self {
            Scheme2::None => true,
            _ => false,
        }
    }
}
const MAX_SCHEME_LEN: usize = 64;
const SCHEME_CHARS: [u8; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    b'+',
    0,
    b'-',
    b'.',
    0,
    b'0',
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b':',
    0,
    0,
    0,
    0,
    0,
    0,
    b'A',
    b'B',
    b'C',
    b'D',
    b'E',
    b'F',
    b'G',
    b'H',
    b'I',
    b'J',
    b'K',
    b'L',
    b'M',
    b'N',
    b'O',
    b'P',
    b'Q',
    b'R',
    b'S',
    b'T',
    b'U',
    b'V',
    b'W',
    b'X',
    b'Y',
    b'Z',
    0,
    0,
    0,
    0,
    0,
    0,
    b'a',
    b'b',
    b'c',
    b'd',
    b'e',
    b'f',
    b'g',
    b'h',
    b'i',
    b'j',
    b'k',
    b'l',
    b'm',
    b'n',
    b'o',
    b'p',
    b'q',
    b'r',
    b's',
    b't',
    b'u',
    b'v',
    b'w',
    b'x',
    b'y',
    b'z',
    0,
    0,
    0,
    b'~',
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
impl Scheme2<usize> {
    fn parse_exact(s: &[u8]) -> Result<Scheme2<()>, InvalidUri> {
        match s {
            b"http" => Ok(Protocol::Http.into()),
            b"https" => Ok(Protocol::Https.into()),
            _ => {
                if s.len() > MAX_SCHEME_LEN {
                    return Err(ErrorKind::SchemeTooLong.into());
                }
                for &b in s {
                    match SCHEME_CHARS[b as usize] {
                        b':' => {
                            return Err(ErrorKind::InvalidScheme.into());
                        }
                        0 => {
                            return Err(ErrorKind::InvalidScheme.into());
                        }
                        _ => {}
                    }
                }
                Ok(Scheme2::Other(()))
            }
        }
    }
    pub(super) fn parse(s: &[u8]) -> Result<Scheme2<usize>, InvalidUri> {
        if s.len() >= 7 {
            if s[..7].eq_ignore_ascii_case(b"http://") {
                return Ok(Protocol::Http.into());
            }
        }
        if s.len() >= 8 {
            if s[..8].eq_ignore_ascii_case(b"https://") {
                return Ok(Protocol::Https.into());
            }
        }
        if s.len() > 3 {
            for i in 0..s.len() {
                let b = s[i];
                if i == MAX_SCHEME_LEN {
                    return Err(ErrorKind::SchemeTooLong.into());
                }
                match SCHEME_CHARS[b as usize] {
                    b':' => {
                        if s.len() < i + 3 {
                            break;
                        }
                        if &s[i + 1..i + 3] != b"//" {
                            break;
                        }
                        return Ok(Scheme2::Other(i));
                    }
                    0 => break,
                    _ => {}
                }
            }
        }
        Ok(Scheme2::None)
    }
}
impl Protocol {
    pub(super) fn len(&self) -> usize {
        match *self {
            Protocol::Http => 4,
            Protocol::Https => 5,
        }
    }
}
impl<T> From<Protocol> for Scheme2<T> {
    fn from(src: Protocol) -> Self {
        Scheme2::Standard(src)
    }
}
#[doc(hidden)]
impl From<Scheme2> for Scheme {
    fn from(src: Scheme2) -> Self {
        Scheme { inner: src }
    }
}
#[cfg(test)]
mod tests_rug_492 {
    use super::*;
    use bytes::Bytes;
    use crate::uri::Scheme;
    use crate::uri::ErrorKind;
    use crate::uri::InvalidUriBytes;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_492_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"http";
        let bytes = Bytes::from_static(rug_fuzz_0);
        let result = Scheme::from_shared(bytes);
        debug_assert_eq!(result.is_ok(), true);
        let scheme = result.unwrap();
        debug_assert_eq!(scheme.as_str(), "http");
        let _rug_ed_tests_rug_492_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_493 {
    use super::super::Scheme;
    #[test]
    fn test_empty_scheme() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let empty_scheme = Scheme::empty();
        debug_assert!(rug_fuzz_0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_494 {
    use super::*;
    use crate::uri::Scheme;
    use crate::uri::scheme::Scheme2;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Scheme::from_shared(Bytes::from(rug_fuzz_0)).unwrap();
        debug_assert_eq!(p0.as_str(), "http");
             }
});    }
}
#[cfg(test)]
mod tests_rug_495 {
    use super::*;
    use crate::uri::Scheme;
    use bytes::Bytes;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Scheme::from_shared(Bytes::from(rug_fuzz_0)).unwrap();
        crate::uri::scheme::Scheme::into_bytes(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_498 {
    use super::*;
    use crate::HttpTryFrom;
    use uri::scheme::Scheme;
    #[test]
    fn test_try_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        <Scheme as HttpTryFrom<&str>>::try_from(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_499 {
    use super::*;
    use crate::std::str::FromStr;
    use uri::scheme::Scheme;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        <Scheme as FromStr>::from_str(&p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_501 {
    use super::*;
    use crate::uri::Scheme;
    use bytes::Bytes;
    use std::str::FromStr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Scheme::from_shared(Bytes::from(rug_fuzz_0)).unwrap();
        p0.as_ref();
             }
});    }
}
#[cfg(test)]
mod tests_rug_502 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::uri::Scheme;
    use bytes::Bytes;
    use std::str::FromStr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Scheme::from_shared(Bytes::from(rug_fuzz_0)).unwrap();
        let mut p1 = Scheme::from_shared(Bytes::from(rug_fuzz_1)).unwrap();
        p0.eq(&p1);
        debug_assert_eq!(p0.eq(& p1), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_504 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::uri::Scheme;
    use bytes::Bytes;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = rug_fuzz_0;
        let mut p1 = Scheme::from_shared(Bytes::from(rug_fuzz_1)).unwrap();
        <str>::eq(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_506 {
    use super::*;
    use crate::uri::scheme::Scheme2;
    #[test]
    fn test_is_none() {
        let _rug_st_tests_rug_506_rrrruuuugggg_test_is_none = 0;
        let p0: Scheme2<()> = Scheme2::None;
        debug_assert_eq!(p0.is_none(), true);
        let _rug_ed_tests_rug_506_rrrruuuugggg_test_is_none = 0;
    }
}
#[cfg(test)]
mod tests_rug_507 {
    use super::*;
    use uri::scheme::{Scheme2, Protocol, MAX_SCHEME_LEN, ErrorKind, InvalidUri};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_507_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"http";
        let p0: &[u8] = rug_fuzz_0;
        <Scheme2<usize>>::parse_exact(p0);
        let _rug_ed_tests_rug_507_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_508 {
    use super::*;
    use crate::uri::scheme::{
        Scheme2, Protocol, MAX_SCHEME_LEN, SCHEME_CHARS, ErrorKind, InvalidUri,
    };
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_508_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"https://www.example.com";
        let mut p0: &[u8] = rug_fuzz_0;
        <Scheme2<usize>>::parse(p0);
        let _rug_ed_tests_rug_508_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_509 {
    use super::*;
    use uri::scheme::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_509_rrrruuuugggg_test_rug = 0;
        let mut p0: Protocol = Protocol::Http;
        debug_assert_eq!(p0.len(), 4);
        let _rug_ed_tests_rug_509_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_510 {
    use super::*;
    use crate::std::convert::From;
    use uri::scheme::{Scheme2, Protocol};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_510_rrrruuuugggg_test_rug = 0;
        let p0: Protocol = Protocol::Http;
        let _ = <Scheme2<Protocol>>::from(p0);
        let _rug_ed_tests_rug_510_rrrruuuugggg_test_rug = 0;
    }
}
