// Copyright 2016 blake2-rfc Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::mem;
use core::slice;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Safe {}

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
    fn as_mut_bytes(&mut self) -> &mut [u8];
}

impl<T: Safe> AsBytes for [T] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<T>())
        }
    }

    #[inline]
    fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_mut_ptr() as *mut u8,
                self.len() * mem::size_of::<T>(),
            )
        }
    }
}

unsafe impl Safe for u8 {}
unsafe impl Safe for u16 {}
unsafe impl Safe for u32 {}
unsafe impl Safe for u64 {}
unsafe impl Safe for i8 {}
unsafe impl Safe for i16 {}
unsafe impl Safe for i32 {}
unsafe impl Safe for i64 {}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::as_bytes::AsBytes;
    
    #[test]
    fn test_as_bytes() {
        let mut p0: [u8; 5] = [1, 2, 3, 4, 5];

        p0.as_bytes();
    }
}#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::as_bytes::AsBytes;

    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [1, 2, 3, 4];

        <[u32]>::as_mut_bytes(&mut p0);
    }
}