// Translated from C to Rust. The original C code can be found at
// https://github.com/ulfjack/ryu and carries the following license:
//
// Copyright 2018 Ulf Adams
//
// The contents of this file may be used under the terms of the Apache License,
// Version 2.0.
//
//    (See accompanying file LICENSE-Apache or copy at
//     http://www.apache.org/licenses/LICENSE-2.0)
//
// Alternatively, the contents of this file may be used under the terms of
// the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-Boost or copy at
//     https://www.boost.org/LICENSE_1_0.txt)
//
// Unless required by applicable law or agreed to in writing, this software
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.

use crate::d2s;

pub const FLOAT_POW5_INV_BITCOUNT: i32 = d2s::DOUBLE_POW5_INV_BITCOUNT - 64;
pub const FLOAT_POW5_BITCOUNT: i32 = d2s::DOUBLE_POW5_BITCOUNT - 64;

#[cfg_attr(feature = "no-panic", inline)]
fn pow5factor_32(mut value: u32) -> u32 {
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        let q = value / 5;
        let r = value % 5;
        if r != 0 {
            break;
        }
        value = q;
        count += 1;
    }
    count
}

// Returns true if value is divisible by 5^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_5_32(value: u32, p: u32) -> bool {
    pow5factor_32(value) >= p
}

// Returns true if value is divisible by 2^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool {
    // __builtin_ctz doesn't appear to be faster here.
    (value & ((1u32 << p) - 1)) == 0
}

// It seems to be slightly faster to avoid uint128_t here, although the
// generated code for uint128_t looks slightly nicer.
#[cfg_attr(feature = "no-panic", inline)]
fn mul_shift_32(m: u32, factor: u64, shift: i32) -> u32 {
    debug_assert!(shift > 32);

    // The casts here help MSVC to avoid calls to the __allmul library
    // function.
    let factor_lo = factor as u32;
    let factor_hi = (factor >> 32) as u32;
    let bits0 = m as u64 * factor_lo as u64;
    let bits1 = m as u64 * factor_hi as u64;

    let sum = (bits0 >> 32) + bits1;
    let shifted_sum = sum >> (shift - 32);
    debug_assert!(shifted_sum <= u32::max_value() as u64);
    shifted_sum as u32
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        // The inverse multipliers are defined as [2^x / 5^y] + 1; the upper 64
        // bits from the double lookup table are the correct bits for [2^x /
        // 5^y], so we have to add 1 here. Note that we rely on the fact that
        // the added 1 that's already stored in the table never overflows into
        // the upper 64 bits.
        let pow5 = unsafe { d2s::compute_inv_pow5(q) };
        mul_shift_32(m, pow5.1 + 1, j)
    }

    #[cfg(not(feature = "small"))]
    {
        debug_assert!(q < d2s::DOUBLE_POW5_INV_SPLIT.len() as u32);
        unsafe {
            mul_shift_32(
                m,
                d2s::DOUBLE_POW5_INV_SPLIT.get_unchecked(q as usize).1 + 1,
                j,
            )
        }
    }
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_div_pow2(m: u32, i: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        let pow5 = unsafe { d2s::compute_pow5(i) };
        mul_shift_32(m, pow5.1, j)
    }

    #[cfg(not(feature = "small"))]
    {
        debug_assert!(i < d2s::DOUBLE_POW5_SPLIT.len() as u32);
        unsafe { mul_shift_32(m, d2s::DOUBLE_POW5_SPLIT.get_unchecked(i as usize).1, j) }
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: u32 = 12345;
        
        crate::f2s_intrinsics::pow5factor_32(p0);
        
    }
}#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::f2s_intrinsics::pow5factor_32;

    #[test]
    fn test_rug() {
        let value: u32 = 12345;
        let p: u32 = 3;

        assert_eq!(multiple_of_power_of_5_32(value, p), pow5factor_32(value) >= p);
    }
}                    
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: u32 = 10;
        let mut p1: u32 = 3;
        
        crate::f2s_intrinsics::multiple_of_power_of_2_32(p0, p1);
    }
}
                                            
#[cfg(test)]
mod tests_rug_21 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut p0: u32 = 1234;
        let mut p1: u64 = 5678;
        let mut p2: i32 = -16;

        crate::f2s_intrinsics::mul_shift_32(p0, p1, p2);
    }
}#[cfg(test)]
mod tests_rug_22 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut p0: u32 = 123; // Sample data for the 1st argument
        let mut p1: u32 = 456; // Sample data for the 2nd argument
        let mut p2: i32 = -789; // Sample data for the 3rd argument

        crate::f2s_intrinsics::mul_pow5_inv_div_pow2(p0, p1, p2);
    }
}#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::f2s_intrinsics::mul_pow5_div_pow2;

    #[test]
    fn test_rug() {
        let mut p0: u32 = 123; // Sample data for the 1st argument
        let mut p1: u32 = 456; // Sample data for the 2nd argument
        let mut p2: i32 = -789; // Sample data for the 3rd argument
        
        mul_pow5_div_pow2(p0, p1, p2);
    }
}