use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";

/// Safe API for formatting floating point numbers to text.
///
/// ## Example
///
/// ```
/// let mut buffer = ryu::Buffer::new();
/// let printed = buffer.format_finite(1.234);
/// assert_eq!(printed, "1.234");
/// ```
pub struct Buffer {
    bytes: [MaybeUninit<u8>; 24],
}

impl Buffer {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function formats NaN as the string "NaN", positive infinity as
    /// "inf", and negative infinity as "-inf" to match std::fmt.
    ///
    /// If your input is known to be finite, you may get better performance by
    /// calling the `format_finite` method instead of `format` to avoid the
    /// checks for special cases.
    #[cfg_attr(feature = "no-panic", inline)]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, f: F) -> &str {
        if f.is_nonfinite() {
            f.format_nonfinite()
        } else {
            self.format_finite(f)
        }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function **does not** check for NaN or infinity. If the input
    /// number is not a finite float, the printed representation will be some
    /// correctly formatted but unspecified numerical value.
    ///
    /// Please check [`is_finite`] yourself before calling this function, or
    /// check [`is_nan`] and [`is_infinite`] and handle those cases yourself.
    ///
    /// [`is_finite`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_finite
    /// [`is_nan`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_nan
    /// [`is_infinite`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_infinite
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
        unsafe {
            let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
            debug_assert!(n <= self.bytes.len());
            let slice = slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
            str::from_utf8_unchecked(slice)
        }
    }
}

impl Copy for Buffer {}

impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        Buffer::new()
    }
}

impl Default for Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn default() -> Self {
        Buffer::new()
    }
}

/// A floating point number, f32 or f64, that can be written into a
/// [`ryu::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of the
/// `ryu` crate.
pub trait Float: Sealed {}
impl Float for f32 {}
impl Float for f64 {}

pub trait Sealed: Copy {
    fn is_nonfinite(self) -> bool;
    fn format_nonfinite(self) -> &'static str;
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize;
}

impl Sealed for f32 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u32 = 0x007fffff;
        const SIGN_MASK: u32 = 0x80000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format32(self, result)
    }
}

impl Sealed for f64 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u64 = 0x000fffffffffffff;
        const SIGN_MASK: u64 = 0x8000000000000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format64(self, result)
    }
}

#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::buffer::Buffer;
    use std::mem::MaybeUninit;

    #[test]
    fn test_rug() {
        let buffer: Buffer = Buffer::new();
        // add assertions here
    }
}
                    
#[cfg(test)]
mod tests_rug_31 {

    use crate::Buffer;
    use crate::Float;
    
    #[test]
    fn test_rug() {
        let mut p0 = Buffer::new();
        let p1 : f64 = 3.14;
        
        p0.format(p1);
    }
}
#[cfg(test)]
mod tests_rug_32 {
    use super::*;
    use crate::Buffer;
    use std::slice;
    use std::str;
    
    #[test]
    fn test_rug() {
        let mut p0: Buffer = Buffer::new();
        let p1: f64 = 3.14159;
        
        p0.format_finite(p1);
    }
}#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::Buffer;
    
    #[test]
    fn test_rug() {
        let mut p0: Buffer = Buffer::new();
        <Buffer as std::clone::Clone>::clone(&p0);
    }
}                        
#[cfg(test)]
mod tests_rug_34 {
    use super::*;
    use crate::buffer::Buffer;
    use std::default::Default;
    
    #[test]
    fn test_rug() {
        <Buffer as Default>::default();
    }
}
            #[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_rug() {
        let p0: f32 = 3.14;

        <f32>::is_nonfinite(p0);
    }
}#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 3.14;

        <f32>::format_nonfinite(p0);
    }
}#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 3.14159;
        let mut p1: *mut u8 = std::ptr::null_mut();

        unsafe {
            <f32 as crate::buffer::Sealed>::write_to_ryu_buffer(p0, p1);
        }
    }
}#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::buffer::Sealed;
    
    #[test]
    fn test_rug() {
        let p0: f64 = 3.14159;
        
        <f64>::is_nonfinite(p0);
    }
}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 3.14159;

        <f64>::format_nonfinite(p0);
    }
}

#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_rug() {
        let f: f64 = 3.1415926; // Sample f64 data

        let mut buffer: [u8; 10] = [0; 10]; // Sample buffer to hold the result

        let result = unsafe { f.write_to_ryu_buffer(buffer.as_mut_ptr()) }; // Calling the function

        // Add assertions as necessary
        assert_eq!(result, 0); // Sample assertion
    }
}
