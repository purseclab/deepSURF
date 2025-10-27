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

use core::ptr;

#[cfg_attr(feature = "no-panic", inline)]
pub fn div5(x: u64) -> u64 {
    x / 5
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn div10(x: u64) -> u64 {
    x / 10
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn div100(x: u64) -> u64 {
    x / 100
}

#[cfg_attr(feature = "no-panic", inline)]
fn pow5_factor(mut value: u64) -> u32 {
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        let q = div5(value);
        let r = (value as u32).wrapping_sub(5u32.wrapping_mul(q as u32));
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
pub fn multiple_of_power_of_5(value: u64, p: u32) -> bool {
    // I tried a case distinction on p, but there was no performance difference.
    pow5_factor(value) >= p
}

// Returns true if value is divisible by 2^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2(value: u64, p: u32) -> bool {
    debug_assert!(value != 0);
    debug_assert!(p < 64);
    // __builtin_ctzll doesn't appear to be faster here.
    (value & ((1u64 << p) - 1)) == 0
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64 {
    let b0 = m as u128 * mul.0 as u128;
    let b2 = m as u128 * mul.1 as u128;
    (((b0 >> 64) + b2) >> (j - 64)) as u64
}

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn mul_shift_all_64(
    m: u64,
    mul: &(u64, u64),
    j: u32,
    vp: *mut u64,
    vm: *mut u64,
    mm_shift: u32,
) -> u64 {
    ptr::write(vp, mul_shift_64(4 * m + 2, mul, j));
    ptr::write(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, mul, j));
    mul_shift_64(4 * m, mul, j)
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: u64 = 15;

        crate::d2s_intrinsics::div5(p0);

    }
}#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: u64 = 42;
        
        crate::d2s_intrinsics::div10(p0);
    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::d2s_intrinsics::div100;

    #[test]
    fn test_rug() {
        // Initialize the first argument with a sample data
        let p0: u64 = 5000;

        // Call the target function
        div100(p0);
    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut value: u64 = 123456789; // Sample value

        crate::d2s_intrinsics::pow5_factor(value);
    }
}#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: u64 = 1234567890;
        let mut p1: u32 = 5;
        
        crate::d2s_intrinsics::multiple_of_power_of_5(p0, p1);

    }
}#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let value: u64 = 1234;
        let p: u32 = 5;

        crate::d2s_intrinsics::multiple_of_power_of_2(value, p);
    }
}        
        
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let p0: u64 = 123456789;
        let p1: (u64, u64) = (987654321, 987654321);
        let p2: u32 = 16;
        
        crate::d2s_intrinsics::mul_shift_64(p0, &p1, p2);

    }
}
                            #[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::d2s_intrinsics::{mul_shift_64, mul_shift_all_64};
    
    #[test]
    fn test_rug() {
        let m: u64 = 42; // Sample data
        let mul: (u64, u64) = (1, 2); // Sample data
        let j: u32 = 5; // Sample data
        let vp: *mut u64 = std::ptr::null_mut(); // Sample data
        let vm: *mut u64 = std::ptr::null_mut(); // Sample data
        let mm_shift: u32 = 3; // Sample data

        unsafe {
            mul_shift_all_64(m, &mul, j, vp, vm, mm_shift);
        }
    }
}