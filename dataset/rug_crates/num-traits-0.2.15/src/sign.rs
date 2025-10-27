use core::num::Wrapping;
use core::ops::Neg;

use crate::float::FloatCore;
use crate::Num;

/// Useful functions for signed numbers (i.e. numbers that can be negative).
pub trait Signed: Sized + Num + Neg<Output = Self> {
    /// Computes the absolute value.
    ///
    /// For `f32` and `f64`, `NaN` will be returned if the number is `NaN`.
    ///
    /// For signed integers, `::MIN` will be returned if the number is `::MIN`.
    fn abs(&self) -> Self;

    /// The positive difference of two numbers.
    ///
    /// Returns `zero` if the number is less than or equal to `other`, otherwise the difference
    /// between `self` and `other` is returned.
    fn abs_sub(&self, other: &Self) -> Self;

    /// Returns the sign of the number.
    ///
    /// For `f32` and `f64`:
    ///
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// * `NaN` if the number is `NaN`
    ///
    /// For signed integers:
    ///
    /// * `0` if the number is zero
    /// * `1` if the number is positive
    /// * `-1` if the number is negative
    fn signum(&self) -> Self;

    /// Returns true if the number is positive and false if the number is zero or negative.
    fn is_positive(&self) -> bool;

    /// Returns true if the number is negative and false if the number is zero or positive.
    fn is_negative(&self) -> bool;
}

macro_rules! signed_impl {
    ($($t:ty)*) => ($(
        impl Signed for $t {
            #[inline]
            fn abs(&self) -> $t {
                if self.is_negative() { -*self } else { *self }
            }

            #[inline]
            fn abs_sub(&self, other: &$t) -> $t {
                if *self <= *other { 0 } else { *self - *other }
            }

            #[inline]
            fn signum(&self) -> $t {
                match *self {
                    n if n > 0 => 1,
                    0 => 0,
                    _ => -1,
                }
            }

            #[inline]
            fn is_positive(&self) -> bool { *self > 0 }

            #[inline]
            fn is_negative(&self) -> bool { *self < 0 }
        }
    )*)
}

signed_impl!(isize i8 i16 i32 i64 i128);

impl<T: Signed> Signed for Wrapping<T>
where
    Wrapping<T>: Num + Neg<Output = Wrapping<T>>,
{
    #[inline]
    fn abs(&self) -> Self {
        Wrapping(self.0.abs())
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        Wrapping(self.0.abs_sub(&other.0))
    }

    #[inline]
    fn signum(&self) -> Self {
        Wrapping(self.0.signum())
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

macro_rules! signed_float_impl {
    ($t:ty) => {
        impl Signed for $t {
            /// Computes the absolute value. Returns `NAN` if the number is `NAN`.
            #[inline]
            fn abs(&self) -> $t {
                FloatCore::abs(*self)
            }

            /// The positive difference of two numbers. Returns `0.0` if the number is
            /// less than or equal to `other`, otherwise the difference between`self`
            /// and `other` is returned.
            #[inline]
            fn abs_sub(&self, other: &$t) -> $t {
                if *self <= *other {
                    0.
                } else {
                    *self - *other
                }
            }

            /// # Returns
            ///
            /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
            /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
            /// - `NAN` if the number is NaN
            #[inline]
            fn signum(&self) -> $t {
                FloatCore::signum(*self)
            }

            /// Returns `true` if the number is positive, including `+0.0` and `INFINITY`
            #[inline]
            fn is_positive(&self) -> bool {
                FloatCore::is_sign_positive(*self)
            }

            /// Returns `true` if the number is negative, including `-0.0` and `NEG_INFINITY`
            #[inline]
            fn is_negative(&self) -> bool {
                FloatCore::is_sign_negative(*self)
            }
        }
    };
}

signed_float_impl!(f32);
signed_float_impl!(f64);

/// Computes the absolute value.
///
/// For `f32` and `f64`, `NaN` will be returned if the number is `NaN`
///
/// For signed integers, `::MIN` will be returned if the number is `::MIN`.
#[inline(always)]
pub fn abs<T: Signed>(value: T) -> T {
    value.abs()
}

/// The positive difference of two numbers.
///
/// Returns zero if `x` is less than or equal to `y`, otherwise the difference
/// between `x` and `y` is returned.
#[inline(always)]
pub fn abs_sub<T: Signed>(x: T, y: T) -> T {
    x.abs_sub(&y)
}

/// Returns the sign of the number.
///
/// For `f32` and `f64`:
///
/// * `1.0` if the number is positive, `+0.0` or `INFINITY`
/// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// * `NaN` if the number is `NaN`
///
/// For signed integers:
///
/// * `0` if the number is zero
/// * `1` if the number is positive
/// * `-1` if the number is negative
#[inline(always)]
pub fn signum<T: Signed>(value: T) -> T {
    value.signum()
}

/// A trait for values which cannot be negative
pub trait Unsigned: Num {}

macro_rules! empty_trait_impl {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
}

empty_trait_impl!(Unsigned for usize u8 u16 u32 u64 u128);

impl<T: Unsigned> Unsigned for Wrapping<T> where Wrapping<T>: Num {}

#[test]
fn unsigned_wrapping_is_unsigned() {
    fn require_unsigned<T: Unsigned>(_: &T) {}
    require_unsigned(&Wrapping(42_u32));
}

#[test]
fn signed_wrapping_is_signed() {
    fn require_signed<T: Signed>(_: &T) {}
    require_signed(&Wrapping(-42));
}
#[cfg(test)]
mod tests_rug_1358 {
    use super::*;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i32> = Wrapping(42);

        crate::sign::abs(p0);

    }
}
#[cfg(test)]
mod tests_rug_1359 {
    use super::*;
    use std::num::Wrapping;
    
    #[test]
    fn test_abs_sub() {
        let p0: Wrapping<i32> = Wrapping(42);
        let p1: Wrapping<i32> = Wrapping(20);
        
        assert_eq!(abs_sub(p0, p1), Wrapping(22));
    }
}#[cfg(test)]
mod tests_rug_1360 {
    use super::*;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i32> = Wrapping(42);
        crate::sign::signum(p0);
    }
}#[cfg(test)]
mod tests_rug_1366 {
    use super::*;
    use crate::Signed;
    
    #[test]
    fn test_abs() {
        let p0: i8 = 10;
        
        <i8 as Signed>::abs(&p0);
    }
}#[cfg(test)]
mod tests_rug_1367 {
    use super::*;
    use crate::Signed;
    
    #[test]
    fn test_rug() {
        let mut p0: i8 = 5;
        let mut p1: i8 = 3;

        p0.abs_sub(&p1);
    }
}#[cfg(test)]
mod tests_rug_1372 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 10;
        let mut p1: i16 = 5;

        <i16 as Signed>::abs_sub(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1376 {
    use super::*;
    use crate::Signed;
    
    #[test]
    fn test_abs() {
        let p0: i32 = 5;
        <i32 as Signed>::abs(&p0);

        let p1: i32 = -10;
        <i32 as Signed>::abs(&p1);

        let p2: i32 = 0;
        <i32 as Signed>::abs(&p2);
    }
}#[cfg(test)]
mod tests_rug_1377 {
    use super::*;
    use crate::sign::Signed;

    #[test]
    fn test_abs_sub() {
        let p0: i32 = 5;
        let p1: i32 = 3;

        <i32 as Signed>::abs_sub(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1378 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_rug() {
        let mut p0: i32 = 10;

        <i32>::signum(p0);
    }
}        
        #[cfg(test)]
        mod tests_rug_1379 {
            use super::*;
            use crate::Signed;
            
            #[test]
            fn test_rug() {
                let mut p0: i32 = 10;
                p0.is_positive();

            }
        }
        #[cfg(test)]
mod tests_rug_1382 {
    use super::*;
    use crate::sign::Signed;

    #[test]
    fn test_rug() {
        let mut p0: i64 = 10;
        let mut p1: i64 = 5;

        p0.abs_sub(&p1);
    }
}        
        #[cfg(test)]
        mod tests_rug_1383 {
            use super::*;
            use crate::Signed;
            #[test]
            fn test_rug() {
                let mut p0: i64 = 10;
                
                p0.signum();

            }
        }
                            #[cfg(test)]
mod tests_rug_1388 {
    use super::*;
    use crate::Signed;
    
    #[test]
    fn test_rug() {
        let p0: i128 = 123;
        
        p0.signum();
    }
}#[cfg(test)]
mod tests_rug_1390 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_rug() {
        let mut p0: i128 = -42;

        p0.is_negative();
    }
}#[cfg(test)]
mod tests_rug_1393 {
    use super::*;
    use crate::Signed;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let p0: Wrapping<i32> = Wrapping(42);
        
        p0.signum();
    }
}#[cfg(test)]
mod tests_rug_1394 {
    use super::*;
    use std::num::Wrapping;
    
    use crate::sign::Signed;
    
    #[test]
    fn test_rug() {
        // Construct the Wrapping<T> type
        let p0: Wrapping<i32> = Wrapping(42);
        
        p0.is_positive();
    }
}

#[cfg(test)]
mod tests_rug_1395 {
    use super::*;
    use crate::Signed;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let mut p0 = Wrapping(42); // sample

        assert_eq!(<Wrapping<i32> as Signed>::is_negative(&p0), false);
    }
}
#[cfg(test)]
mod tests_rug_1396 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_abs() {
        let p0: f32 = 3.14;

        <f32>::abs(p0);
    }
}#[cfg(test)]
mod tests_rug_1398 {
    use super::*;
    use crate::sign::Signed;

    #[test]
    fn test_rug() {
        let p0: f32 = 3.14;
        <f32 as Signed>::signum(&p0);
    }
}#[cfg(test)]
mod tests_rug_1401 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 5.67;

        p0.abs();
    }
}#[cfg(test)]
mod tests_rug_1402 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_abs_sub() {
        let p0: f64 = 5.0;  // Sample value for the first argument
        let p1: f64 = 3.0;  // Sample value for the second argument

        <f64 as Signed>::abs_sub(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1403 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_rug() {
        let p0: f64 = 3.14;

        p0.signum();
    }
}#[cfg(test)]
        mod tests_rug_1404 {
            use super::*;
            use crate::Signed;

            #[test]
            fn test_rug() {
                let mut p0: f64 = 3.14;

                <f64>::is_positive(p0);

            }
        }#[cfg(test)]
mod tests_rug_1405 {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 10.5;

        <f64>::is_negative(p0);
    }
}