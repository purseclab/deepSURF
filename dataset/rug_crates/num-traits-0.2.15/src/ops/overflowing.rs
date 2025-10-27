use core::ops::{Add, Mul, Sub};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

macro_rules! overflowing_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, v: &Self) -> (Self, bool) {
                <$t>::$method(*self, *v)
            }
        }
    };
}

/// Performs addition with a flag for overflow.
pub trait OverflowingAdd: Sized + Add<Self, Output = Self> {
    /// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_add(&self, v: &Self) -> (Self, bool);
}

overflowing_impl!(OverflowingAdd, overflowing_add, u8);
overflowing_impl!(OverflowingAdd, overflowing_add, u16);
overflowing_impl!(OverflowingAdd, overflowing_add, u32);
overflowing_impl!(OverflowingAdd, overflowing_add, u64);
overflowing_impl!(OverflowingAdd, overflowing_add, usize);
overflowing_impl!(OverflowingAdd, overflowing_add, u128);

overflowing_impl!(OverflowingAdd, overflowing_add, i8);
overflowing_impl!(OverflowingAdd, overflowing_add, i16);
overflowing_impl!(OverflowingAdd, overflowing_add, i32);
overflowing_impl!(OverflowingAdd, overflowing_add, i64);
overflowing_impl!(OverflowingAdd, overflowing_add, isize);
overflowing_impl!(OverflowingAdd, overflowing_add, i128);

/// Performs substraction with a flag for overflow.
pub trait OverflowingSub: Sized + Sub<Self, Output = Self> {
    /// Returns a tuple of the difference along with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_sub(&self, v: &Self) -> (Self, bool);
}

overflowing_impl!(OverflowingSub, overflowing_sub, u8);
overflowing_impl!(OverflowingSub, overflowing_sub, u16);
overflowing_impl!(OverflowingSub, overflowing_sub, u32);
overflowing_impl!(OverflowingSub, overflowing_sub, u64);
overflowing_impl!(OverflowingSub, overflowing_sub, usize);
overflowing_impl!(OverflowingSub, overflowing_sub, u128);

overflowing_impl!(OverflowingSub, overflowing_sub, i8);
overflowing_impl!(OverflowingSub, overflowing_sub, i16);
overflowing_impl!(OverflowingSub, overflowing_sub, i32);
overflowing_impl!(OverflowingSub, overflowing_sub, i64);
overflowing_impl!(OverflowingSub, overflowing_sub, isize);
overflowing_impl!(OverflowingSub, overflowing_sub, i128);

/// Performs multiplication with a flag for overflow.
pub trait OverflowingMul: Sized + Mul<Self, Output = Self> {
    /// Returns a tuple of the product along with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_mul(&self, v: &Self) -> (Self, bool);
}

overflowing_impl!(OverflowingMul, overflowing_mul, u8);
overflowing_impl!(OverflowingMul, overflowing_mul, u16);
overflowing_impl!(OverflowingMul, overflowing_mul, u32);
overflowing_impl!(OverflowingMul, overflowing_mul, u64);
overflowing_impl!(OverflowingMul, overflowing_mul, usize);
overflowing_impl!(OverflowingMul, overflowing_mul, u128);

overflowing_impl!(OverflowingMul, overflowing_mul, i8);
overflowing_impl!(OverflowingMul, overflowing_mul, i16);
overflowing_impl!(OverflowingMul, overflowing_mul, i32);
overflowing_impl!(OverflowingMul, overflowing_mul, i64);
overflowing_impl!(OverflowingMul, overflowing_mul, isize);
overflowing_impl!(OverflowingMul, overflowing_mul, i128);

#[test]
fn test_overflowing_traits() {
    fn overflowing_add<T: OverflowingAdd>(a: T, b: T) -> (T, bool) {
        a.overflowing_add(&b)
    }
    fn overflowing_sub<T: OverflowingSub>(a: T, b: T) -> (T, bool) {
        a.overflowing_sub(&b)
    }
    fn overflowing_mul<T: OverflowingMul>(a: T, b: T) -> (T, bool) {
        a.overflowing_mul(&b)
    }
    assert_eq!(overflowing_add(5i16, 2), (7, false));
    assert_eq!(overflowing_add(i16::MAX, 1), (i16::MIN, true));
    assert_eq!(overflowing_sub(5i16, 2), (3, false));
    assert_eq!(overflowing_sub(i16::MIN, 1), (i16::MAX, true));
    assert_eq!(overflowing_mul(5i16, 2), (10, false));
    assert_eq!(overflowing_mul(1_000_000_000i32, 10), (1410065408, true));
}
#[cfg(test)]
mod tests_rug_1683 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;
    
    #[test]
    fn test_rug() {
        let mut p0: u8 = 50;
        let mut p1: u8 = 100;
        
        <u8>::overflowing_add(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1686 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;

    #[test]
    fn test_rug() {
        let mut p0: u64 = 10;  // Sample value for the first argument
        let mut p1: u64 = 20;  // Sample value for the second argument

        <u64>::overflowing_add(p0, p1);
    }
}#[cfg(test)]
        mod tests_rug_1687 {
            use super::*;
            use crate::ops::overflowing::OverflowingAdd;
            
            #[test]
            fn test_rug() {
                let mut p0: usize = 10;
                let mut p1: usize = 5;
                
                <usize>::overflowing_add(p0, p1);

            }
        }#[cfg(test)]
mod tests_rug_1690 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 42;
        let mut p1: i16 = 100;

        <i16 as OverflowingAdd>::overflowing_add(&p0, &p1);
    }
}                    
#[cfg(test)]
mod tests_rug_1692 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;
    
    #[test]
    fn test_rug() {
        let mut p0: i64 = 10;
        let mut p1: i64 = 5;
        
        <i64>::overflowing_add(p0, p1);
    }
}        
#[cfg(test)]
mod tests_rug_1693 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;
    
    #[test]
    fn test_rug() {
        let mut p0: isize = 10;
        let mut p1: isize = 20;
        
        <isize>::overflowing_add(p0, p1);
    }
}
                            #[cfg(test)]
mod tests_rug_1696 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;
    
    #[test]
    fn test_rug() {
        let mut p0: u16 = 42;
        let mut p1: u16 = 24;

        <u16 as OverflowingSub>::overflowing_sub(&p0, &p1);
    }
}#[cfg(test)]
        mod tests_rug_1698 {
            use super::*;
            use crate::ops::overflowing::OverflowingSub;
            #[test]
            fn test_rug() {
                let mut p0: u64 = 123;
                let mut p1: u64 = 456;

                
                <u64>::overflowing_sub(p0, p1);

            }
        }#[cfg(test)]
mod tests_rug_1700 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;

    #[test]
    fn test_overflowing_sub() {
        let mut p0: u128 = 1234567890;
        let mut p1: u128 = 987654321;

        <u128 as OverflowingSub>::overflowing_sub(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1701 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;

    #[test]
    fn test_rug() {
        let mut p0: i8 = 42;
        let mut p1: i8 = 10;
        
        <i8 as OverflowingSub>::overflowing_sub(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1703 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;

    #[test]
    fn test_rug() {
        let mut p0 = 42;
        let mut p1 = -10;

        p0.overflowing_sub(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1705 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;
    
    #[test]
    fn test_rug() {
        let mut p0 = 10;
        let mut p1 = 5;
        
        p0.overflowing_sub(&p1);
    }
}
                            
#[cfg(test)]
mod tests_rug_1709 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    
    #[test]
    fn test_rug() {
        let p0: u32 = 42;
        let p1: u32 = 24;
        
        <u32 as OverflowingMul>::overflowing_mul(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1711 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    
    #[test]
    fn test_rug() {
        let mut p0: usize = 10;
        let mut p1: usize = 5;
        
        <usize>::overflowing_mul(p0, p1);

    }
}
#[cfg(test)]
mod tests_rug_1713 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;

    #[test]
    fn test_rug() {
        let mut p0: i8 = 5;
        let mut p1: i8 = -3;

        <i8 as OverflowingMul>::overflowing_mul(&p0, &p1);
    }
}
#[cfg(test)]
mod tests_rug_1715 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    
    #[test]
    fn test_rug() {
        let mut p0: i32 = 123;
        let mut p1: i32 = 456;
        
        <i32 as OverflowingMul>::overflowing_mul(&p0, &p1);
    }
}                        
#[cfg(test)]
mod tests_rug_1716 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    
    #[test]
    fn test_rug() {
        let mut p0: i64 = 123;
        let mut p1: i64 = 456;
        
        <i64>::overflowing_mul(p0, p1);
    }
}
    #[cfg(test)]
mod tests_rug_1717 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;

    #[test]
    fn test_rug() {
        let mut p0: isize = 10;
        let mut p1: isize = 5;

        <isize as OverflowingMul>::overflowing_mul(&p0, &p1);
    }
}                    
#[cfg(test)]
mod tests_rug_1718 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;

    #[test]
    fn test_rug() {
        let mut p0: i128 = 12345;
        let mut p1: i128 = -67890;

        <i128>::overflowing_mul(p0, p1);
    }
}