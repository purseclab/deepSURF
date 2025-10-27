use core::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

/// Performs addition that returns `None` instead of wrapping around on
/// overflow.
pub trait CheckedAdd: Sized + Add<Self, Output = Self> {
    /// Adds two numbers, checking for overflow. If overflow happens, `None` is
    /// returned.
    fn checked_add(&self, v: &Self) -> Option<Self>;
}

macro_rules! checked_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, v: &$t) -> Option<$t> {
                <$t>::$method(*self, *v)
            }
        }
    };
}

checked_impl!(CheckedAdd, checked_add, u8);
checked_impl!(CheckedAdd, checked_add, u16);
checked_impl!(CheckedAdd, checked_add, u32);
checked_impl!(CheckedAdd, checked_add, u64);
checked_impl!(CheckedAdd, checked_add, usize);
checked_impl!(CheckedAdd, checked_add, u128);

checked_impl!(CheckedAdd, checked_add, i8);
checked_impl!(CheckedAdd, checked_add, i16);
checked_impl!(CheckedAdd, checked_add, i32);
checked_impl!(CheckedAdd, checked_add, i64);
checked_impl!(CheckedAdd, checked_add, isize);
checked_impl!(CheckedAdd, checked_add, i128);

/// Performs subtraction that returns `None` instead of wrapping around on underflow.
pub trait CheckedSub: Sized + Sub<Self, Output = Self> {
    /// Subtracts two numbers, checking for underflow. If underflow happens,
    /// `None` is returned.
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedSub, checked_sub, u8);
checked_impl!(CheckedSub, checked_sub, u16);
checked_impl!(CheckedSub, checked_sub, u32);
checked_impl!(CheckedSub, checked_sub, u64);
checked_impl!(CheckedSub, checked_sub, usize);
checked_impl!(CheckedSub, checked_sub, u128);

checked_impl!(CheckedSub, checked_sub, i8);
checked_impl!(CheckedSub, checked_sub, i16);
checked_impl!(CheckedSub, checked_sub, i32);
checked_impl!(CheckedSub, checked_sub, i64);
checked_impl!(CheckedSub, checked_sub, isize);
checked_impl!(CheckedSub, checked_sub, i128);

/// Performs multiplication that returns `None` instead of wrapping around on underflow or
/// overflow.
pub trait CheckedMul: Sized + Mul<Self, Output = Self> {
    /// Multiplies two numbers, checking for underflow or overflow. If underflow
    /// or overflow happens, `None` is returned.
    fn checked_mul(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedMul, checked_mul, u8);
checked_impl!(CheckedMul, checked_mul, u16);
checked_impl!(CheckedMul, checked_mul, u32);
checked_impl!(CheckedMul, checked_mul, u64);
checked_impl!(CheckedMul, checked_mul, usize);
checked_impl!(CheckedMul, checked_mul, u128);

checked_impl!(CheckedMul, checked_mul, i8);
checked_impl!(CheckedMul, checked_mul, i16);
checked_impl!(CheckedMul, checked_mul, i32);
checked_impl!(CheckedMul, checked_mul, i64);
checked_impl!(CheckedMul, checked_mul, isize);
checked_impl!(CheckedMul, checked_mul, i128);

/// Performs division that returns `None` instead of panicking on division by zero and instead of
/// wrapping around on underflow and overflow.
pub trait CheckedDiv: Sized + Div<Self, Output = Self> {
    /// Divides two numbers, checking for underflow, overflow and division by
    /// zero. If any of that happens, `None` is returned.
    fn checked_div(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedDiv, checked_div, u8);
checked_impl!(CheckedDiv, checked_div, u16);
checked_impl!(CheckedDiv, checked_div, u32);
checked_impl!(CheckedDiv, checked_div, u64);
checked_impl!(CheckedDiv, checked_div, usize);
checked_impl!(CheckedDiv, checked_div, u128);

checked_impl!(CheckedDiv, checked_div, i8);
checked_impl!(CheckedDiv, checked_div, i16);
checked_impl!(CheckedDiv, checked_div, i32);
checked_impl!(CheckedDiv, checked_div, i64);
checked_impl!(CheckedDiv, checked_div, isize);
checked_impl!(CheckedDiv, checked_div, i128);

/// Performs an integral remainder that returns `None` instead of panicking on division by zero and
/// instead of wrapping around on underflow and overflow.
pub trait CheckedRem: Sized + Rem<Self, Output = Self> {
    /// Finds the remainder of dividing two numbers, checking for underflow, overflow and division
    /// by zero. If any of that happens, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::CheckedRem;
    /// use std::i32::MIN;
    ///
    /// assert_eq!(CheckedRem::checked_rem(&10, &7), Some(3));
    /// assert_eq!(CheckedRem::checked_rem(&10, &-7), Some(3));
    /// assert_eq!(CheckedRem::checked_rem(&-10, &7), Some(-3));
    /// assert_eq!(CheckedRem::checked_rem(&-10, &-7), Some(-3));
    ///
    /// assert_eq!(CheckedRem::checked_rem(&10, &0), None);
    ///
    /// assert_eq!(CheckedRem::checked_rem(&MIN, &1), Some(0));
    /// assert_eq!(CheckedRem::checked_rem(&MIN, &-1), None);
    /// ```
    fn checked_rem(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedRem, checked_rem, u8);
checked_impl!(CheckedRem, checked_rem, u16);
checked_impl!(CheckedRem, checked_rem, u32);
checked_impl!(CheckedRem, checked_rem, u64);
checked_impl!(CheckedRem, checked_rem, usize);
checked_impl!(CheckedRem, checked_rem, u128);

checked_impl!(CheckedRem, checked_rem, i8);
checked_impl!(CheckedRem, checked_rem, i16);
checked_impl!(CheckedRem, checked_rem, i32);
checked_impl!(CheckedRem, checked_rem, i64);
checked_impl!(CheckedRem, checked_rem, isize);
checked_impl!(CheckedRem, checked_rem, i128);

macro_rules! checked_impl_unary {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self) -> Option<$t> {
                <$t>::$method(*self)
            }
        }
    };
}

/// Performs negation that returns `None` if the result can't be represented.
pub trait CheckedNeg: Sized {
    /// Negates a number, returning `None` for results that can't be represented, like signed `MIN`
    /// values that can't be positive, or non-zero unsigned values that can't be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::CheckedNeg;
    /// use std::i32::MIN;
    ///
    /// assert_eq!(CheckedNeg::checked_neg(&1_i32), Some(-1));
    /// assert_eq!(CheckedNeg::checked_neg(&-1_i32), Some(1));
    /// assert_eq!(CheckedNeg::checked_neg(&MIN), None);
    ///
    /// assert_eq!(CheckedNeg::checked_neg(&0_u32), Some(0));
    /// assert_eq!(CheckedNeg::checked_neg(&1_u32), None);
    /// ```
    fn checked_neg(&self) -> Option<Self>;
}

checked_impl_unary!(CheckedNeg, checked_neg, u8);
checked_impl_unary!(CheckedNeg, checked_neg, u16);
checked_impl_unary!(CheckedNeg, checked_neg, u32);
checked_impl_unary!(CheckedNeg, checked_neg, u64);
checked_impl_unary!(CheckedNeg, checked_neg, usize);
checked_impl_unary!(CheckedNeg, checked_neg, u128);

checked_impl_unary!(CheckedNeg, checked_neg, i8);
checked_impl_unary!(CheckedNeg, checked_neg, i16);
checked_impl_unary!(CheckedNeg, checked_neg, i32);
checked_impl_unary!(CheckedNeg, checked_neg, i64);
checked_impl_unary!(CheckedNeg, checked_neg, isize);
checked_impl_unary!(CheckedNeg, checked_neg, i128);

/// Performs a left shift that returns `None` on shifts larger than
/// the type width.
pub trait CheckedShl: Sized + Shl<u32, Output = Self> {
    /// Checked shift left. Computes `self << rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// ```
    /// use num_traits::CheckedShl;
    ///
    /// let x: u16 = 0x0001;
    ///
    /// assert_eq!(CheckedShl::checked_shl(&x, 0),  Some(0x0001));
    /// assert_eq!(CheckedShl::checked_shl(&x, 1),  Some(0x0002));
    /// assert_eq!(CheckedShl::checked_shl(&x, 15), Some(0x8000));
    /// assert_eq!(CheckedShl::checked_shl(&x, 16), None);
    /// ```
    fn checked_shl(&self, rhs: u32) -> Option<Self>;
}

macro_rules! checked_shift_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, rhs: u32) -> Option<$t> {
                <$t>::$method(*self, rhs)
            }
        }
    };
}

checked_shift_impl!(CheckedShl, checked_shl, u8);
checked_shift_impl!(CheckedShl, checked_shl, u16);
checked_shift_impl!(CheckedShl, checked_shl, u32);
checked_shift_impl!(CheckedShl, checked_shl, u64);
checked_shift_impl!(CheckedShl, checked_shl, usize);
checked_shift_impl!(CheckedShl, checked_shl, u128);

checked_shift_impl!(CheckedShl, checked_shl, i8);
checked_shift_impl!(CheckedShl, checked_shl, i16);
checked_shift_impl!(CheckedShl, checked_shl, i32);
checked_shift_impl!(CheckedShl, checked_shl, i64);
checked_shift_impl!(CheckedShl, checked_shl, isize);
checked_shift_impl!(CheckedShl, checked_shl, i128);

/// Performs a right shift that returns `None` on shifts larger than
/// the type width.
pub trait CheckedShr: Sized + Shr<u32, Output = Self> {
    /// Checked shift right. Computes `self >> rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// ```
    /// use num_traits::CheckedShr;
    ///
    /// let x: u16 = 0x8000;
    ///
    /// assert_eq!(CheckedShr::checked_shr(&x, 0),  Some(0x8000));
    /// assert_eq!(CheckedShr::checked_shr(&x, 1),  Some(0x4000));
    /// assert_eq!(CheckedShr::checked_shr(&x, 15), Some(0x0001));
    /// assert_eq!(CheckedShr::checked_shr(&x, 16), None);
    /// ```
    fn checked_shr(&self, rhs: u32) -> Option<Self>;
}

checked_shift_impl!(CheckedShr, checked_shr, u8);
checked_shift_impl!(CheckedShr, checked_shr, u16);
checked_shift_impl!(CheckedShr, checked_shr, u32);
checked_shift_impl!(CheckedShr, checked_shr, u64);
checked_shift_impl!(CheckedShr, checked_shr, usize);
checked_shift_impl!(CheckedShr, checked_shr, u128);

checked_shift_impl!(CheckedShr, checked_shr, i8);
checked_shift_impl!(CheckedShr, checked_shr, i16);
checked_shift_impl!(CheckedShr, checked_shr, i32);
checked_shift_impl!(CheckedShr, checked_shr, i64);
checked_shift_impl!(CheckedShr, checked_shr, isize);
checked_shift_impl!(CheckedShr, checked_shr, i128);
#[cfg(test)]
mod tests_rug_1522 {
    use super::*;
    use crate::ops::checked::CheckedSub;
    
    #[test]
    fn test_checked_sub() {
        let p0: i16 = 42;
        let p1: i16 = 10;
        
        <i16 as CheckedSub>::checked_sub(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1529 {
    use super::*;
    use crate::ops::checked::CheckedMul;

    #[test]
    fn test_checked_mul() {
        // Sample data
        let p0: u32 = 5;
        let p1: u32 = 10;

        <u32 as CheckedMul>::checked_mul(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1534 {
    use super::*;
    use crate::ops::checked::CheckedMul;
    
    #[test]
    fn test_checked_mul() {
        let p0: i16 = 10;
        let p1: i16 = 5;
        
        <i16 as CheckedMul>::checked_mul(&p0, &p1);
    }
}        
#[cfg(test)]
mod tests_rug_1548 {
    use super::*;
    use crate::ops::checked::CheckedDiv;

    #[test]
    fn test_checked_div() {
        let p0: i64 = 10; // sample data for the first argument
        let p1: i64 = 2; // sample data for the second argument
        
        let result = <i64 as CheckedDiv>::checked_div(&p0, &p1);
        
        // assert the result here
    }
}#[cfg(test)]
mod tests_rug_1558 {
    use super::*;
    use crate::CheckedRem;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 10;
        let mut p1: i16 = 3;

        <i16 as CheckedRem>::checked_rem(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1563 {
    use super::*;
    use crate::ops::checked::CheckedNeg;

    #[test]
    fn test_checked_neg() {
        // Initialize the argument with sample data
        let p0: u8 = 10;

        // Call the target function
        u8::checked_neg(p0);
    }
}#[cfg(test)]
mod tests_rug_1564 {
    use super::*;
    use crate::CheckedNeg;
    
    #[test]
    fn test_rug() {
        let mut p0: u16 = 42;  // Sample value for the first argument
        
        <u16>::checked_neg(p0);
    }
}#[cfg(test)]
        mod tests_rug_1565 {
            use super::*;
            use crate::CheckedNeg;
            #[test]
            fn test_rug() {
                let mut p0: u32 = 10;

                <u32>::checked_neg(p0);

            }
        }#[cfg(test)]
mod tests_rug_1566 {
    use super::*;
    use crate::CheckedNeg;
    
    #[test]
    fn test_checked_neg() {
        let p0: u64 = 42;

        p0.checked_neg();
    }
}
#[cfg(test)]
mod tests_rug_1568 {
    use super::*;
    use crate::CheckedNeg;

    #[test]
    fn test_checked_neg() {
        let p0: u128 = 123456789; // Example sample data

        p0.checked_neg();
    }
}#[cfg(test)]
mod tests_rug_1569 {
use super::*;

use crate::ops::checked::CheckedNeg;

#[test]
fn test_rug() {
    let mut p0: i8 = -10;

    p0.checked_neg();

}
}#[cfg(test)]
mod tests_rug_1570 {
    use super::*;
    use crate::CheckedNeg;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 42;


        <i16>::checked_neg(p0);

    }
}
#[cfg(test)]
mod tests_rug_1572 {
    use super::*;
    use crate::ops::checked::CheckedNeg;
    
    #[test]
    fn test_checked_neg() {
        let p0: i64 = 10;
        
        p0.checked_neg();
    
        // Add assertions here
    }
}
#[cfg(test)]
mod tests_rug_1574 {
    use super::*;
    use crate::ops::checked::CheckedNeg;

    #[test]
    fn test_checked_neg() {
        let p0: i128 = 42;

        <i128 as CheckedNeg>::checked_neg(&p0);
    }
}#[cfg(test)]
mod tests_rug_1575 {
    use super::*;
    use crate::CheckedShl;

    #[test]
    fn test_checked_shl() {
        let p0: u8 = 5;
        let p1: u32 = 10;

        p0.checked_shl(p1);
    }
}        
        
#[cfg(test)]
mod tests_rug_1576 {
    use super::*;
    use crate::CheckedShl;

    #[test]
    fn test_checked_shl() {
        let p0: u16 = 10;
        let p1: u32 = 3;
        
        <u16>::checked_shl(p0, p1);
    }
}
        #[cfg(test)]
mod tests_rug_1577 {
    use super::*;
    use crate::CheckedShl;
    
    #[test]
    fn test_checked_shl() {
        let p0: u32 = 123; // Sample data for the first argument
        let p1: u32 = 5; // Sample data for the second argument
        p0.checked_shl(p1);
    }
}        
        
#[cfg(test)]
mod tests_rug_1578 {
    use super::*;
    use crate::CheckedShl;
    
    #[test]
    fn test_checked_shl() {
        let mut p0: u64 = 10;
        let mut p1: u32 = 4;
        
        p0.checked_shl(p1);

    }
}
        #[cfg(test)]
mod tests_rug_1579 {
    use super::*;
    use crate::ops::checked::CheckedShl;
    
    #[test]
    fn test_checked_shl() {
        let p0: usize = 10;
        let p1: u32 = 2;
        
        p0.checked_shl(p1);
    }
}        
#[cfg(test)]
mod tests_rug_1580 {
    use super::*;
    use crate::CheckedShl;
    
    #[test]
    fn test_checked_shl() {
        let p0: u128 = 123456789;
        let p1: u32 = 5;
        
        p0.checked_shl(p1);
    }
}
                                #[cfg(test)]
mod tests_rug_1581 {
    use super::*;
    use crate::ops::checked::CheckedShl;
    
    #[test]
    fn test_checked_shl() {
        let p0: i8 = 5;
        let p1: u32 = 2;
        
        p0.checked_shl(p1);
    }
}#[cfg(test)]
mod tests_rug_1582 {
    use super::*;
    use crate::ops::checked::CheckedShl;
    #[test]
    fn test_checked_shl() {
        let mut p0: i16 = 10;
        let mut p1: u32 = 3;
        
        p0.checked_shl(p1);

    }
}        
#[cfg(test)]
mod tests_rug_1583 {
    use super::*;
    use crate::CheckedShl;
    
    #[test]
    fn test_rug() {
        let mut p0: i32 = 10;
        let mut p1: u32 = 2;
        
        <i32>::checked_shl(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_1584 {
    use super::*;
    use crate::CheckedShl;

    #[test]
    fn test_rug() {
        let mut p0: i64 = 10;
        let mut p1: u32 = 3;
        
        p0.checked_shl(p1);
    }
}
        
#[cfg(test)]
mod tests_rug_1585 {
    use super::*;
    use crate::CheckedShl;

    #[test]
    fn test_checked_shl() {
        let mut p0: isize = 10;
        let mut p1: u32 = 2;

        p0.checked_shl(p1);
    }
}#[cfg(test)]
mod tests_rug_1586 {
    use super::*;
    use crate::CheckedShl;

    #[test]
    fn test_checked_shl() {
        let mut p0: i128 = 12345;
        let mut p1: u32 = 6;

        p0.checked_shl(p1);
    }
}#[cfg(test)]
mod tests_rug_1587 {
    use super::*;
    use crate::CheckedShr;
    
    #[test]
    fn test_rug() {
        let mut p0: u8 = 42;
        let mut p1: u32 = 8;
        
        p0.checked_shr(p1);
    }
}#[cfg(test)]
mod tests_rug_1588 {
    use super::*;
    use crate::CheckedShr;
    
    #[test]
    fn test_checked_shr() {
        let p0: u16 = 10;
        let p1: u32 = 3;
        
        p0.checked_shr(p1);
    }
}#[cfg(test)]
mod tests_rug_1589 {
    use super::*;
    use crate::CheckedShr;
    
    #[test]
    fn test_rug() {
        let mut p0: u32 = 10;
        let mut p1: u32 = 2;
        
        p0.checked_shr(p1);
    }
}#[cfg(test)]
mod tests_rug_1590 {
    use super::*;
    use crate::CheckedShr;

    #[test]
    fn test_rug() {
        let mut p0: u64 = 123;
        let mut p1: u32 = 5;

        <u64>::checked_shr(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1591 {
    use super::*;
    use crate::CheckedShr;

    #[test]
    fn test_rug() {
        let mut p0: usize = 10;
        let mut p1: u32 = 2;

        p0.checked_shr(p1);
    }
}        
#[cfg(test)]
mod tests_rug_1592 {
    use super::*;
    use crate::CheckedShr;

    #[test]
    fn test_rug() {
        let mut p0: u128 = 0xABCD1234DCBA4321;
        let mut p1: u32 = 16;

        <u128 as CheckedShr>::checked_shr(&p0, p1);
    }
}
            
#[cfg(test)]
mod tests_rug_1593 {
    use super::*;
    use crate::CheckedShr;
    
    #[test]
    fn test_rug() {
        let mut p0: i8 = 10;
        let mut p1: u32 = 2;

        p0.checked_shr(p1);
    }
}
    
#[cfg(test)]
mod tests_rug_1594 {
    use super::*;
    use crate::CheckedShr;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 10; // Sample data for the first argument
        let mut p1: u32 = 2; // Sample data for the second argument

        p0.checked_shr(p1);
    }
}
#[cfg(test)]
mod tests_rug_1595 {
    use super::*;
    use crate::CheckedShr;

    #[test]
    fn test_rug() {
        let mut p0: i32 = 42;
        let mut p1: u32 = 2;

        p0.checked_shr(p1);
    }
}        
#[cfg(test)]
mod tests_rug_1596 {
    use super::*;
    use crate::CheckedShr;
    
    #[test]
    fn test_rug() {
        let mut p0: i64 = 42;
        let mut p1: u32 = 3;

        <i64>::checked_shr(p0, p1);
    }
}
    #[cfg(test)]
mod tests_rug_1597 {
    use super::*;
    use crate::CheckedShr;

    #[test]
    fn test_rug() {
        let mut p0: isize = 7;
        let mut p1: u32 = 2;

        p0.checked_shr(p1);
    }
}        
        #[cfg(test)]
        mod tests_rug_1598 {
            use super::*;
            use crate::ops::checked::CheckedShr;
            
            #[test]
            fn test_rug() {
                let mut p0: i128 = 10;
                let mut p1: u32 = 2;
                
                p0.checked_shr(p1);
            }
        }
                            