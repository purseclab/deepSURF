use bytes::Bytes;
use std::{ops, str};
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
impl ByteStr {
    #[inline]
    pub fn new() -> ByteStr {
        ByteStr { bytes: Bytes::new() }
    }
    #[inline]
    pub fn from_static(val: &'static str) -> ByteStr {
        ByteStr {
            bytes: Bytes::from_static(val.as_bytes()),
        }
    }
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr {
        if cfg!(debug_assertions) {
            match str::from_utf8(&bytes) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "ByteStr::from_utf8_unchecked() with invalid bytes; error = {}, bytes = {:?}",
                        err, bytes
                    )
                }
            }
        }
        ByteStr { bytes: bytes }
    }
}
impl ops::Deref for ByteStr {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        let b: &[u8] = self.bytes.as_ref();
        unsafe { str::from_utf8_unchecked(b) }
    }
}
impl From<String> for ByteStr {
    #[inline]
    fn from(src: String) -> ByteStr {
        ByteStr { bytes: Bytes::from(src) }
    }
}
impl<'a> From<&'a str> for ByteStr {
    #[inline]
    fn from(src: &'a str) -> ByteStr {
        ByteStr { bytes: Bytes::from(src) }
    }
}
impl From<ByteStr> for Bytes {
    fn from(src: ByteStr) -> Self {
        src.bytes
    }
}
#[cfg(test)]
mod tests_rug_512 {
    use super::*;
    use crate::byte_str::ByteStr;
    use crate::byte_str::Bytes;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_512_rrrruuuugggg_test_rug = 0;
        ByteStr::new();
        let _rug_ed_tests_rug_512_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_513 {
    use super::*;
    use crate::byte_str::ByteStr;
    #[test]
    fn test_from_static() {
        let _rug_st_tests_rug_513_rrrruuuugggg_test_from_static = 0;
        let rug_fuzz_0 = "Hello, World!";
        let static_str_data: &'static str = rug_fuzz_0;
        let p0: &'static str = static_str_data;
        ByteStr::from_static(p0);
        let _rug_ed_tests_rug_513_rrrruuuugggg_test_from_static = 0;
    }
}
#[cfg(test)]
mod tests_rug_515 {
    use super::*;
    use crate::byte_str::ByteStr;
    use crate::std::ops::Deref;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_515_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "sample_data";
        let mut p0 = ByteStr::from_static(rug_fuzz_0);
        p0.deref();
        let _rug_ed_tests_rug_515_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_516 {
    use super::*;
    use crate::std::convert::From;
    use crate::byte_str::{ByteStr, Bytes};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: std::string::String = String::from(rug_fuzz_0);
        <ByteStr as std::convert::From<std::string::String>>::from(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_518 {
    use super::*;
    use crate::byte_str::ByteStr;
    use bytes::Bytes;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_518_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "test_data";
        let p0 = ByteStr::from_static(rug_fuzz_0);
        let _result = <Bytes>::from(p0);
        let _rug_ed_tests_rug_518_rrrruuuugggg_test_rug = 0;
    }
}
