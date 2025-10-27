use crate::digit_table::*;
use core::ptr;

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent3(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }

    debug_assert!(k < 1000);
    if k >= 100 {
        *result = b'0' + (k / 100) as u8;
        k %= 100;
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result.offset(1), 2);
        sign as usize + 3
    } else if k >= 10 {
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent2(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }

    debug_assert!(k < 100);
    if k >= 10 {
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}
#[cfg(test)]
        mod tests_rug_24 {
            use super::*;
            use std::ptr;
            
            #[test]
            fn test_rug() {
                let mut p0 = 100;
                let mut p1 = Vec::with_capacity(4);
                let p1_ptr = p1.as_mut_ptr();
                unsafe {
                    crate::pretty::exponent::write_exponent3(p0, p1_ptr);
                }
            }
        }#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::pretty::exponent::write_exponent2;
    
    #[test]
    fn test_rug() {
        let mut p0: isize = 42;
        let mut p1: *mut u8 = std::ptr::null_mut();

        // Unit test for write_exponent2
        unsafe {
            write_exponent2(p0, p1);
        }
    }
}