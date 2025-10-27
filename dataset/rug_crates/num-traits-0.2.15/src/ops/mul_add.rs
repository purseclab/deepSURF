/// Fused multiply-add. Computes `(self * a) + b` with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
///
/// Using `mul_add` can be more performant than an unfused multiply-add if
/// the target architecture has a dedicated `fma` CPU instruction.
///
/// Note that `A` and `B` are `Self` by default, but this is not mandatory.
///
/// # Example
///
/// ```
/// use std::f32;
///
/// let m = 10.0_f32;
/// let x = 4.0_f32;
/// let b = 60.0_f32;
///
/// // 100.0
/// let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();
///
/// assert!(abs_difference <= 100.0 * f32::EPSILON);
/// ```
pub trait MulAdd<A = Self, B = Self> {
    /// The resulting type after applying the fused multiply-add.
    type Output;

    /// Performs the fused multiply-add operation.
    fn mul_add(self, a: A, b: B) -> Self::Output;
}

/// The fused multiply-add assignment operation.
pub trait MulAddAssign<A = Self, B = Self> {
    /// Performs the fused multiply-add operation.
    fn mul_add_assign(&mut self, a: A, b: B);
}

#[cfg(any(feature = "std", feature = "libm"))]
impl MulAdd<f32, f32> for f32 {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        <Self as crate::Float>::mul_add(self, a, b)
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl MulAdd<f64, f64> for f64 {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        <Self as crate::Float>::mul_add(self, a, b)
    }
}

macro_rules! mul_add_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            type Output = Self;

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self::Output {
                (self * a) + b
            }
        }
    )*}
}

mul_add_impl!(MulAdd for isize i8 i16 i32 i64 i128);
mul_add_impl!(MulAdd for usize u8 u16 u32 u64 u128);

#[cfg(any(feature = "std", feature = "libm"))]
impl MulAddAssign<f32, f32> for f32 {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = <Self as crate::Float>::mul_add(*self, a, b)
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl MulAddAssign<f64, f64> for f64 {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = <Self as crate::Float>::mul_add(*self, a, b)
    }
}

macro_rules! mul_add_assign_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn mul_add_assign(&mut self, a: Self, b: Self) {
                *self = (*self * a) + b
            }
        }
    )*}
}

mul_add_assign_impl!(MulAddAssign for isize i8 i16 i32 i64 i128);
mul_add_assign_impl!(MulAddAssign for usize u8 u16 u32 u64 u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_add_integer() {
        macro_rules! test_mul_add {
            ($($t:ident)+) => {
                $(
                    {
                        let m: $t = 2;
                        let x: $t = 3;
                        let b: $t = 4;

                        assert_eq!(MulAdd::mul_add(m, x, b), (m*x + b));
                    }
                )+
            };
        }

        test_mul_add!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
    }

    #[test]
    #[cfg(feature = "std")]
    fn mul_add_float() {
        macro_rules! test_mul_add {
            ($($t:ident)+) => {
                $(
                    {
                        use core::$t;

                        let m: $t = 12.0;
                        let x: $t = 3.4;
                        let b: $t = 5.6;

                        let abs_difference = (MulAdd::mul_add(m, x, b) - (m*x + b)).abs();

                        assert!(abs_difference <= 46.4 * $t::EPSILON);
                    }
                )+
            };
        }

        test_mul_add!(f32 f64);
    }
}
                        
#[cfg(test)]
mod tests_rug_1655 {
    use super::*;
    use crate::MulAdd;
    
    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.0;
        let mut p1: f32 = 3.0;
        let mut p2: f32 = 4.0;

        <f32>::mul_add(p0, p1, p2);
    }
}#[cfg(test)]
mod tests_rug_1656 {
    use super::*;
    use crate::ops::mul_add;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 3.14;
        let mut p1: f64 = 2.71;
        let mut p2: f64 = 1.23;
        
        <f64 as crate::ops::mul_add::MulAdd>::mul_add(p0, p1, p2);
    }
}#[cfg(test)]
mod tests_rug_1660 {
    use super::*;
    use crate::MulAdd;

    #[test]
    fn test_rug() {
        let mut p0: i32 = 5;
        let mut p1: i32 = 10;
        let mut p2: i32 = 15;

        p0.mul_add(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_1665 {
    use super::*;
    use crate::MulAdd;

    #[test]
    fn test_mul_add() {
        let mut p0: u16 = 10;
        let mut p1: u16 = 5;
        let mut p2: u16 = 2;

        u16::mul_add(p0, p1, p2);
    }
}
#[cfg(test)]
mod tests_rug_1666 {
    use super::*;
    use crate::ops::mul_add::MulAdd;

    #[test]
    fn test_mul_add() {
        let p0: u32 = 5;
        let p1: u32 = 6;
        let p2: u32 = 7;

        p0.mul_add(p1, p2);
    }
}
                        
#[cfg(test)]
mod tests_rug_1667 {
    use super::*;
    use crate::MulAdd;
    
    #[test]
    fn test_mul_add() {
        let p0: u64 = 10;
        let p1: u64 = 5;
        let p2: u64 = 3;
        
        <u64 as crate::ops::mul_add::MulAdd>::mul_add(p0, p1, p2);
    }
}

#[cfg(test)]
mod tests_rug_1669 {
    use super::*;
    use crate::MulAddAssign;
    
    #[test]
    fn test_rug() {
        let mut p0: f32 = 1.5;
        let mut p1: f32 = 2.5;
        let mut p2: f32 = 3.5;
        
        p0.mul_add_assign(p1, p2);
        // assert or verify the output here
    }
}
#[cfg(test)]
mod tests_rug_1670 {
    use super::*;
    use crate::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: f64 = 3.0;
        let mut p2: f64 = 5.0;

        p0.mul_add_assign(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_1671 {
    use super::*;
    use crate::ops::mul_add::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: isize = 2;
        let mut p1: isize = 3;
        let mut p2: isize = 4;

        <isize as MulAddAssign>::mul_add_assign(&mut p0, p1, p2);
        
        assert_eq!(p0, 10);
    }
}#[cfg(test)]
mod tests_rug_1672 {
    use super::*;
    use crate::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: i8 = 10;
        let p1: i8 = 5;
        let p2: i8 = 2;

        p0.mul_add_assign(p1, p2);
        
        assert_eq!(p0, 52);
    }
}#[cfg(test)]
mod tests_rug_1673 {
    use super::*;
    use crate::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 10;
        let mut p1: i16 = 5;
        let mut p2: i16 = 7;

        p0.mul_add_assign(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_1676 {
    use super::*;
    use crate::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: i128 = 10;
        let mut p1: i128 = 20;
        let mut p2: i128 = 30;

        p0.mul_add_assign(p1, p2);
        
        // Add assertions here
        
    }
}#[cfg(test)]
mod tests_rug_1678 {
    use super::*;
    use crate::ops::mul_add::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: u8 = 5;
        let mut p1: u8 = 3;
        let mut p2: u8 = 2;

        p0.mul_add_assign(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_1679 {
    use super::*;
    use crate::ops::mul_add::MulAddAssign;
    
    #[test]
    fn test_rug() {
        let mut p0: u16 = 10;
        let mut p1: u16 = 5;
        let mut p2: u16 = 3;
        
        <u16 as crate::ops::mul_add::MulAddAssign>::mul_add_assign(&mut p0, p1, p2);
        
        assert_eq!(p0, (p0 * p1) + p2);
    }
}#[cfg(test)]
mod tests_rug_1680 {
    use super::*;
    use crate::MulAddAssign;
    
    #[test]
    fn test_mul_add_assign() {
        let mut p0: u32 = 5;
        let mut p1: u32 = 10;
        let mut p2: u32 = 3;

        p0.mul_add_assign(p1, p2);
        
        assert_eq!(p0, 53);
    }
}#[cfg(test)]
mod tests_rug_1682 {
    use super::*;
    use crate::MulAddAssign;

    #[test]
    fn test_rug() {
        let mut p0: u128 = 10;
        let mut p1: u128 = 5;
        let mut p2: u128 = 2;

        p0.mul_add_assign(p1, p2);
    }
}