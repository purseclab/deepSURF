#[allow(unused, deprecated)]
use std::ascii::AsciiExt;
use std::{cmp, fmt, str};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use bytes::Bytes;
use byte_str::ByteStr;
use convert::HttpTryFrom;
use super::{ErrorKind, InvalidUri, InvalidUriBytes, URI_CHARS, Port};
/// Represents the authority component of a URI.
#[derive(Clone)]
pub struct Authority {
    pub(super) data: ByteStr,
}
impl Authority {
    pub(super) fn empty() -> Self {
        Authority { data: ByteStr::new() }
    }
    /// Attempt to convert an `Authority` from `Bytes`.
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
    /// let bytes = Bytes::from("example.com");
    /// let authority = Authority::from_shared(bytes).unwrap();
    ///
    /// assert_eq!(authority.host(), "example.com");
    /// # }
    /// ```
    pub fn from_shared(s: Bytes) -> Result<Self, InvalidUriBytes> {
        let authority_end = Authority::parse_non_empty(&s[..]).map_err(InvalidUriBytes)?;
        if authority_end != s.len() {
            return Err(ErrorKind::InvalidUriChar.into());
        }
        Ok(Authority {
            data: unsafe { ByteStr::from_utf8_unchecked(s) },
        })
    }
    /// Attempt to convert an `Authority` from a static string.
    ///
    /// This function will not perform any copying, and the string will be
    /// checked if it is empty or contains an invalid character.
    ///
    /// # Panics
    ///
    /// This function panics if the argument contains invalid characters or
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::uri::Authority;
    /// let authority = Authority::from_static("example.com");
    /// assert_eq!(authority.host(), "example.com");
    /// ```
    pub fn from_static(src: &'static str) -> Self {
        let s = src.as_bytes();
        let b = Bytes::from_static(s);
        let authority_end = Authority::parse_non_empty(&b[..])
            .expect("static str is not valid authority");
        if authority_end != b.len() {
            panic!("static str is not valid authority");
        }
        Authority {
            data: unsafe { ByteStr::from_utf8_unchecked(b) },
        }
    }
    pub(super) fn parse(s: &[u8]) -> Result<usize, InvalidUri> {
        let mut colon_cnt = 0;
        let mut start_bracket = false;
        let mut end_bracket = false;
        let mut has_percent = false;
        let mut end = s.len();
        let mut at_sign_pos = None;
        for (i, &b) in s.iter().enumerate() {
            match URI_CHARS[b as usize] {
                b'/' | b'?' | b'#' => {
                    end = i;
                    break;
                }
                b':' => {
                    colon_cnt += 1;
                }
                b'[' => {
                    start_bracket = true;
                    if has_percent {
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                }
                b']' => {
                    end_bracket = true;
                    colon_cnt = 0;
                    has_percent = false;
                }
                b'@' => {
                    at_sign_pos = Some(i);
                    colon_cnt = 0;
                    has_percent = false;
                }
                0 if b == b'%' => {
                    has_percent = true;
                }
                0 => {
                    return Err(ErrorKind::InvalidUriChar.into());
                }
                _ => {}
            }
        }
        if start_bracket ^ end_bracket {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        if colon_cnt > 1 {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        if end > 0 && at_sign_pos == Some(end - 1) {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        if has_percent {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        Ok(end)
    }
    fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> {
        if s.is_empty() {
            return Err(ErrorKind::Empty.into());
        }
        Authority::parse(s)
    }
    /// Get the host of this `Authority`.
    ///
    /// The host subcomponent of authority is identified by an IP literal
    /// encapsulated within square brackets, an IPv4 address in dotted- decimal
    /// form, or a registered name.  The host subcomponent is **case-insensitive**.
    ///
    /// ```notrust
    /// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
    ///                         |---------|
    ///                              |
    ///                             host
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::uri::*;
    /// let authority: Authority = "example.org:80".parse().unwrap();
    ///
    /// assert_eq!(authority.host(), "example.org");
    /// ```
    #[inline]
    pub fn host(&self) -> &str {
        host(self.as_str())
    }
    #[deprecated(since = "0.1.14", note = "use `port_part` or `port_u16` instead")]
    #[doc(hidden)]
    pub fn port(&self) -> Option<u16> {
        self.port_u16()
    }
    /// Get the port part of this `Authority`.
    ///
    /// The port subcomponent of authority is designated by an optional port
    /// number following the host and delimited from it by a single colon (":")
    /// character. It can be turned into a decimal port number with the `as_u16`
    /// method or as a `str` with the `as_str` method.
    ///
    /// ```notrust
    /// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
    ///                                     |-|
    ///                                      |
    ///                                     port
    /// ```
    ///
    /// # Examples
    ///
    /// Authority with port
    ///
    /// ```
    /// # use http::uri::Authority;
    /// let authority: Authority = "example.org:80".parse().unwrap();
    ///
    /// let port = authority.port_part().unwrap();
    /// assert_eq!(port.as_u16(), 80);
    /// assert_eq!(port.as_str(), "80");
    /// ```
    ///
    /// Authority without port
    ///
    /// ```
    /// # use http::uri::Authority;
    /// let authority: Authority = "example.org".parse().unwrap();
    ///
    /// assert!(authority.port_part().is_none());
    /// ```
    pub fn port_part(&self) -> Option<Port<&str>> {
        let bytes = self.as_str();
        bytes.rfind(":").and_then(|i| Port::from_str(&bytes[i + 1..]).ok())
    }
    /// Get the port of this `Authority` as a `u16`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::uri::Authority;
    /// let authority: Authority = "example.org:80".parse().unwrap();
    ///
    /// assert_eq!(authority.port_u16(), Some(80));
    /// ```
    pub fn port_u16(&self) -> Option<u16> {
        self.port_part().and_then(|p| Some(p.as_u16()))
    }
    /// Return a str representation of the authority
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.data[..]
    }
    /// Converts this `Authority` back to a sequence of bytes
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}
impl AsRef<str> for Authority {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl PartialEq for Authority {
    fn eq(&self, other: &Authority) -> bool {
        self.data.eq_ignore_ascii_case(&other.data)
    }
}
impl Eq for Authority {}
/// Case-insensitive equality
///
/// # Examples
///
/// ```
/// # use http::uri::Authority;
/// let authority: Authority = "HELLO.com".parse().unwrap();
/// assert_eq!(authority, "hello.coM");
/// assert_eq!("hello.com", authority);
/// ```
impl PartialEq<str> for Authority {
    fn eq(&self, other: &str) -> bool {
        self.data.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<Authority> for str {
    fn eq(&self, other: &Authority) -> bool {
        self.eq_ignore_ascii_case(other.as_str())
    }
}
impl<'a> PartialEq<Authority> for &'a str {
    fn eq(&self, other: &Authority) -> bool {
        self.eq_ignore_ascii_case(other.as_str())
    }
}
impl<'a> PartialEq<&'a str> for Authority {
    fn eq(&self, other: &&'a str) -> bool {
        self.data.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<String> for Authority {
    fn eq(&self, other: &String) -> bool {
        self.data.eq_ignore_ascii_case(other.as_str())
    }
}
impl PartialEq<Authority> for String {
    fn eq(&self, other: &Authority) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }
}
/// Case-insensitive ordering
///
/// # Examples
///
/// ```
/// # use http::uri::Authority;
/// let authority: Authority = "DEF.com".parse().unwrap();
/// assert!(authority < "ghi.com");
/// assert!(authority > "abc.com");
/// ```
impl PartialOrd for Authority {
    fn partial_cmp(&self, other: &Authority) -> Option<cmp::Ordering> {
        let left = self.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
impl PartialOrd<str> for Authority {
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        let left = self.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
impl PartialOrd<Authority> for str {
    fn partial_cmp(&self, other: &Authority) -> Option<cmp::Ordering> {
        let left = self.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
impl<'a> PartialOrd<Authority> for &'a str {
    fn partial_cmp(&self, other: &Authority) -> Option<cmp::Ordering> {
        let left = self.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
impl<'a> PartialOrd<&'a str> for Authority {
    fn partial_cmp(&self, other: &&'a str) -> Option<cmp::Ordering> {
        let left = self.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
impl PartialOrd<String> for Authority {
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        let left = self.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
impl PartialOrd<Authority> for String {
    fn partial_cmp(&self, other: &Authority) -> Option<cmp::Ordering> {
        let left = self.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
}
/// Case-insensitive hashing
///
/// # Examples
///
/// ```
/// # use http::uri::Authority;
/// # use std::hash::{Hash, Hasher};
/// # use std::collections::hash_map::DefaultHasher;
///
/// let a: Authority = "HELLO.com".parse().unwrap();
/// let b: Authority = "hello.coM".parse().unwrap();
///
/// let mut s = DefaultHasher::new();
/// a.hash(&mut s);
/// let a = s.finish();
///
/// let mut s = DefaultHasher::new();
/// b.hash(&mut s);
/// let b = s.finish();
///
/// assert_eq!(a, b);
/// ```
impl Hash for Authority {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.data.len().hash(state);
        for &b in self.data.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}
impl HttpTryFrom<Bytes> for Authority {
    type Error = InvalidUriBytes;
    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Authority::from_shared(bytes)
    }
}
impl<'a> HttpTryFrom<&'a [u8]> for Authority {
    type Error = InvalidUri;
    #[inline]
    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        let end = Authority::parse_non_empty(s)?;
        if end != s.len() {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        Ok(Authority {
            data: unsafe { ByteStr::from_utf8_unchecked(s.into()) },
        })
    }
}
impl<'a> HttpTryFrom<&'a str> for Authority {
    type Error = InvalidUri;
    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        HttpTryFrom::try_from(s.as_bytes())
    }
}
impl FromStr for Authority {
    type Err = InvalidUri;
    fn from_str(s: &str) -> Result<Self, InvalidUri> {
        HttpTryFrom::try_from(s)
    }
}
impl From<Authority> for Bytes {
    #[inline]
    fn from(src: Authority) -> Bytes {
        src.data.into()
    }
}
impl fmt::Debug for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
fn host(auth: &str) -> &str {
    let host_port = auth
        .rsplitn(2, '@')
        .next()
        .expect("split always has at least 1 item");
    if host_port.as_bytes()[0] == b'[' {
        let i = host_port.find(']').expect("parsing should validate brackets");
        &host_port[0..i + 1]
    } else {
        host_port.split(':').next().expect("split always has at least 1 item")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_empty_string_is_error() {
        let err = Authority::parse_non_empty(b"").unwrap_err();
        assert_eq!(err.0, ErrorKind::Empty);
    }
    #[test]
    fn equal_to_self_of_same_authority() {
        let authority1: Authority = "example.com".parse().unwrap();
        let authority2: Authority = "EXAMPLE.COM".parse().unwrap();
        assert_eq!(authority1, authority2);
        assert_eq!(authority2, authority1);
    }
    #[test]
    fn not_equal_to_self_of_different_authority() {
        let authority1: Authority = "example.com".parse().unwrap();
        let authority2: Authority = "test.com".parse().unwrap();
        assert_ne!(authority1, authority2);
        assert_ne!(authority2, authority1);
    }
    #[test]
    fn equates_with_a_str() {
        let authority: Authority = "example.com".parse().unwrap();
        assert_eq!(& authority, "EXAMPLE.com");
        assert_eq!("EXAMPLE.com", & authority);
        assert_eq!(authority, "EXAMPLE.com");
        assert_eq!("EXAMPLE.com", authority);
    }
    #[test]
    fn not_equal_with_a_str_of_a_different_authority() {
        let authority: Authority = "example.com".parse().unwrap();
        assert_ne!(& authority, "test.com");
        assert_ne!("test.com", & authority);
        assert_ne!(authority, "test.com");
        assert_ne!("test.com", authority);
    }
    #[test]
    fn equates_with_a_string() {
        let authority: Authority = "example.com".parse().unwrap();
        assert_eq!(authority, "EXAMPLE.com".to_string());
        assert_eq!("EXAMPLE.com".to_string(), authority);
    }
    #[test]
    fn equates_with_a_string_of_a_different_authority() {
        let authority: Authority = "example.com".parse().unwrap();
        assert_ne!(authority, "test.com".to_string());
        assert_ne!("test.com".to_string(), authority);
    }
    #[test]
    fn compares_to_self() {
        let authority1: Authority = "abc.com".parse().unwrap();
        let authority2: Authority = "def.com".parse().unwrap();
        assert!(authority1 < authority2);
        assert!(authority2 > authority1);
    }
    #[test]
    fn compares_with_a_str() {
        let authority: Authority = "def.com".parse().unwrap();
        assert!(& authority < "ghi.com");
        assert!("ghi.com" > & authority);
        assert!(& authority > "abc.com");
        assert!("abc.com" < & authority);
        assert!(authority < "ghi.com");
        assert!("ghi.com" > authority);
        assert!(authority > "abc.com");
        assert!("abc.com" < authority);
    }
    #[test]
    fn compares_with_a_string() {
        let authority: Authority = "def.com".parse().unwrap();
        assert!(authority < "ghi.com".to_string());
        assert!("ghi.com".to_string() > authority);
        assert!(authority > "abc.com".to_string());
        assert!("abc.com".to_string() < authority);
    }
    #[test]
    fn allows_percent_in_userinfo() {
        let authority_str = "a%2f:b%2f@example.com";
        let authority: Authority = authority_str.parse().unwrap();
        assert_eq!(authority, authority_str);
    }
    #[test]
    fn rejects_percent_in_hostname() {
        let err = Authority::parse_non_empty(b"example%2f.com").unwrap_err();
        assert_eq!(err.0, ErrorKind::InvalidAuthority);
        let err = Authority::parse_non_empty(b"a%2f:b%2f@example%2f.com").unwrap_err();
        assert_eq!(err.0, ErrorKind::InvalidAuthority);
    }
    #[test]
    fn allows_percent_in_ipv6_address() {
        let authority_str = "[fe80::1:2:3:4%25eth0]";
        let result: Authority = authority_str.parse().unwrap();
        assert_eq!(result, authority_str);
    }
    #[test]
    fn rejects_percent_outside_ipv6_address() {
        let err = Authority::parse_non_empty(b"1234%20[fe80::1:2:3:4]").unwrap_err();
        assert_eq!(err.0, ErrorKind::InvalidAuthority);
        let err = Authority::parse_non_empty(b"[fe80::1:2:3:4]%20").unwrap_err();
        assert_eq!(err.0, ErrorKind::InvalidAuthority);
    }
}
#[cfg(test)]
mod tests_rug_362 {
    use super::*;
    #[test]
    fn test_host() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        debug_assert_eq!(host(& p0), "localhost");
             }
});    }
}
#[cfg(test)]
mod tests_rug_363 {
    use super::*;
    use crate::uri::authority::Authority;
    use crate::uri::authority::Authority as _;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_363_rrrruuuugggg_test_rug = 0;
        Authority::empty();
        let _rug_ed_tests_rug_363_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_364 {
    use super::*;
    use bytes::Bytes;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_364_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"example.com";
        let mut p0: Bytes = Bytes::from_static(rug_fuzz_0);
        crate::uri::authority::Authority::from_shared(p0).unwrap();
        let _rug_ed_tests_rug_364_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_365 {
    use super::*;
    use crate::uri::authority::Authority;
    #[test]
    fn test_from_static() {
        let _rug_st_tests_rug_365_rrrruuuugggg_test_from_static = 0;
        let rug_fuzz_0 = "example.com";
        let p0: &str = rug_fuzz_0;
        let authority = Authority::from_static(&p0);
        debug_assert_eq!(authority.host(), "example.com");
        let _rug_ed_tests_rug_365_rrrruuuugggg_test_from_static = 0;
    }
}
#[cfg(test)]
mod tests_rug_366 {
    use super::*;
    use crate::uri::authority::{Authority, ErrorKind, InvalidUri};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_366_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"www.example.com:8080/path";
        let p0: &[u8] = rug_fuzz_0;
        let result = Authority::parse(p0);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_366_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_367 {
    use super::*;
    use uri::authority::{Authority, InvalidUri, ErrorKind};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_367_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"example.com";
        let mut p0 = rug_fuzz_0;
        let result = Authority::parse_non_empty(p0);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_367_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_368 {
    use super::*;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let authority = Authority::from_shared(bytes).unwrap();
        debug_assert_eq!(authority.host(), "example.com");
             }
});    }
}
#[cfg(test)]
mod tests_rug_369 {
    use super::*;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        <Authority>::port(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_370 {
    use bytes::Bytes;
    use crate::uri::Authority;
    use crate::uri::Port;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let authority = Authority::from_shared(bytes).unwrap();
        let port = authority.port_part().unwrap();
        debug_assert_eq!(port.as_u16(), 80);
        debug_assert_eq!(port.as_str(), "80");
             }
});    }
}
#[cfg(test)]
mod tests_rug_371 {
    use super::*;
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
        debug_assert_eq!(p0.port_u16(), Some(80));
             }
});    }
}
#[cfg(test)]
mod tests_rug_372 {
    use super::*;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        debug_assert_eq!(rug_fuzz_1, p0.as_str());
             }
});    }
}
#[cfg(test)]
mod tests_rug_373 {
    use super::*;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        p0.into_bytes();
             }
});    }
}
#[cfg(test)]
mod tests_rug_374 {
    use super::*;
    use crate::std::convert::AsRef;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        p0.as_ref();
             }
});    }
}
#[cfg(test)]
mod tests_rug_375 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes_p0 = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes_p0).unwrap();
        let bytes_p1 = Bytes::from(rug_fuzz_1);
        let p1 = Authority::from_shared(bytes_p1).unwrap();
        debug_assert_eq!(< Authority as std::cmp::PartialEq > ::eq(& p0, & p1), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_376_prepare {
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn sample() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: &str = rug_fuzz_1;
             }
});    }
}
#[cfg(test)]
mod tests_rug_376 {
    use bytes::Bytes;
    use crate::uri::Authority;
    use crate::std::cmp::PartialEq;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: &str = rug_fuzz_1;
        <Authority as std::cmp::PartialEq<str>>::eq(&p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_377 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1 = rug_fuzz_1;
        <str>::eq(&p1, &p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_378 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_378_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "example.com";
        let rug_fuzz_1 = "example.com";
        let bytes = Bytes::from(rug_fuzz_0);
        let p1 = Authority::from_shared(bytes).unwrap();
        let p0_str: &'static str = rug_fuzz_1;
        let p0 = &p0_str;
        p0.eq(&p1);
        let _rug_ed_tests_rug_378_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_379 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: &str = rug_fuzz_1;
        p0.eq(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_380 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::uri::Authority;
    #[test]
    fn test_eq() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: String = String::from(rug_fuzz_1);
        debug_assert_eq!(p0.eq(& p1), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_381 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1 = rug_fuzz_1.to_string();
        <std::string::String>::eq(&p1, &p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_382 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes0 = Bytes::from(rug_fuzz_0);
        let bytes1 = Bytes::from(rug_fuzz_1);
        let p0 = Authority::from_shared(bytes0).unwrap();
        let p1 = Authority::from_shared(bytes1).unwrap();
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_383 {
    use super::*;
    use crate::uri::Authority;
    use bytes::Bytes;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: &str = rug_fuzz_1;
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_384 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use crate::uri::authority::Authority;
    use bytes::Bytes;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        let bytes = Bytes::from(rug_fuzz_1);
        let mut p1 = Authority::from_shared(bytes).unwrap();
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_385 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: &str = rug_fuzz_1;
        p0.partial_cmp(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_386 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1: &str = rug_fuzz_1;
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_387 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_partial_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let authority = Authority::from_shared(bytes).unwrap();
        let other = String::from(rug_fuzz_1);
        authority.partial_cmp(&other);
             }
});    }
}
#[cfg(test)]
mod tests_rug_388 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        let p1 = rug_fuzz_1.to_string();
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_389 {
    use super::*;
    use crate::std::hash::{Hash, Hasher};
    use bytes::Bytes;
    use crate::uri::Authority;
    struct IdHasher(u64);
    impl Hasher for IdHasher {
        fn write(&mut self, _: &[u8]) {
            unreachable!("TypeId calls write_u64");
        }
        #[inline]
        fn write_u64(&mut self, id: u64) {
            self.0 = id;
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }
    }
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let authority = Authority::from_shared(bytes).unwrap();
        let mut hasher = IdHasher(rug_fuzz_1);
        authority.hash(&mut hasher);
             }
});    }
}
#[cfg(test)]
mod tests_rug_391 {
    use super::*;
    use crate::HttpTryFrom;
    use crate::uri::authority::Authority;
    #[test]
    fn test_try_from_function() {
        let _rug_st_tests_rug_391_rrrruuuugggg_test_try_from_function = 0;
        let rug_fuzz_0 = b"www.example.com:8080";
        let data: &'static [u8] = rug_fuzz_0;
        Authority::try_from(data);
        let _rug_ed_tests_rug_391_rrrruuuugggg_test_try_from_function = 0;
    }
}
#[cfg(test)]
mod tests_rug_392 {
    use super::*;
    use crate::{HttpTryFrom, uri::authority::Authority};
    #[test]
    fn test_try_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        let result = <Authority as HttpTryFrom<&str>>::try_from(&p0);
        debug_assert_eq!(result.is_ok(), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_393 {
    use super::*;
    use crate::uri::authority::Authority;
    use std::str::FromStr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        <Authority as std::str::FromStr>::from_str(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_394 {
    use super::*;
    use crate::std::convert::From;
    use bytes::Bytes;
    use crate::uri::Authority;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = Bytes::from(rug_fuzz_0);
        let p0 = Authority::from_shared(bytes).unwrap();
        <bytes::Bytes>::from(p0);
             }
});    }
}
