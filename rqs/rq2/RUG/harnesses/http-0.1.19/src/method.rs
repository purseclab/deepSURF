//! The HTTP request method
//!
//! This module contains HTTP-method related structs and errors and such. The
//! main type of this module, `Method`, is also reexported at the root of the
//! crate as `http::Method` and is intended for import through that location
//! primarily.
//!
//! # Examples
//!
//! ```
//! use http::Method;
//!
//! assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
//! assert!(Method::GET.is_idempotent());
//! assert_eq!(Method::POST.as_str(), "POST");
//! ```
use HttpTryFrom;
use self::Inner::*;
use std::{fmt, str};
use std::convert::AsRef;
use std::error::Error;
use std::str::FromStr;
/// The Request Method (VERB)
///
/// This type also contains constants for a number of common HTTP methods such
/// as GET, POST, etc.
///
/// Currently includes 8 variants representing the 8 methods defined in
/// [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH,
/// and an Extension variant for all extensions.
///
/// # Examples
///
/// ```
/// use http::Method;
///
/// assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
/// assert!(Method::GET.is_idempotent());
/// assert_eq!(Method::POST.as_str(), "POST");
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);
/// A possible error value when converting `Method` from bytes.
pub struct InvalidMethod {
    _priv: (),
}
#[derive(Clone, PartialEq, Eq, Hash)]
enum Inner {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    ExtensionInline([u8; MAX_INLINE], u8),
    ExtensionAllocated(Box<[u8]>),
}
const MAX_INLINE: usize = 15;
const METHOD_CHARS: [u8; 256] = [
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'!',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'*',
    b'+',
    b'\0',
    b'-',
    b'.',
    b'\0',
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
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
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
    b'\0',
    b'\0',
    b'\0',
    b'^',
    b'_',
    b'`',
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
    b'\0',
    b'|',
    b'\0',
    b'~',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
    b'\0',
];
impl Method {
    /// GET
    pub const GET: Method = Method(Get);
    /// POST
    pub const POST: Method = Method(Post);
    /// PUT
    pub const PUT: Method = Method(Put);
    /// DELETE
    pub const DELETE: Method = Method(Delete);
    /// HEAD
    pub const HEAD: Method = Method(Head);
    /// OPTIONS
    pub const OPTIONS: Method = Method(Options);
    /// CONNECT
    pub const CONNECT: Method = Method(Connect);
    /// PATCH
    pub const PATCH: Method = Method(Patch);
    /// TRACE
    pub const TRACE: Method = Method(Trace);
    /// Converts a slice of bytes to an HTTP method.
    pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src.len() {
            0 => Err(InvalidMethod::new()),
            3 => {
                match src {
                    b"GET" => Ok(Method(Get)),
                    b"PUT" => Ok(Method(Put)),
                    _ => Method::extension_inline(src),
                }
            }
            4 => {
                match src {
                    b"POST" => Ok(Method(Post)),
                    b"HEAD" => Ok(Method(Head)),
                    _ => Method::extension_inline(src),
                }
            }
            5 => {
                match src {
                    b"PATCH" => Ok(Method(Patch)),
                    b"TRACE" => Ok(Method(Trace)),
                    _ => Method::extension_inline(src),
                }
            }
            6 => {
                match src {
                    b"DELETE" => Ok(Method(Delete)),
                    _ => Method::extension_inline(src),
                }
            }
            7 => {
                match src {
                    b"OPTIONS" => Ok(Method(Options)),
                    b"CONNECT" => Ok(Method(Connect)),
                    _ => Method::extension_inline(src),
                }
            }
            _ => {
                if src.len() < MAX_INLINE {
                    Method::extension_inline(src)
                } else {
                    let mut data: Vec<u8> = vec![0; src.len()];
                    write_checked(src, &mut data)?;
                    Ok(Method(ExtensionAllocated(data.into_boxed_slice())))
                }
            }
        }
    }
    fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {
        let mut data: [u8; MAX_INLINE] = Default::default();
        write_checked(src, &mut data)?;
        Ok(Method(ExtensionInline(data, src.len() as u8)))
    }
    /// Whether a method is considered "safe", meaning the request is
    /// essentially read-only.
    ///
    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.1)
    /// for more words.
    pub fn is_safe(&self) -> bool {
        match self.0 {
            Get | Head | Options | Trace => true,
            _ => false,
        }
    }
    /// Whether a method is considered "idempotent", meaning the request has
    /// the same result if executed multiple times.
    ///
    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.2) for
    /// more words.
    pub fn is_idempotent(&self) -> bool {
        if self.is_safe() {
            true
        } else {
            match self.0 {
                Put | Delete => true,
                _ => false,
            }
        }
    }
    /// Return a &str representation of the HTTP method
    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            Options => "OPTIONS",
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
            Head => "HEAD",
            Trace => "TRACE",
            Connect => "CONNECT",
            Patch => "PATCH",
            ExtensionInline(ref data, len) => {
                unsafe { str::from_utf8_unchecked(&data[..len as usize]) }
            }
            ExtensionAllocated(ref data) => unsafe { str::from_utf8_unchecked(data) }
        }
    }
}
fn write_checked(src: &[u8], dst: &mut [u8]) -> Result<(), InvalidMethod> {
    for (i, &b) in src.iter().enumerate() {
        let b = METHOD_CHARS[b as usize];
        if b == 0 {
            return Err(InvalidMethod::new());
        }
        dst[i] = b;
    }
    Ok(())
}
impl AsRef<str> for Method {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<'a> PartialEq<&'a Method> for Method {
    #[inline]
    fn eq(&self, other: &&'a Method) -> bool {
        self == *other
    }
}
impl<'a> PartialEq<Method> for &'a Method {
    #[inline]
    fn eq(&self, other: &Method) -> bool {
        *self == other
    }
}
impl PartialEq<str> for Method {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
impl PartialEq<Method> for str {
    #[inline]
    fn eq(&self, other: &Method) -> bool {
        self == other.as_ref()
    }
}
impl<'a> PartialEq<&'a str> for Method {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> PartialEq<Method> for &'a str {
    #[inline]
    fn eq(&self, other: &Method) -> bool {
        *self == other.as_ref()
    }
}
impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl fmt::Display for Method {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}
impl Default for Method {
    #[inline]
    fn default() -> Method {
        Method::GET
    }
}
impl<'a> From<&'a Method> for Method {
    #[inline]
    fn from(t: &'a Method) -> Self {
        t.clone()
    }
}
impl<'a> HttpTryFrom<&'a Method> for Method {
    type Error = ::error::Never;
    #[inline]
    fn try_from(t: &'a Method) -> Result<Self, Self::Error> {
        Ok(t.clone())
    }
}
impl<'a> HttpTryFrom<&'a [u8]> for Method {
    type Error = InvalidMethod;
    #[inline]
    fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        Method::from_bytes(t)
    }
}
impl<'a> HttpTryFrom<&'a str> for Method {
    type Error = InvalidMethod;
    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        HttpTryFrom::try_from(t.as_bytes())
    }
}
impl FromStr for Method {
    type Err = InvalidMethod;
    #[inline]
    fn from_str(t: &str) -> Result<Self, Self::Err> {
        HttpTryFrom::try_from(t)
    }
}
impl InvalidMethod {
    fn new() -> InvalidMethod {
        InvalidMethod { _priv: () }
    }
}
impl fmt::Debug for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidMethod").finish()
    }
}
impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InvalidMethod {
    fn description(&self) -> &str {
        "invalid HTTP method"
    }
}
#[test]
fn test_method_eq() {
    assert_eq!(Method::GET, Method::GET);
    assert_eq!(Method::GET, "GET");
    assert_eq!(& Method::GET, "GET");
    assert_eq!("GET", Method::GET);
    assert_eq!("GET", & Method::GET);
    assert_eq!(& Method::GET, Method::GET);
    assert_eq!(Method::GET, & Method::GET);
}
#[test]
fn test_invalid_method() {
    assert!(Method::from_str("").is_err());
    assert!(Method::from_bytes(b"").is_err());
}
#[cfg(test)]
mod tests_rug_246 {
    use super::*;
    use crate::method::{write_checked, InvalidMethod};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 3], u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let p0: &[u8] = rug_fuzz_0;
        let mut p1: [u8; 3] = [rug_fuzz_1; 3];
        debug_assert!(write_checked(p0, & mut p1).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_247 {
    use super::*;
    use crate::method::{Method, InvalidMethod};
    use crate::method::{Get, Put, Post, Head, Patch, Trace, Delete, Options, Connect};
    use crate::method::{MAX_INLINE, ExtensionAllocated};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_247_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"GET";
        let p0: &[u8] = rug_fuzz_0;
        <Method>::from_bytes(p0);
        let _rug_ed_tests_rug_247_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_248 {
    use super::*;
    use crate::method::{
        Method, ExtensionInline, InvalidMethod, write_checked, MAX_INLINE,
    };
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_248_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"GET";
        let src: &[u8] = rug_fuzz_0;
        let mut data: [u8; MAX_INLINE] = Default::default();
        write_checked(src, &mut data).unwrap();
        let p0: &[u8] = &data;
        <Method>::extension_inline(p0).unwrap();
        let _rug_ed_tests_rug_248_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_249 {
    use super::*;
    use crate::Method;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_249_rrrruuuugggg_test_rug = 0;
        let mut p0 = Method::GET;
        p0.is_safe();
        let _rug_ed_tests_rug_249_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_250 {
    use super::*;
    use crate::Method;
    #[test]
    fn test_is_idempotent() {
        let _rug_st_tests_rug_250_rrrruuuugggg_test_is_idempotent = 0;
        let mut p0 = Method::GET;
        debug_assert_eq!(p0.is_idempotent(), true);
        let _rug_ed_tests_rug_250_rrrruuuugggg_test_is_idempotent = 0;
    }
}
#[cfg(test)]
mod tests_rug_251 {
    use super::*;
    use crate::Method;
    #[test]
    fn test_as_str() {
        let _rug_st_tests_rug_251_rrrruuuugggg_test_as_str = 0;
        let mut p0 = Method::GET;
        debug_assert_eq!(p0.as_str(), "GET");
        let _rug_ed_tests_rug_251_rrrruuuugggg_test_as_str = 0;
    }
}
#[cfg(test)]
mod tests_rug_252 {
    use super::*;
    use crate::Method;
    use crate::std::convert::AsRef;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_252_rrrruuuugggg_test_rug = 0;
        let mut p0 = Method::GET;
        <Method as AsRef<str>>::as_ref(&p0);
        let _rug_ed_tests_rug_252_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_253 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::Method;
    #[test]
    fn test_eq() {
        let _rug_st_tests_rug_253_rrrruuuugggg_test_eq = 0;
        let mut p0 = Method::GET;
        let mut p1 = Method::POST;
        debug_assert_eq!(p0.eq(& p0), true);
        debug_assert_eq!(p0.eq(& p1), false);
        let _rug_ed_tests_rug_253_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_rug_254 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::Method;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_254_rrrruuuugggg_test_rug = 0;
        let mut p0 = Method::GET;
        let mut p1 = Method::POST;
        p0.eq(&p1);
        let _rug_ed_tests_rug_254_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_255 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::Method;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Method::GET;
        let mut p1 = rug_fuzz_0;
        <Method as std::cmp::PartialEq<str>>::eq(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_256 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::Method;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = rug_fuzz_0;
        let mut p1 = Method::GET;
        <str>::eq(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_257 {
    use super::*;
    use crate::Method;
    use crate::std::cmp::PartialEq;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_257_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "POST";
        let mut p0 = Method::GET;
        let p1: &'static str = rug_fuzz_0;
        <Method as std::cmp::PartialEq<&str>>::eq(&p0, &&p1);
        let _rug_ed_tests_rug_257_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_259 {
    use super::*;
    use crate::method::Method;
    use std::default::Default;
    #[test]
    fn test_default_method() {
        let _rug_st_tests_rug_259_rrrruuuugggg_test_default_method = 0;
        let default_method: Method = Default::default();
        debug_assert_eq!(default_method, Method::GET);
        let _rug_ed_tests_rug_259_rrrruuuugggg_test_default_method = 0;
    }
}
#[cfg(test)]
mod tests_rug_260 {
    use super::*;
    use crate::std::convert::From;
    use crate::Method;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_260_rrrruuuugggg_test_rug = 0;
        let mut p0 = Method::GET;
        <Method>::from(&p0);
        let _rug_ed_tests_rug_260_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_261 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::Method;
    #[test]
    fn test_try_from_method() {
        let _rug_st_tests_rug_261_rrrruuuugggg_test_try_from_method = 0;
        let p0 = Method::GET;
        <Method as HttpTryFrom<&Method>>::try_from(&p0).unwrap();
        let _rug_ed_tests_rug_261_rrrruuuugggg_test_try_from_method = 0;
    }
}
#[cfg(test)]
mod tests_rug_262 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::Method;
    #[test]
    fn test_try_from() {
        let _rug_st_tests_rug_262_rrrruuuugggg_test_try_from = 0;
        let rug_fuzz_0 = b"GET";
        let p0: &[u8] = rug_fuzz_0;
        <Method as HttpTryFrom<&[u8]>>::try_from(p0).unwrap();
        let _rug_ed_tests_rug_262_rrrruuuugggg_test_try_from = 0;
    }
}
#[cfg(test)]
mod tests_rug_263 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::Method;
    #[test]
    fn test_try_from_method() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        <Method as HttpTryFrom<&str>>::try_from(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_264 {
    use super::*;
    use crate::std::str::FromStr;
    use crate::method::Method;
    #[test]
    fn test_method_from_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        <Method>::from_str(&p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_265 {
    use super::*;
    use crate::method::InvalidMethod;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_265_rrrruuuugggg_test_rug = 0;
        let invalid_method = InvalidMethod::new();
        let _rug_ed_tests_rug_265_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_266 {
    use super::*;
    use crate::std::error::Error;
    use crate::header::Keys;
    use crate::method;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_266_rrrruuuugggg_test_rug = 0;
        let p0 = method::InvalidMethod::new();
        <method::InvalidMethod as Error>::description(&p0);
        let _rug_ed_tests_rug_266_rrrruuuugggg_test_rug = 0;
    }
}
