//! Finds the difference between sequential versions of files.
//!
//! Based on the rsync algorithm.
//! The `BlockHashes` struct will find the differences between versions of the same file.
//! It does this through the [`diff_and_update()`](struct.BlockHashes.html#method.diff_and_update) method.
//!
//! # Example
//!
//! ```
//! use std::io::Cursor;
//! use rdiff::BlockHashes;
//!
//! let mut hash = BlockHashes::new(Cursor::new("The initial version"), 8).unwrap();
//! let diffs = hash.diff_and_update(Cursor::new("The next version")).unwrap();
//! println!("Diffs: {:?}", diffs);
//! // Outputs "Diffs: Diff{inserts: [Insert(0, The next vers)], deletes:[Delete(13, 16)]}"
//! ```
//!
//! This crate also contains methods relating to finding the differences between two strings, in the [string_diff](string_diff/index.html) module.
//! These methods can be used to refine the course differences found through the rsync method.
#![deny(missing_docs)]
extern crate crypto;
extern crate byteorder;
#[macro_use]
extern crate log;
mod window;
mod hashing;
pub mod string_diff;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::slice::Iter;
use std::fmt;
use std::mem;
use std::string::FromUtf8Error;
use byteorder::{NetworkEndian, ByteOrder};
/// Used for calculating and re-calculating the differences between two versions of the same file
///
/// See the [module level documentation](index.html) for examples on how to use this
#[derive(Debug, PartialEq)]
pub struct BlockHashes {
    hashes: HashMap<u32, Vec<(usize, [u8; 16])>>,
    block_size: usize,
    file_size: usize,
}
/// Represents an operation to insert bytes at a particular position into a file
#[derive(PartialEq)]
pub struct Insert {
    position: usize,
    data: Vec<u8>,
}
/// Represents an operation to delete a certain number of bytes at a particular position in a file
#[derive(PartialEq)]
pub struct Delete {
    position: usize,
    len: usize,
}
/// Represents a series of operations that were performed on a file to transform it into a new
/// version.
///
/// The operations are stored in file order, which means that every operation that affects
/// an earlier part of the file must be stored before an operation that affects a later part.
/// The diff also assumes that insert operations are performed prior to delete operations.
#[derive(Debug, PartialEq)]
pub struct Diff {
    inserts: Vec<Insert>,
    deletes: Vec<Delete>,
}
/// A sliding window over a reader.  This monatins an internal buffer read from the file,
/// which can be read from at any time.
struct Window<R: Read> {
    front: Vec<u8>,
    back: Vec<u8>,
    block_size: usize,
    offset: usize,
    bytes_read: usize,
    reader: R,
}
impl Diff {
    /// Creates a new `Diff`
    #[inline]
    pub fn new() -> Diff {
        Diff {
            inserts: Vec::new(),
            deletes: Vec::new(),
        }
    }
    /// Adds an insert operation into this diff.  The operation must occur after
    /// all previously added insert operations in file order.  If the operation
    /// can be merged with the previous operation, then it is.
    ///
    /// Consumes the data that is passed in
    fn add_insert(&mut self, position: usize, mut data: Vec<u8>) {
        if let Some(tail) = self.inserts.last_mut() {
            if tail.position + tail.data.len() == position {
                tail.data.append(&mut data);
                return;
            }
        }
        self.inserts
            .push(Insert {
                position: position,
                data: data,
            });
    }
    /// all previously added insert and delete operations in file order.  If the operation
    /// can be merged with the previous operation, then it is.
    fn add_delete(&mut self, position: usize, len: usize) {
        if let Some(tail) = self.deletes.last_mut() {
            if tail.position == position {
                tail.len += len;
                return;
            }
        }
        self.deletes
            .push(Delete {
                position: position,
                len: len,
            });
    }
    /// Gets an iterator over all insert operations
    pub fn inserts(&self) -> Iter<Insert> {
        self.inserts.iter()
    }
    /// Gets an iterator over all delete operations
    pub fn deletes(&self) -> Iter<Delete> {
        self.deletes.iter()
    }
    /// Checks if this set of diffs has any actual content
    pub fn is_empty(&self) -> bool {
        self.deletes.is_empty() && self.inserts.is_empty()
    }
    /// Applies all of the operations in the diff to the given string.
    /// Gives an error if the resulting string can't be represented by utf8.
    ///
    /// # Panics
    /// When the operations refer to positions that are not represented by the string.
    pub fn apply_to_string(&self, string: &str) -> Result<String, FromUtf8Error> {
        let mut old_bytes = string.bytes();
        let mut new_bytes = Vec::new();
        let mut index = 0;
        for insert in self.inserts() {
            while index < insert.position {
                new_bytes.push(old_bytes.next().unwrap().clone());
                index += 1;
            }
            new_bytes.append(&mut insert.data.clone());
            index += insert.data.len();
        }
        while let Some(byte) = old_bytes.next() {
            new_bytes.push(byte);
        }
        let old_bytes = mem::replace(&mut new_bytes, Vec::new());
        let mut old_bytes = old_bytes.into_iter();
        index = 0;
        for delete in self.deletes() {
            while index < delete.position {
                new_bytes.push(old_bytes.next().unwrap());
                index += 1;
            }
            for _ in 0..delete.len {
                old_bytes.next();
            }
        }
        while let Some(byte) = old_bytes.next() {
            new_bytes.push(byte);
        }
        String::from_utf8(new_bytes)
    }
    /// Apply the operations in this sequence to a file.  This should not be called until after
    /// the sequence has been integrated via [`Engine::integrate_remote`](struct.Engine.html#method.integrate_remote)
    /// The file must have been opened on both read and write mode (see [OpenOptions](https://doc.rust-lang.org/nightly/std/fs/struct.OpenOptions.html)).
    pub fn apply(&self, file: &mut File) -> io::Result<()> {
        let mut new_bytes = Vec::new();
        try!(file.seek(SeekFrom::Start(0)));
        let mut old_bytes = file.try_clone().unwrap().bytes();
        let mut index = 0;
        for insert in self.inserts.iter() {
            while index < insert.position {
                new_bytes.push(try!(old_bytes.next().unwrap()).clone());
                index += 1;
            }
            new_bytes.extend_from_slice(&insert.data[..]);
            index += insert.data.len();
        }
        while let Some(byte) = old_bytes.next() {
            new_bytes.push(try!(byte));
        }
        let old_bytes = mem::replace(&mut new_bytes, Vec::new());
        let mut old_bytes = old_bytes.into_iter();
        index = 0;
        for delete in self.deletes.iter() {
            while index < delete.position {
                new_bytes.push(old_bytes.next().unwrap());
                index += 1;
            }
            for _ in 0..delete.len {
                old_bytes.next();
            }
        }
        while let Some(byte) = old_bytes.next() {
            new_bytes.push(byte);
        }
        try!(file.seek(SeekFrom::Start(0)));
        try!(file.set_len(new_bytes.len() as u64));
        file.write_all(new_bytes.as_slice())
    }
    /// Compress this diff and write to `writer`.  The output can then be expanded
    /// back into an equivilent Diff using `expand_from()`
    pub fn compress_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut int_buf = [0; 4];
        NetworkEndian::write_u32(&mut int_buf, self.inserts.len() as u32);
        try!(writer.write(& mut int_buf));
        for insert in self.inserts.iter() {
            try!(insert.compress_to(writer));
        }
        NetworkEndian::write_u32(&mut int_buf, self.deletes.len() as u32);
        try!(writer.write(& mut int_buf));
        for delete in self.deletes.iter() {
            try!(delete.compress_to(writer));
        }
        Ok(())
    }
    /// Expand this diff from previously compressed data in `reader`.  The data in reader
    /// should have been written using `compress_to()`
    pub fn expand_from<R: Read>(reader: &mut R) -> io::Result<Diff> {
        let mut int_buf = [0; 4];
        trace!("Reading insert length");
        try!(reader.read_exact(& mut int_buf));
        let insert_len = NetworkEndian::read_u32(&int_buf);
        trace!("Insert length was: {}", insert_len);
        let inserts = (0..insert_len)
            .map(|_| Insert::expand_from(reader).unwrap())
            .collect();
        trace!("Read inserts");
        trace!("Reading delete length");
        try!(reader.read_exact(& mut int_buf));
        let delete_len = NetworkEndian::read_u32(&int_buf);
        trace!("Delete length was: {}", delete_len);
        let deletes = (0..delete_len)
            .map(|_| Delete::expand_from(reader).unwrap())
            .collect();
        trace!("Read deletes");
        Ok(Diff {
            inserts: inserts,
            deletes: deletes,
        })
    }
}
impl fmt::Debug for Insert {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt, "Insert({}, '{}')", self.position, String::from_utf8_lossy(& self.data)
            .replace('\r', "").replace('\n', "\\n")
        )
    }
}
impl fmt::Debug for Delete {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Delete({}, {})", self.position, self.len)
    }
}
impl Insert {
    /// Gets the byte position of this insert operation in its file
    #[inline]
    pub fn get_position(&self) -> usize {
        self.position
    }
    /// Gets the data this insert operation will insert
    #[inline]
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
    /// Compress this operation and write to `writer`.  The output can then be expanded
    /// back into an equivilent operation using `expand_from()`
    pub fn compress_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut int_buf = [0; 4];
        NetworkEndian::write_u32(&mut int_buf, self.position as u32);
        try!(writer.write(& int_buf));
        NetworkEndian::write_u32(&mut int_buf, self.data.len() as u32);
        try!(writer.write(& int_buf));
        try!(writer.write(& self.data));
        Ok(())
    }
    /// Expand this operation from previously compressed data in `reader`.  The data in reader
    /// should have been written using `compress_to()`
    pub fn expand_from<R: Read>(reader: &mut R) -> io::Result<Insert> {
        let mut int_buf = [0; 4];
        try!(reader.read_exact(& mut int_buf));
        let position = NetworkEndian::read_u32(&int_buf);
        try!(reader.read_exact(& mut int_buf));
        let data_len = NetworkEndian::read_u32(&int_buf) as usize;
        let mut data = Vec::with_capacity(data_len);
        data.resize(data_len, 0);
        try!(reader.read_exact(& mut data));
        Ok(Insert {
            position: position as usize,
            data: data,
        })
    }
}
impl Delete {
    /// Gets the byte position of this delete operation in its file
    #[inline]
    pub fn get_position(&self) -> usize {
        self.position
    }
    /// Gets the length in bytes of this delete operation
    #[inline]
    pub fn get_length(&self) -> usize {
        self.len
    }
    /// Compress this operation and write to `writer`.  The output can then be expanded
    /// back into an equivilent operation using `expand_from()`
    pub fn compress_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut int_buf = [0; 4];
        NetworkEndian::write_u32(&mut int_buf, self.position as u32);
        try!(writer.write(& int_buf));
        NetworkEndian::write_u32(&mut int_buf, self.len as u32);
        try!(writer.write(& int_buf));
        Ok(())
    }
    /// Expand this operation from previously compressed data in `reader`.  The data in reader
    /// should have been written using `compress_to()`
    pub fn expand_from<R: Read>(reader: &mut R) -> io::Result<Delete> {
        let mut int_buf = [0; 4];
        try!(reader.read_exact(& mut int_buf));
        let position = NetworkEndian::read_u32(&int_buf);
        try!(reader.read_exact(& mut int_buf));
        let len = NetworkEndian::read_u32(&int_buf);
        Ok(Delete {
            position: position as usize,
            len: len as usize,
        })
    }
}
#[cfg(test)]
mod test {
    use super::Diff;
    #[test]
    fn applying_diff_to_string() {
        let string = "Mr. and Mrs. Dursley, of number four, Privet Drive, were proud to say that they were perfectly normal, thank you very much. They were the last people you'd expect to be involved in anything strange or mysterious, because they just didn't hold with such nonsense.";
        let mut diff = Diff::new();
        diff.add_insert(2, vec![115]);
        diff.add_insert(37, vec![116, 121]);
        diff.add_insert(98, vec![97, 98]);
        diff.add_insert(253, vec![109]);
        diff.add_delete(35, 1);
        diff.add_delete(181, 34);
        diff.add_delete(219, 1);
        let result = diff.apply_to_string(string).unwrap();
        assert_eq!(
            result,
            "Mrs. and Mrs. Dursley, of number forty, Privet Drive, were proud to say that they were perfectly abnormal, thank you very much. They were the last people you'd expect to be involved, because they just didn't hold with much nonsense."
            .to_string()
        );
    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use crate::Diff;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_28_rrrruuuugggg_test_rug = 0;
        let diff = Diff::new();
        let _rug_ed_tests_rug_28_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_29 {
    use super::*;
    use crate::Diff;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Diff::new();
        let p1: usize = rug_fuzz_0;
        let mut p2: std::vec::Vec<u8> = std::vec::Vec::new();
        p0.add_insert(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::{Delete, Diff};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Diff::new();
        let p1: usize = rug_fuzz_0;
        let p2: usize = rug_fuzz_1;
        p0.add_delete(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_31 {
    use super::*;
    use crate::{Diff, Insert};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_31_rrrruuuugggg_test_rug = 0;
        let mut p0 = Diff::new();
        p0.inserts();
        let _rug_ed_tests_rug_31_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_32 {
    use super::*;
    use crate::Diff;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_32_rrrruuuugggg_test_rug = 0;
        let mut p0 = Diff::new();
        p0.deletes();
        let _rug_ed_tests_rug_32_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::Diff;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_33_rrrruuuugggg_test_rug = 0;
        let mut p0 = Diff::new();
        debug_assert_eq!(p0.is_empty(), true);
        let _rug_ed_tests_rug_33_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_34 {
    use super::*;
    use crate::Diff;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Diff::new();
        let p1 = rug_fuzz_0;
        p0.apply_to_string(&p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use std::fs::File;
    use std::io::{self, Seek, SeekFrom, Write};
    use std::io::Read;
    use std::io::BufReader;
    use std::mem;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Diff::new();
        let mut p1 = File::create(rug_fuzz_0).unwrap();
        <Diff>::apply(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use std::io::Cursor;
    use byteorder::{NetworkEndian, WriteBytesExt};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_36_rrrruuuugggg_test_rug = 0;
        let mut p0 = Diff::new();
        let mut writer = Cursor::new(Vec::new());
        p0.compress_to(&mut writer).unwrap();
        let _rug_ed_tests_rug_36_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use std::fs::File;
    use byteorder::{NetworkEndian, ReadBytesExt};
    use std::io::{Cursor, Read};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Cursor<Vec<u8>> = Cursor::new(
            vec![rug_fuzz_0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 'a' as u8],
        );
        Diff::expand_from(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use std::io::{Cursor, Read, Write};
    use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let position = rug_fuzz_0;
        let data = vec![rug_fuzz_1, 101, 108, 108, 111];
        let insert = Insert { position, data };
        debug_assert_eq!(insert.get_position(), position);
             }
});    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use byteorder::{NetworkEndian, WriteBytesExt};
    use std::io::{self, Write};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Insert {
            position: rug_fuzz_0,
            data: vec![rug_fuzz_1, 2, 3],
        };
        let mut p1 = std::io::Cursor::new(vec![]);
        p0.compress_to(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_41 {
    use super::*;
    use std::io::Read;
    use byteorder::{NetworkEndian, ReadBytesExt};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_41_rrrruuuugggg_test_rug = 0;
        let stdin = std::io::stdin();
        let stdin_lock = stdin.lock();
        let mut p0: std::io::StdinLock<'_> = stdin_lock;
        <Insert>::expand_from(&mut p0).unwrap();
        let _rug_ed_tests_rug_41_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_42 {
    use super::*;
    use crate::Delete;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Delete {
            position: rug_fuzz_0,
            len: rug_fuzz_1,
        };
        debug_assert_eq!(< Delete > ::get_position(& p0), 100);
             }
});    }
}
#[cfg(test)]
mod tests_rug_43 {
    use super::*;
    use crate::Delete;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Delete {
            position: rug_fuzz_0,
            len: rug_fuzz_1,
        };
        debug_assert_eq!(p0.get_length(), 50);
             }
});    }
}
#[cfg(test)]
mod tests_rug_44 {
    use super::*;
    use std::io::Write;
    use byteorder::{NetworkEndian, WriteBytesExt};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Delete {
            position: rug_fuzz_0,
            len: rug_fuzz_1,
        };
        let mut p1 = std::io::stdout();
        p0.compress_to(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_45 {
    use super::*;
    use std::io::Cursor;
    use byteorder::{ReadBytesExt, NetworkEndian};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: Vec<u8> = vec![rug_fuzz_0, 0, 0, 5, 0, 0, 0, 8];
        let cursor = Cursor::new(data);
        let mut p0: Cursor<Vec<u8>> = cursor;
        let result = Delete::expand_from(&mut p0).unwrap();
        debug_assert_eq!(result.position, 5);
        debug_assert_eq!(result.len, 8);
             }
});    }
}
