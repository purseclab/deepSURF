use core::num::Wrapping;
use core::ops::{Add, Mul, Neg, Shl, Shr, Sub};

macro_rules! wrapping_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, v: &Self) -> Self {
                <$t>::$method(*self, *v)
            }
        }
    };
    ($trait_name:ident, $method:ident, $t:ty, $rhs:ty) => {
        impl $trait_name<$rhs> for $t {
            #[inline]
            fn $method(&self, v: &$rhs) -> Self {
                <$t>::$method(*self, *v)
            }
        }
    };
}

/// Performs addition that wraps around on overflow.
pub trait WrappingAdd: Sized + Add<Self, Output = Self> {
    /// Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of
    /// the type.
    fn wrapping_add(&self, v: &Self) -> Self;
}

wrapping_impl!(WrappingAdd, wrapping_add, u8);
wrapping_impl!(WrappingAdd, wrapping_add, u16);
wrapping_impl!(WrappingAdd, wrapping_add, u32);
wrapping_impl!(WrappingAdd, wrapping_add, u64);
wrapping_impl!(WrappingAdd, wrapping_add, usize);
wrapping_impl!(WrappingAdd, wrapping_add, u128);

wrapping_impl!(WrappingAdd, wrapping_add, i8);
wrapping_impl!(WrappingAdd, wrapping_add, i16);
wrapping_impl!(WrappingAdd, wrapping_add, i32);
wrapping_impl!(WrappingAdd, wrapping_add, i64);
wrapping_impl!(WrappingAdd, wrapping_add, isize);
wrapping_impl!(WrappingAdd, wrapping_add, i128);

/// Performs subtraction that wraps around on overflow.
pub trait WrappingSub: Sized + Sub<Self, Output = Self> {
    /// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary
    /// of the type.
    fn wrapping_sub(&self, v: &Self) -> Self;
}

wrapping_impl!(WrappingSub, wrapping_sub, u8);
wrapping_impl!(WrappingSub, wrapping_sub, u16);
wrapping_impl!(WrappingSub, wrapping_sub, u32);
wrapping_impl!(WrappingSub, wrapping_sub, u64);
wrapping_impl!(WrappingSub, wrapping_sub, usize);
wrapping_impl!(WrappingSub, wrapping_sub, u128);

wrapping_impl!(WrappingSub, wrapping_sub, i8);
wrapping_impl!(WrappingSub, wrapping_sub, i16);
wrapping_impl!(WrappingSub, wrapping_sub, i32);
wrapping_impl!(WrappingSub, wrapping_sub, i64);
wrapping_impl!(WrappingSub, wrapping_sub, isize);
wrapping_impl!(WrappingSub, wrapping_sub, i128);

/// Performs multiplication that wraps around on overflow.
pub trait WrappingMul: Sized + Mul<Self, Output = Self> {
    /// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary
    /// of the type.
    fn wrapping_mul(&self, v: &Self) -> Self;
}

wrapping_impl!(WrappingMul, wrapping_mul, u8);
wrapping_impl!(WrappingMul, wrapping_mul, u16);
wrapping_impl!(WrappingMul, wrapping_mul, u32);
wrapping_impl!(WrappingMul, wrapping_mul, u64);
wrapping_impl!(WrappingMul, wrapping_mul, usize);
wrapping_impl!(WrappingMul, wrapping_mul, u128);

wrapping_impl!(WrappingMul, wrapping_mul, i8);
wrapping_impl!(WrappingMul, wrapping_mul, i16);
wrapping_impl!(WrappingMul, wrapping_mul, i32);
wrapping_impl!(WrappingMul, wrapping_mul, i64);
wrapping_impl!(WrappingMul, wrapping_mul, isize);
wrapping_impl!(WrappingMul, wrapping_mul, i128);

macro_rules! wrapping_unary_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self) -> $t {
                <$t>::$method(*self)
            }
        }
    };
}

/// Performs a negation that does not panic.
pub trait WrappingNeg: Sized {
    /// Wrapping (modular) negation. Computes `-self`,
    /// wrapping around at the boundary of the type.
    ///
    /// Since unsigned types do not have negative equivalents
    /// all applications of this function will wrap (except for `-0`).
    /// For values smaller than the corresponding signed type's maximum
    /// the result is the same as casting the corresponding signed value.
    /// Any larger values are equivalent to `MAX + 1 - (val - MAX - 1)` where
    /// `MAX` is the corresponding signed type's maximum.
    ///
    /// ```
    /// use num_traits::WrappingNeg;
    ///
    /// assert_eq!(100i8.wrapping_neg(), -100);
    /// assert_eq!((-100i8).wrapping_neg(), 100);
    /// assert_eq!((-128i8).wrapping_neg(), -128); // wrapped!
    /// ```
    fn wrapping_neg(&self) -> Self;
}

wrapping_unary_impl!(WrappingNeg, wrapping_neg, u8);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u16);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u32);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u64);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, usize);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u128);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i8);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i16);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i32);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i64);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, isize);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i128);

macro_rules! wrapping_shift_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, rhs: u32) -> $t {
                <$t>::$method(*self, rhs)
            }
        }
    };
}

/// Performs a left shift that does not panic.
pub trait WrappingShl: Sized + Shl<usize, Output = Self> {
    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    /// where `mask` removes any high order bits of `rhs` that would
    /// cause the shift to exceed the bitwidth of the type.
    ///
    /// ```
    /// use num_traits::WrappingShl;
    ///
    /// let x: u16 = 0x0001;
    ///
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 0),  0x0001);
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 1),  0x0002);
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 15), 0x8000);
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 16), 0x0001);
    /// ```
    fn wrapping_shl(&self, rhs: u32) -> Self;
}

wrapping_shift_impl!(WrappingShl, wrapping_shl, u8);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u16);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u32);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u64);
wrapping_shift_impl!(WrappingShl, wrapping_shl, usize);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u128);

wrapping_shift_impl!(WrappingShl, wrapping_shl, i8);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i16);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i32);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i64);
wrapping_shift_impl!(WrappingShl, wrapping_shl, isize);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i128);

/// Performs a right shift that does not panic.
pub trait WrappingShr: Sized + Shr<usize, Output = Self> {
    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`,
    /// where `mask` removes any high order bits of `rhs` that would
    /// cause the shift to exceed the bitwidth of the type.
    ///
    /// ```
    /// use num_traits::WrappingShr;
    ///
    /// let x: u16 = 0x8000;
    ///
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 0),  0x8000);
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 1),  0x4000);
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 15), 0x0001);
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 16), 0x8000);
    /// ```
    fn wrapping_shr(&self, rhs: u32) -> Self;
}

wrapping_shift_impl!(WrappingShr, wrapping_shr, u8);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u16);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u32);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u64);
wrapping_shift_impl!(WrappingShr, wrapping_shr, usize);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u128);

wrapping_shift_impl!(WrappingShr, wrapping_shr, i8);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i16);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i32);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i64);
wrapping_shift_impl!(WrappingShr, wrapping_shr, isize);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i128);

// Well this is a bit funny, but all the more appropriate.
impl<T: WrappingAdd> WrappingAdd for Wrapping<T>
where
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    fn wrapping_add(&self, v: &Self) -> Self {
        Wrapping(self.0.wrapping_add(&v.0))
    }
}
impl<T: WrappingSub> WrappingSub for Wrapping<T>
where
    Wrapping<T>: Sub<Output = Wrapping<T>>,
{
    fn wrapping_sub(&self, v: &Self) -> Self {
        Wrapping(self.0.wrapping_sub(&v.0))
    }
}
impl<T: WrappingMul> WrappingMul for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    fn wrapping_mul(&self, v: &Self) -> Self {
        Wrapping(self.0.wrapping_mul(&v.0))
    }
}
impl<T: WrappingNeg> WrappingNeg for Wrapping<T>
where
    Wrapping<T>: Neg<Output = Wrapping<T>>,
{
    fn wrapping_neg(&self) -> Self {
        Wrapping(self.0.wrapping_neg())
    }
}
impl<T: WrappingShl> WrappingShl for Wrapping<T>
where
    Wrapping<T>: Shl<usize, Output = Wrapping<T>>,
{
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Wrapping(self.0.wrapping_shl(rhs))
    }
}
impl<T: WrappingShr> WrappingShr for Wrapping<T>
where
    Wrapping<T>: Shr<usize, Output = Wrapping<T>>,
{
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Wrapping(self.0.wrapping_shr(rhs))
    }
}

#[test]
fn test_wrapping_traits() {
    fn wrapping_add<T: WrappingAdd>(a: T, b: T) -> T {
        a.wrapping_add(&b)
    }
    fn wrapping_sub<T: WrappingSub>(a: T, b: T) -> T {
        a.wrapping_sub(&b)
    }
    fn wrapping_mul<T: WrappingMul>(a: T, b: T) -> T {
        a.wrapping_mul(&b)
    }
    fn wrapping_neg<T: WrappingNeg>(a: T) -> T {
        a.wrapping_neg()
    }
    fn wrapping_shl<T: WrappingShl>(a: T, b: u32) -> T {
        a.wrapping_shl(b)
    }
    fn wrapping_shr<T: WrappingShr>(a: T, b: u32) -> T {
        a.wrapping_shr(b)
    }
    assert_eq!(wrapping_add(255, 1), 0u8);
    assert_eq!(wrapping_sub(0, 1), 255u8);
    assert_eq!(wrapping_mul(255, 2), 254u8);
    assert_eq!(wrapping_neg(255), 1u8);
    assert_eq!(wrapping_shl(255, 8), 255u8);
    assert_eq!(wrapping_shr(255, 8), 255u8);
    assert_eq!(wrapping_add(255, 1), (Wrapping(255u8) + Wrapping(1u8)).0);
    assert_eq!(wrapping_sub(0, 1), (Wrapping(0u8) - Wrapping(1u8)).0);
    assert_eq!(wrapping_mul(255, 2), (Wrapping(255u8) * Wrapping(2u8)).0);
    assert_eq!(wrapping_neg(255), (-Wrapping(255u8)).0);
    assert_eq!(wrapping_shl(255, 8), (Wrapping(255u8) << 8).0);
    assert_eq!(wrapping_shr(255, 8), (Wrapping(255u8) >> 8).0);
}

#[test]
fn wrapping_is_wrappingadd() {
    fn require_wrappingadd<T: WrappingAdd>(_: &T) {}
    require_wrappingadd(&Wrapping(42));
}

#[test]
fn wrapping_is_wrappingsub() {
    fn require_wrappingsub<T: WrappingSub>(_: &T) {}
    require_wrappingsub(&Wrapping(42));
}

#[test]
fn wrapping_is_wrappingmul() {
    fn require_wrappingmul<T: WrappingMul>(_: &T) {}
    require_wrappingmul(&Wrapping(42));
}

#[test]
fn wrapping_is_wrappingneg() {
    fn require_wrappingneg<T: WrappingNeg>(_: &T) {}
    require_wrappingneg(&Wrapping(42));
}

#[test]
fn wrapping_is_wrappingshl() {
    fn require_wrappingshl<T: WrappingShl>(_: &T) {}
    require_wrappingshl(&Wrapping(42));
}

#[test]
fn wrapping_is_wrappingshr() {
    fn require_wrappingshr<T: WrappingShr>(_: &T) {}
    require_wrappingshr(&Wrapping(42));
}
#[cfg(test)]
mod tests_rug_1780 {
    use super::*;
    use crate::ops::wrapping::WrappingAdd;

    #[test]
    fn test_wrapping_add() {
        let p0: u16 = 42;
        let p1: u16 = 123;
        
        let result = <u16 as WrappingAdd>::wrapping_add(&p0, &p1);
        
        assert_eq!(result, 165);
    }
}#[cfg(test)]
mod tests_rug_1790 {
    use super::*;
    use crate::WrappingAdd;

    #[test]
    fn test_rug() {
        let mut p0: i128 = 12345;
        let mut p1: i128 = 67890;

        <i128>::wrapping_add(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1799 {
    use super::*;
    use crate::WrappingSub;
    
    #[test]
    fn test_rug() {
        let mut p0: i32 = 10;
        let mut p1: i32 = 5;
        
        p0.wrapping_sub(p1);
        
    }
}#[cfg(test)]
mod tests_rug_1801 {
    use super::*;
    use crate::ops::wrapping::WrappingSub;
    
    #[test]
    fn test_rug() {
        let mut p0: isize = 10;
        let mut p1: isize = 5;
        <isize as WrappingSub>::wrapping_sub(&p0, &p1);
    }
}#[cfg(test)]
        mod tests_rug_1807 {
            use super::*;
            use crate::WrappingMul;
            use std::usize;
            
            #[test]
            fn test_rug() {
                let mut p0: usize = 10;
                let mut p1: usize = 5;

                p0.wrapping_mul(p1);
            }
        }#[cfg(test)]
mod tests_rug_1811 {
    use super::*;
    use crate::ops::wrapping::WrappingMul;
    
    #[test]
    fn test_rug() {
        let p0: i32 = 10;
        let p1: i32 = 5;
        
        <i32>::wrapping_mul(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1816 {
    use super::*;
    use crate::ops::wrapping::WrappingNeg;

    #[test]
    fn test_rug() {
        let mut p0: u16 = 42;
        
        p0.wrapping_neg();
    }
}#[cfg(test)]
mod tests_rug_1820 {
    use super::*;
    use crate::WrappingNeg;

    #[test]
    fn test_rug() {
        let mut p0: u128 = 12345;

        p0.wrapping_neg();
    }
}#[cfg(test)]
mod tests_rug_1825 {
    use super::*;
    use crate::ops::wrapping::WrappingNeg;

    #[test]
    fn test_rug() {
        let p0: isize = 42;

        <isize as WrappingNeg>::wrapping_neg(&p0);
    }
}#[cfg(test)]
mod tests_rug_1830 {
    use super::*;
    use crate::WrappingShl;

    #[test]
    fn test_rug() {
        let mut p0: u64 = 10;
        let mut p1: u32 = 3;

        <u64>::wrapping_shl(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1831 {
    use super::*;
    use crate::ops::wrapping::WrappingShl;
    
    #[test]
    fn test_rug() {
        let mut p0: usize = 10; // sample value
        let mut p1: u32 = 3; // sample value
        
        <usize as WrappingShl>::wrapping_shl(&p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1833 {
    use super::*;
    use crate::ops::wrapping::WrappingShl;

    #[test]
    fn test_rug() {
        let mut p0: i8 = 42;
        let mut p1: u32 = 3;
        
        p0.wrapping_shl(p1);
    }
}#[cfg(test)]
mod tests_rug_1835 {
    use super::*;
    use crate::ops::wrapping::WrappingShl;
    
    #[test]
    fn test_rug() {
        let mut p0: i32 = 42;
        let mut p1: u32 = 3;
        
        <i32 as WrappingShl>::wrapping_shl(&p0, p1);

    }
}#[cfg(test)]
mod tests_rug_1841 {
    use super::*;
    use crate::ops::wrapping::WrappingShr;
    
    #[test]
    fn test_rug() {
        let mut p0: u32 = 10;
        let mut p1: u32 = 3;
        
        p0.wrapping_shr(p1);
    }
}
#[cfg(test)]
mod tests_rug_1851 {
    use super::*;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let p0 = Wrapping(10); // Replace with appropriate value
        let p1 = Wrapping(5); // Replace with appropriate value

        p0.wrapping_add(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1853 {
    use super::*;
    use std::num::Wrapping;
    
    #[test]
    fn test_wrapping_mul() {
        let p0: Wrapping<u64> = Wrapping(5);
        let p1: Wrapping<u64> = Wrapping(10);
        
        let result = p0.wrapping_mul(&p1);
        
        // Add assertions here
        assert_eq!(result, Wrapping(50));
    }
}
#[cfg(test)]
mod tests_rug_1854 {
    use super::*;
    use crate::WrappingNeg;
    
    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<i16> = std::num::Wrapping(42);

        p0.wrapping_neg();

    }
}
#[cfg(test)]
mod tests_rug_1855 {
    use super::*;
    use crate::WrappingShl;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let p0: std::num::Wrapping<u8> = Wrapping(42);
        let p1: u32 = 8;
        
        p0.wrapping_shl(p1);
    }
}
