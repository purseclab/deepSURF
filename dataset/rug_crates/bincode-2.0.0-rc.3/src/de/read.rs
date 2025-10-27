//! This module contains reader-based structs and traits.
//!
//! Because `std::io::Read` is only limited to `std` and not `core`, we provide 2 alternative readers.
//!
//! [Reader] is a reader for sources that do not own their data. It is assumed that the reader's data is dropped after the `read` method is called. This reader is incapable of reading borrowed data, like `&str` and `&[u8]`.
//!
//! [BorrowReader] is an extension of `Reader` that also allows returning borrowed data. A `BorrowReader` allows reading `&str` and `&[u8]`.
//!
//! Specifically the `Reader` trait is used by [Decode] and the `BorrowReader` trait is used by `[BorrowDecode]`.
//!
//! [Decode]: ../trait.Decode.html
//! [BorrowDecode]: ../trait.BorrowDecode.html

use crate::error::DecodeError;

/// A reader for owned data. See the module documentation for more information.
pub trait Reader {
    /// Fill the given `bytes` argument with values. Exactly the length of the given slice must be filled, or else an error must be returned.
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError>;

    /// If this reader wraps a buffer of any kind, this function lets callers access contents of
    /// the buffer without passing data through a buffer first.
    #[inline]
    fn peek_read(&mut self, _: usize) -> Option<&[u8]> {
        None
    }

    /// If an implementation of `peek_read` is provided, an implementation of this function
    /// must be provided so that subsequent reads or peek-reads do not return the same bytes
    #[inline]
    fn consume(&mut self, _: usize) {}
}

impl<T> Reader for &mut T
where
    T: Reader,
{
    #[inline]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        (**self).read(bytes)
    }

    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&[u8]> {
        (**self).peek_read(n)
    }

    #[inline]
    fn consume(&mut self, n: usize) {
        (*self).consume(n)
    }
}

/// A reader for borrowed data. Implementors of this must also implement the [Reader] trait. See the module documentation for more information.
pub trait BorrowReader<'storage>: Reader {
    /// Read exactly `length` bytes and return a slice to this data. If not enough bytes could be read, an error should be returned.
    ///
    /// *note*: Exactly `length` bytes must be returned. If less bytes are returned, bincode may panic. If more bytes are returned, the excess bytes may be discarded.
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError>;
}

/// A reader type for `&[u8]` slices. Implements both [Reader] and [BorrowReader], and thus can be used for borrowed data.
pub struct SliceReader<'storage> {
    pub(crate) slice: &'storage [u8],
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }
}

impl<'storage> Reader for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        if bytes.len() > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd {
                additional: bytes.len() - self.slice.len(),
            });
        }
        let (read_slice, remaining) = self.slice.split_at(bytes.len());
        bytes.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(())
    }

    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&'storage [u8]> {
        self.slice.get(..n)
    }

    #[inline]
    fn consume(&mut self, n: usize) {
        self.slice = self.slice.get(n..).unwrap_or_default();
    }
}

impl<'storage> BorrowReader<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError> {
        if length > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd {
                additional: length - self.slice.len(),
            });
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }
}
#[cfg(test)]
mod tests_rug_292 {
    use super::*;

    #[test]
    fn test_peek_read() {
        use std::io::BufReader;

        let data: &[u8] = &[1, 2, 3, 4, 5];
        let v78 = BufReader::new(data);
        
        let mut p0 = v78;
        let p1: usize = 10;

        crate::de::read::Reader::peek_read(&mut p0, p1);
    }
}#[cfg(test)]
mod tests_rug_293 {
    use super::*;
    use crate::de::read::Reader;

    #[test]
    fn test_rug() {
        let mut p0: crate::de::read::SliceReader<'_> = crate::de::read::SliceReader::new(&[0u8, 1u8, 2u8]);
        let p1: usize = 10;

        crate::de::read::Reader::consume(&mut p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_294 {
    use super::*;
    use crate::de::read::Reader;
    #[test]
    fn test_rug() {
        use std::io::BufReader;

        let data: &[u8] = &[1, 2, 3, 4, 5];

        let mut p0 = BufReader::new(data);
        let mut p1: &mut [u8] = &mut [0u8; 5];

        p0.read(p1).unwrap();

    }
}

#[cfg(test)]
mod tests_rug_295 {
    use super::*;
    use crate::de::read::Reader;
    use std::io::BufReader;

    #[test]
    fn test_rug() {
        let mut p0: BufReader<&[u8]> = BufReader::new(&[1, 2, 3, 4, 5]);

        let mut p1: usize = 3;

        p0.peek_read(p1);
    }
}
                            
#[cfg(test)]
mod tests_rug_296 {
    use super::*;
    use crate::de::read::Reader;
    use std::io::BufReader;
    
    #[test]
    fn test_rug() {
        let mut p0: BufReader<&[u8]> = BufReader::new(&[1, 2, 3, 4, 5]);
        let p1: usize = 10;
        
        p0.consume(p1);
    }
}
#[cfg(test)]
mod tests_rug_297 {
    use super::*;
    use crate::de::read::SliceReader;

    #[test]
    fn test_rug() {
        let p0: &[u8] = &[1, 2, 3];
        SliceReader::new(p0);
    }
}#[cfg(test)]
mod tests_rug_298 {
    use super::*;
    use crate::de::read::Reader;
    use crate::de::read::SliceReader;

    #[test]
    fn test_read() {
        let mut p0: SliceReader<'static> = SliceReader::new(&[]); // sample
        let mut p1: [u8; 10] = [0; 10]; // sample

        p0.read(&mut p1).unwrap();
    }
}#[cfg(test)]
mod tests_rug_299 {
    use super::*;
    use crate::de::read::Reader;
    use crate::de::read::SliceReader;

    #[test]
    fn test_rug() {
        let mut p0: SliceReader<'static> = SliceReader::new(&[]);
        let p1: usize = 10;

        p0.peek_read(p1);
    }
}#[cfg(test)]
mod tests_rug_300 {
    use super::*;
    use crate::de::read::Reader;
    use crate::de::read::SliceReader;
    
    #[test]
    fn test_rug() {
        let mut p0: SliceReader<'static> = SliceReader::new(&[]);
        let p1: usize = 10;
        
        p0.consume(p1);
    }
}#[cfg(test)]
mod tests_rug_301 {
    use super::*;
    use crate::de::read::{BorrowReader, DecodeError};
    use crate::de::read::SliceReader;

    #[test]
    fn test_rug() {
        let mut p0: SliceReader<'static> = SliceReader::new(&[]);
        let p1: usize = 10;

        <SliceReader<'static> as BorrowReader<'static>>::take_bytes(&mut p0, p1).unwrap();
    }
}