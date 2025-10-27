use bytes::{Bytes, BytesMut};
use std::{cmp, fmt, mem, str};
use std::error::Error;
use std::str::FromStr;
use ::convert::HttpTryFrom;
use ::error::Never;
use header::name::HeaderName;
/// Represents an HTTP header field value.
///
/// In practice, HTTP header field values are usually valid ASCII. However, the
/// HTTP spec allows for a header value to contain opaque bytes as well. In this
/// case, the header field value is not able to be represented as a string.
///
/// To handle this, the `HeaderValue` is useable as a type and can be compared
/// with strings and implements `Debug`. A `to_str` fn is provided that returns
/// an `Err` if the header value contains non visible ascii characters.
#[derive(Clone, Hash)]
pub struct HeaderValue {
    inner: Bytes,
    is_sensitive: bool,
}
/// A possible error when converting a `HeaderValue` from a string or byte
/// slice.
pub struct InvalidHeaderValue {
    _priv: (),
}
/// A possible error when converting a `HeaderValue` from a string or byte
/// slice.
#[derive(Debug)]
pub struct InvalidHeaderValueBytes(InvalidHeaderValue);
/// A possible error when converting a `HeaderValue` to a string representation.
///
/// Header field values may contain opaque bytes, in which case it is not
/// possible to represent the value as a string.
#[derive(Debug)]
pub struct ToStrError {
    _priv: (),
}
impl HeaderValue {
    /// Convert a static string to a `HeaderValue`.
    ///
    /// This function will not perform any copying, however the string is
    /// checked to ensure that no invalid characters are present. Only visible
    /// ASCII characters (32-127) are permitted.
    ///
    /// # Panics
    ///
    /// This function panics if the argument contains invalid header value
    /// characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_static("hello");
    /// assert_eq!(val, "hello");
    /// ```
    #[inline]
    pub fn from_static(src: &'static str) -> HeaderValue {
        let bytes = src.as_bytes();
        for &b in bytes {
            if !is_visible_ascii(b) {
                panic!("invalid header value");
            }
        }
        HeaderValue {
            inner: Bytes::from_static(bytes),
            is_sensitive: false,
        }
    }
    /// Attempt to convert a string to a `HeaderValue`.
    ///
    /// If the argument contains invalid header value characters, an error is
    /// returned. Only visible ASCII characters (32-127) are permitted. Use
    /// `from_bytes` to create a `HeaderValue` that includes opaque octets
    /// (128-255).
    ///
    /// This function is intended to be replaced in the future by a `TryFrom`
    /// implementation once the trait is stabilized in std.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_str("hello").unwrap();
    /// assert_eq!(val, "hello");
    /// ```
    ///
    /// An invalid value
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_str("\n");
    /// assert!(val.is_err());
    /// ```
    #[inline]
    pub fn from_str(src: &str) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::try_from(src)
    }
    /// Converts a HeaderName into a HeaderValue
    ///
    /// Since every valid HeaderName is a valid HeaderValue this is done infallibly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderValue, HeaderName};
    /// # use http::header::ACCEPT;
    /// let val = HeaderValue::from_name(ACCEPT);
    /// assert_eq!(val, HeaderValue::from_bytes(b"accept").unwrap());
    /// ```
    #[inline]
    pub fn from_name(name: HeaderName) -> HeaderValue {
        name.into()
    }
    /// Attempt to convert a byte slice to a `HeaderValue`.
    ///
    /// If the argument contains invalid header value bytes, an error is
    /// returned. Only byte values between 32 and 255 (inclusive) are permitted,
    /// excluding byte 127 (DEL).
    ///
    /// This function is intended to be replaced in the future by a `TryFrom`
    /// implementation once the trait is stabilized in std.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_bytes(b"hello\xfa").unwrap();
    /// assert_eq!(val, &b"hello\xfa"[..]);
    /// ```
    ///
    /// An invalid value
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_bytes(b"\n");
    /// assert!(val.is_err());
    /// ```
    #[inline]
    pub fn from_bytes(src: &[u8]) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::try_from(src)
    }
    /// Attempt to convert a `Bytes` buffer to a `HeaderValue`.
    ///
    /// If the argument contains invalid header value bytes, an error is
    /// returned. Only byte values between 32 and 255 (inclusive) are permitted,
    /// excluding byte 127 (DEL).
    ///
    /// This function is intended to be replaced in the future by a `TryFrom`
    /// implementation once the trait is stabilized in std.
    #[inline]
    pub fn from_shared(src: Bytes) -> Result<HeaderValue, InvalidHeaderValueBytes> {
        HeaderValue::try_from(src).map_err(InvalidHeaderValueBytes)
    }
    /// Convert a `Bytes` directly into a `HeaderValue` without validating.
    ///
    /// This function does NOT validate that illegal bytes are not contained
    /// within the buffer.
    #[inline]
    pub unsafe fn from_shared_unchecked(src: Bytes) -> HeaderValue {
        if cfg!(debug_assertions) {
            match HeaderValue::from_shared(src) {
                Ok(val) => val,
                Err(_err) => {
                    panic!("HeaderValue::from_shared_unchecked() with invalid bytes");
                }
            }
        } else {
            HeaderValue {
                inner: src,
                is_sensitive: false,
            }
        }
    }
    fn try_from<T: AsRef<[u8]> + Into<Bytes>>(
        src: T,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        for &b in src.as_ref() {
            if !is_valid(b) {
                return Err(InvalidHeaderValue { _priv: () });
            }
        }
        Ok(HeaderValue {
            inner: src.into(),
            is_sensitive: false,
        })
    }
    /// Yields a `&str` slice if the `HeaderValue` only contains visible ASCII
    /// chars.
    ///
    /// This function will perform a scan of the header value, checking all the
    /// characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_static("hello");
    /// assert_eq!(val.to_str().unwrap(), "hello");
    /// ```
    pub fn to_str(&self) -> Result<&str, ToStrError> {
        let bytes = self.as_ref();
        for &b in bytes {
            if !is_visible_ascii(b) {
                return Err(ToStrError { _priv: () });
            }
        }
        unsafe { Ok(str::from_utf8_unchecked(bytes)) }
    }
    /// Returns the length of `self`.
    ///
    /// This length is in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_static("hello");
    /// assert_eq!(val.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.as_ref().len()
    }
    /// Returns true if the `HeaderValue` has a length of zero bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_static("");
    /// assert!(val.is_empty());
    ///
    /// let val = HeaderValue::from_static("hello");
    /// assert!(!val.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Converts a `HeaderValue` to a byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let val = HeaderValue::from_static("hello");
    /// assert_eq!(val.as_bytes(), b"hello");
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
    /// Mark that the header value represents sensitive information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let mut val = HeaderValue::from_static("my secret");
    ///
    /// val.set_sensitive(true);
    /// assert!(val.is_sensitive());
    ///
    /// val.set_sensitive(false);
    /// assert!(!val.is_sensitive());
    /// ```
    #[inline]
    pub fn set_sensitive(&mut self, val: bool) {
        self.is_sensitive = val;
    }
    /// Returns `true` if the value represents sensitive data.
    ///
    /// Sensitive data could represent passwords or other data that should not
    /// be stored on disk or in memory. This setting can be used by components
    /// like caches to avoid storing the value. HPACK encoders must set the
    /// header field to never index when `is_sensitive` returns true.
    ///
    /// Note that sensitivity is not factored into equality or ordering.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::HeaderValue;
    /// let mut val = HeaderValue::from_static("my secret");
    ///
    /// val.set_sensitive(true);
    /// assert!(val.is_sensitive());
    ///
    /// val.set_sensitive(false);
    /// assert!(!val.is_sensitive());
    /// ```
    #[inline]
    pub fn is_sensitive(&self) -> bool {
        self.is_sensitive
    }
}
impl AsRef<[u8]> for HeaderValue {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}
impl fmt::Debug for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_sensitive {
            f.write_str("Sensitive")
        } else {
            f.write_str("\"")?;
            let mut from = 0;
            let bytes = self.as_bytes();
            for (i, &b) in bytes.iter().enumerate() {
                if !is_visible_ascii(b) || b == b'"' {
                    if from != i {
                        f.write_str(unsafe {
                            str::from_utf8_unchecked(&bytes[from..i])
                        })?;
                    }
                    if b == b'"' {
                        f.write_str("\\\"")?;
                    } else {
                        write!(f, "\\x{:x}", b)?;
                    }
                    from = i + 1;
                }
            }
            f.write_str(unsafe { str::from_utf8_unchecked(&bytes[from..]) })?;
            f.write_str("\"")
        }
    }
}
impl From<HeaderName> for HeaderValue {
    #[inline]
    fn from(h: HeaderName) -> HeaderValue {
        HeaderValue {
            inner: h.into(),
            is_sensitive: false,
        }
    }
}
macro_rules! from_integers {
    ($($name:ident : $t:ident => $max_len:expr),*) => {
        $(impl From <$t > for HeaderValue { fn from(num : $t) -> HeaderValue { let mut
        buf = if mem::size_of::< BytesMut > () - 1 < $max_len { if num as u64 >
        999_999_999_999_999_999 { BytesMut::with_capacity($max_len) } else {
        BytesMut::new() } } else { BytesMut::new() }; let _ = ::itoa::fmt(& mut buf,
        num); HeaderValue { inner : buf.freeze(), is_sensitive : false, } } } impl
        HttpTryFrom <$t > for HeaderValue { type Error = Never; #[inline] fn try_from(num
        : $t) -> Result < Self, Self::Error > { Ok(num.into()) } } #[test] fn $name () {
        let n : $t = 55; let val = HeaderValue::from(n); assert_eq!(val, & n
        .to_string()); let n = ::std::$t ::MAX; let val = HeaderValue::from(n);
        assert_eq!(val, & n.to_string()); })*
    };
}
from_integers! {
    from_u16 : u16 => 5, from_i16 : i16 => 6, from_u32 : u32 => 10, from_i32 : i32 => 11,
    from_u64 : u64 => 20, from_i64 : i64 => 20
}
#[cfg(target_pointer_width = "16")]
from_integers! {
    from_usize : usize => 5, from_isize : isize => 6
}
#[cfg(target_pointer_width = "32")]
from_integers! {
    from_usize : usize => 10, from_isize : isize => 11
}
#[cfg(target_pointer_width = "64")]
from_integers! {
    from_usize : usize => 20, from_isize : isize => 20
}
#[cfg(test)]
mod from_header_name_tests {
    use super::*;
    use header::map::HeaderMap;
    use header::name;
    #[test]
    fn it_can_insert_header_name_as_header_value() {
        let mut map = HeaderMap::new();
        map.insert(name::UPGRADE, name::SEC_WEBSOCKET_PROTOCOL.into());
        map.insert(
            name::ACCEPT,
            name::HeaderName::from_bytes(b"hello-world").unwrap().into(),
        );
        assert_eq!(
            map.get(name::UPGRADE).unwrap(),
            HeaderValue::from_bytes(b"sec-websocket-protocol").unwrap()
        );
        assert_eq!(
            map.get(name::ACCEPT).unwrap(), HeaderValue::from_bytes(b"hello-world")
            .unwrap()
        );
    }
}
impl FromStr for HeaderValue {
    type Err = InvalidHeaderValue;
    #[inline]
    fn from_str(s: &str) -> Result<HeaderValue, Self::Err> {
        HeaderValue::from_str(s)
    }
}
impl From<HeaderValue> for Bytes {
    #[inline]
    fn from(value: HeaderValue) -> Bytes {
        value.inner
    }
}
impl<'a> From<&'a HeaderValue> for HeaderValue {
    #[inline]
    fn from(t: &'a HeaderValue) -> Self {
        t.clone()
    }
}
impl<'a> HttpTryFrom<&'a HeaderValue> for HeaderValue {
    type Error = ::error::Never;
    #[inline]
    fn try_from(t: &'a HeaderValue) -> Result<Self, Self::Error> {
        Ok(t.clone())
    }
}
impl<'a> HttpTryFrom<&'a str> for HeaderValue {
    type Error = InvalidHeaderValue;
    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        t.parse()
    }
}
impl<'a> HttpTryFrom<&'a String> for HeaderValue {
    type Error = InvalidHeaderValue;
    #[inline]
    fn try_from(s: &'a String) -> Result<Self, Self::Error> {
        Self::from_bytes(s.as_bytes())
    }
}
impl<'a> HttpTryFrom<&'a [u8]> for HeaderValue {
    type Error = InvalidHeaderValue;
    #[inline]
    fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        HeaderValue::from_bytes(t)
    }
}
impl HttpTryFrom<String> for HeaderValue {
    type Error = InvalidHeaderValueBytes;
    #[inline]
    fn try_from(t: String) -> Result<Self, Self::Error> {
        HeaderValue::from_shared(t.into())
    }
}
impl HttpTryFrom<Bytes> for HeaderValue {
    type Error = InvalidHeaderValueBytes;
    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        HeaderValue::from_shared(bytes)
    }
}
impl HttpTryFrom<HeaderName> for HeaderValue {
    type Error = InvalidHeaderValue;
    #[inline]
    fn try_from(name: HeaderName) -> Result<Self, Self::Error> {
        Ok(name.into())
    }
}
#[cfg(test)]
mod try_from_header_name_tests {
    use super::*;
    use header::name;
    #[test]
    fn it_converts_using_try_from() {
        assert_eq!(
            HeaderValue::try_from(name::UPGRADE).unwrap(),
            HeaderValue::from_bytes(b"upgrade").unwrap()
        );
    }
}
fn is_visible_ascii(b: u8) -> bool {
    b >= 32 && b < 127 || b == b'\t'
}
#[inline]
fn is_valid(b: u8) -> bool {
    b >= 32 && b != 127 || b == b'\t'
}
impl fmt::Debug for InvalidHeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidHeaderValue").finish()
    }
}
impl fmt::Display for InvalidHeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}
impl Error for InvalidHeaderValue {
    fn description(&self) -> &str {
        "failed to parse header value"
    }
}
impl fmt::Display for InvalidHeaderValueBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl Error for InvalidHeaderValueBytes {
    fn description(&self) -> &str {
        self.0.description()
    }
}
impl fmt::Display for ToStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}
impl Error for ToStrError {
    fn description(&self) -> &str {
        "failed to convert header to a str"
    }
}
impl PartialEq for HeaderValue {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        self.inner == other.inner
    }
}
impl Eq for HeaderValue {}
impl PartialOrd for HeaderValue {
    #[inline]
    fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}
impl Ord for HeaderValue {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}
impl PartialEq<str> for HeaderValue {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.inner == other.as_bytes()
    }
}
impl PartialEq<[u8]> for HeaderValue {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.inner == other
    }
}
impl PartialOrd<str> for HeaderValue {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        (*self.inner).partial_cmp(other.as_bytes())
    }
}
impl PartialOrd<[u8]> for HeaderValue {
    #[inline]
    fn partial_cmp(&self, other: &[u8]) -> Option<cmp::Ordering> {
        (*self.inner).partial_cmp(other)
    }
}
impl PartialEq<HeaderValue> for str {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}
impl PartialEq<HeaderValue> for [u8] {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}
impl PartialOrd<HeaderValue> for str {
    #[inline]
    fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
}
impl PartialOrd<HeaderValue> for [u8] {
    #[inline]
    fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_bytes())
    }
}
impl PartialEq<String> for HeaderValue {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        *self == &other[..]
    }
}
impl PartialOrd<String> for HeaderValue {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(other.as_bytes())
    }
}
impl PartialEq<HeaderValue> for String {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}
impl PartialOrd<HeaderValue> for String {
    #[inline]
    fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
}
impl<'a> PartialEq<HeaderValue> for &'a HeaderValue {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        **self == *other
    }
}
impl<'a> PartialOrd<HeaderValue> for &'a HeaderValue {
    #[inline]
    fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        (**self).partial_cmp(other)
    }
}
impl<'a, T: ?Sized> PartialEq<&'a T> for HeaderValue
where
    HeaderValue: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &&'a T) -> bool {
        *self == **other
    }
}
impl<'a, T: ?Sized> PartialOrd<&'a T> for HeaderValue
where
    HeaderValue: PartialOrd<T>,
{
    #[inline]
    fn partial_cmp(&self, other: &&'a T) -> Option<cmp::Ordering> {
        self.partial_cmp(*other)
    }
}
impl<'a> PartialEq<HeaderValue> for &'a str {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}
impl<'a> PartialOrd<HeaderValue> for &'a str {
    #[inline]
    fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
}
#[test]
fn test_try_from() {
    HeaderValue::try_from(vec![127]).unwrap_err();
}
#[test]
fn test_debug() {
    let cases = &[
        ("hello", "\"hello\""),
        ("hello \"world\"", "\"hello \\\"world\\\"\""),
        ("\u{7FFF}hello", "\"\\xe7\\xbf\\xbfhello\""),
    ];
    for &(value, expected) in cases {
        let val = HeaderValue::from_bytes(value.as_bytes()).unwrap();
        let actual = format!("{:?}", val);
        assert_eq!(expected, actual);
    }
    let mut sensitive = HeaderValue::from_static("password");
    sensitive.set_sensitive(true);
    assert_eq!("Sensitive", format!("{:?}", sensitive));
}
#[cfg(test)]
mod tests_rug_179 {
    use super::*;
    #[test]
    fn test_is_visible_ascii() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(rug_fuzz_1, crate ::header::value::is_visible_ascii(p0));
             }
});    }
}
#[cfg(test)]
mod tests_rug_180 {
    use super::*;
    #[test]
    fn test_is_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::header::value::is_valid(p0), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_from_static() {
        let _rug_st_tests_rug_181_rrrruuuugggg_test_from_static = 0;
        let rug_fuzz_0 = "sample_str_data";
        let p0 = rug_fuzz_0;
        HeaderValue::from_static(p0);
        let _rug_ed_tests_rug_181_rrrruuuugggg_test_from_static = 0;
    }
}
#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_header_value_from_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        let result = HeaderValue::from_str(p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    use crate::header::{HeaderValue, HeaderName, ACCEPT};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_183_rrrruuuugggg_test_rug = 0;
        let p0 = HeaderName::from(ACCEPT);
        let _ = HeaderValue::from_name(p0);
        let _rug_ed_tests_rug_183_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_184 {
    use super::*;
    use crate::header::HeaderValue;
    use crate::header::InvalidHeaderValue;
    #[test]
    fn test_from_bytes() {
        let _rug_st_tests_rug_184_rrrruuuugggg_test_from_bytes = 0;
        let rug_fuzz_0 = b"hello\xfa";
        let rug_fuzz_1 = b"\n";
        let mut p0: &[u8] = rug_fuzz_0;
        let val = HeaderValue::from_bytes(p0).unwrap();
        debug_assert_eq!(val, & b"hello\xfa"[..]);
        let p1: &[u8] = rug_fuzz_1;
        let val_err = HeaderValue::from_bytes(p1);
        debug_assert!(val_err.is_err());
        let _rug_ed_tests_rug_184_rrrruuuugggg_test_from_bytes = 0;
    }
}
#[cfg(test)]
mod tests_rug_185 {
    use super::*;
    use bytes::Bytes;
    use crate::header::value::{HeaderValue, InvalidHeaderValueBytes};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_185_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"example_data";
        let mut p0: bytes::Bytes = bytes::Bytes::from_static(rug_fuzz_0);
        crate::header::value::HeaderValue::from_shared(p0).unwrap();
        let _rug_ed_tests_rug_185_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    use crate::header::HeaderName;
    #[test]
    fn test_header_value_try_from() {
        let _rug_st_tests_rug_187_rrrruuuugggg_test_header_value_try_from = 0;
        let rug_fuzz_0 = "content-length";
        let p0 = HeaderName::from_static(rug_fuzz_0);
        crate::header::value::HeaderValue::try_from(p0).unwrap();
        let _rug_ed_tests_rug_187_rrrruuuugggg_test_header_value_try_from = 0;
    }
}
#[cfg(test)]
mod tests_rug_188 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_to_str() {
        let _rug_st_tests_rug_188_rrrruuuugggg_test_to_str = 0;
        let rug_fuzz_0 = "hello";
        let val = HeaderValue::from_static(rug_fuzz_0);
        debug_assert_eq!(val.to_str().unwrap(), "hello");
        let _rug_ed_tests_rug_188_rrrruuuugggg_test_to_str = 0;
    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_len() {
        let _rug_st_tests_rug_189_rrrruuuugggg_test_len = 0;
        let rug_fuzz_0 = "hello";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        debug_assert_eq!(p0.len(), 5);
        let _rug_ed_tests_rug_189_rrrruuuugggg_test_len = 0;
    }
}
#[cfg(test)]
mod tests_rug_190 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_190_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        debug_assert!(p0.is_empty());
        let _rug_ed_tests_rug_190_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_191 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_191_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "hello";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        debug_assert_eq!(< HeaderValue > ::as_bytes(& p0), b"hello");
        let _rug_ed_tests_rug_191_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_192 {
    use super::*;
    use crate::header::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_192_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "my secret";
        let rug_fuzz_1 = true;
        let mut p0 = HeaderValue::from_static(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        p0.set_sensitive(p1);
        debug_assert!(p0.is_sensitive());
        let _rug_ed_tests_rug_192_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_193_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "my secret";
        let mut p0 = HeaderValue::from_static(rug_fuzz_0);
        debug_assert!(p0.is_sensitive());
        let _rug_ed_tests_rug_193_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_196 {
    use super::*;
    use crate::header::value::HeaderValue;
    use std::convert::From;
    use bytes::{BufMut, BytesMut};
    #[test]
    fn test_from_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let p0: u16 = num;
        <HeaderValue as std::convert::From<u16>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_197 {
    use super::*;
    use crate::{HttpTryFrom, header::value::HeaderValue};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u16 = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<u16>>::try_from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_198 {
    use super::*;
    use header::value::HeaderValue;
    use bytes::BytesMut;
    use std::convert::From;
    use itoa;
    #[test]
    fn test_from_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i16 = rug_fuzz_0;
        <HeaderValue as std::convert::From<i16>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_199 {
    use super::*;
    use crate::{HttpTryFrom, header::value::HeaderValue};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i16 = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<i16>>::try_from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_200 {
    use super::*;
    use crate::header::value::HeaderValue;
    use bytes::{BytesMut, BufMut};
    use std::mem;
    #[test]
    fn test_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u32 = rug_fuzz_0;
        <HeaderValue as std::convert::From<u32>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_201 {
    use super::*;
    use crate::header::value::HeaderValue;
    use crate::HttpTryFrom;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u32 = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<u32>>::try_from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_202 {
    use super::*;
    use crate::header::value::HeaderValue;
    use bytes::{BytesMut, BufMut};
    use itoa;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i32 = rug_fuzz_0;
        let p0 = num;
        <HeaderValue as std::convert::From<i32>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_203 {
    use super::*;
    use crate::HttpTryFrom;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<i32>>::try_from(p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_204 {
    use super::*;
    use header::value::HeaderValue;
    use bytes::{BytesMut, BufMut};
    use itoa;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        <HeaderValue as std::convert::From<u64>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_205 {
    use super::*;
    use crate::header::value::HeaderValue;
    use crate::convert::HttpTryFrom;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<u64>>::try_from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_206 {
    use super::*;
    use crate::itoa;
    use crate::header::value::HeaderValue;
    use bytes::BytesMut;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        <HeaderValue as std::convert::From<i64>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_208 {
    use super::*;
    use crate::header::value::HeaderValue;
    use crate::bytes::{BytesMut, BufMut, buf::IntoBuf};
    use crate::itoa;
    #[test]
    fn test_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        <HeaderValue as From<usize>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_209 {
    use super::*;
    use crate::HttpTryFrom;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let result = <HeaderValue as HttpTryFrom<usize>>::try_from(p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_210 {
    use super::*;
    use crate::std::convert::From;
    use header::value::HeaderValue;
    use bytes::{BytesMut, BufMut};
    use itoa;
    #[test]
    fn test_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: isize = rug_fuzz_0;
        let result = <HeaderValue as From<isize>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_211 {
    use super::*;
    use crate::header::value::HeaderValue;
    use crate::HttpTryFrom;
    #[test]
    fn test_try_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: isize = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<isize>>::try_from(p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_212 {
    use super::*;
    use crate::std::str::FromStr;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        <HeaderValue as std::str::FromStr>::from_str(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_header_value_from() {
        let _rug_st_tests_rug_214_rrrruuuugggg_test_header_value_from = 0;
        let rug_fuzz_0 = "text/plain";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        <HeaderValue as std::convert::From<&HeaderValue>>::from(&p0);
        let _rug_ed_tests_rug_214_rrrruuuugggg_test_header_value_from = 0;
    }
}
#[cfg(test)]
mod tests_rug_216 {
    use super::*;
    use crate::HttpTryFrom;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<&str>>::try_from(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_218 {
    use super::*;
    use crate::HttpTryFrom;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_218_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"example_value";
        let mut p0: &[u8] = rug_fuzz_0;
        <HeaderValue as HttpTryFrom<&[u8]>>::try_from(p0).unwrap();
        let _rug_ed_tests_rug_218_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    use crate::HttpTryFrom;
    use header::value::HeaderValue;
    #[test]
    fn test_try_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: std::string::String = rug_fuzz_0.to_string();
        <HeaderValue as HttpTryFrom<std::string::String>>::try_from(p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_220 {
    use super::*;
    use crate::HttpTryFrom;
    use bytes::Bytes;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_220_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"example_data";
        let mut p0: bytes::Bytes = bytes::Bytes::from_static(rug_fuzz_0);
        <HeaderValue>::try_from(p0);
        let _rug_ed_tests_rug_220_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_225 {
    use super::*;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_225_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "value1";
        let rug_fuzz_1 = "value1";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        let p1 = HeaderValue::from_static(rug_fuzz_1);
        debug_assert!(p0.eq(& p1));
        let _rug_ed_tests_rug_225_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_226 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_226_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "example1";
        let rug_fuzz_1 = "example2";
        let mut p0: HeaderValue = HeaderValue::from_static(rug_fuzz_0);
        let mut p1: HeaderValue = HeaderValue::from_static(rug_fuzz_1);
        p0.partial_cmp(&p1);
        let _rug_ed_tests_rug_226_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_229 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_229_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = b"example";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        debug_assert!(< HeaderValue as PartialEq < [u8] > > ::eq(& p0, p1));
        let _rug_ed_tests_rug_229_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_233 {
    use super::*;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_233_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"example_data";
        let rug_fuzz_1 = "example_value";
        let p0: &[u8] = rug_fuzz_0;
        let p1: HeaderValue = HeaderValue::from_static(rug_fuzz_1);
        debug_assert_eq!(< [u8] > ::eq(& p0, & p1), false);
        let _rug_ed_tests_rug_233_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_234 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_234_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "sample_data";
        let rug_fuzz_1 = "sample_header_value";
        let p0: &str = rug_fuzz_0;
        let p1 = HeaderValue::from_static(rug_fuzz_1);
        <str>::partial_cmp(&p0, &p1);
        let _rug_ed_tests_rug_234_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_235 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_235_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"sample_data_1";
        let rug_fuzz_1 = b"sample_value";
        let p0: &[u8] = rug_fuzz_0;
        let p1 = HeaderValue::from_bytes(rug_fuzz_1).unwrap();
        <[u8]>::partial_cmp(&p0, &p1);
        let _rug_ed_tests_rug_235_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_238 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: std::string::String = rug_fuzz_0.to_string();
        let mut p1: HeaderValue = HeaderValue::from_str(rug_fuzz_1).unwrap();
        p0.eq(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_239 {
    use super::*;
    use crate::std::cmp::PartialOrd;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: std::string::String = rug_fuzz_0.to_string();
        let mut p1: HeaderValue = HeaderValue::from_str(rug_fuzz_1).unwrap();
        p0.partial_cmp(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_240 {
    use super::*;
    use crate::std::cmp::PartialEq;
    use crate::header::value::HeaderValue;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_240_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "example1";
        let rug_fuzz_1 = "example2";
        let p0 = HeaderValue::from_static(rug_fuzz_0);
        let p1 = HeaderValue::from_static(rug_fuzz_1);
        p0.eq(&p1);
        let _rug_ed_tests_rug_240_rrrruuuugggg_test_rug = 0;
    }
}
