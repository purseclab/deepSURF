use crate::{CheckedMul, One};
use core::num::Wrapping;
use core::ops::Mul;

/// Binary operator for raising a value to a power.
pub trait Pow<RHS> {
    /// The result after applying the operator.
    type Output;

    /// Returns `self` to the power `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Pow;
    /// assert_eq!(Pow::pow(10u32, 2u32), 100);
    /// ```
    fn pow(self, rhs: RHS) -> Self::Output;
}

macro_rules! pow_impl {
    ($t:ty) => {
        pow_impl!($t, u8);
        pow_impl!($t, usize);

        // FIXME: these should be possible
        // pow_impl!($t, u16);
        // pow_impl!($t, u32);
        // pow_impl!($t, u64);
    };
    ($t:ty, $rhs:ty) => {
        pow_impl!($t, $rhs, usize, pow);
    };
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:expr) => {
        impl Pow<$rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                ($method)(self, <$desired_rhs>::from(rhs))
            }
        }

        impl<'a> Pow<&'a $rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                ($method)(self, <$desired_rhs>::from(*rhs))
            }
        }

        impl<'a> Pow<$rhs> for &'a $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                ($method)(*self, <$desired_rhs>::from(rhs))
            }
        }

        impl<'a, 'b> Pow<&'a $rhs> for &'b $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                ($method)(*self, <$desired_rhs>::from(*rhs))
            }
        }
    };
}

pow_impl!(u8, u8, u32, u8::pow);
pow_impl!(u8, u16, u32, u8::pow);
pow_impl!(u8, u32, u32, u8::pow);
pow_impl!(u8, usize);
pow_impl!(i8, u8, u32, i8::pow);
pow_impl!(i8, u16, u32, i8::pow);
pow_impl!(i8, u32, u32, i8::pow);
pow_impl!(i8, usize);
pow_impl!(u16, u8, u32, u16::pow);
pow_impl!(u16, u16, u32, u16::pow);
pow_impl!(u16, u32, u32, u16::pow);
pow_impl!(u16, usize);
pow_impl!(i16, u8, u32, i16::pow);
pow_impl!(i16, u16, u32, i16::pow);
pow_impl!(i16, u32, u32, i16::pow);
pow_impl!(i16, usize);
pow_impl!(u32, u8, u32, u32::pow);
pow_impl!(u32, u16, u32, u32::pow);
pow_impl!(u32, u32, u32, u32::pow);
pow_impl!(u32, usize);
pow_impl!(i32, u8, u32, i32::pow);
pow_impl!(i32, u16, u32, i32::pow);
pow_impl!(i32, u32, u32, i32::pow);
pow_impl!(i32, usize);
pow_impl!(u64, u8, u32, u64::pow);
pow_impl!(u64, u16, u32, u64::pow);
pow_impl!(u64, u32, u32, u64::pow);
pow_impl!(u64, usize);
pow_impl!(i64, u8, u32, i64::pow);
pow_impl!(i64, u16, u32, i64::pow);
pow_impl!(i64, u32, u32, i64::pow);
pow_impl!(i64, usize);

pow_impl!(u128, u8, u32, u128::pow);
pow_impl!(u128, u16, u32, u128::pow);
pow_impl!(u128, u32, u32, u128::pow);
pow_impl!(u128, usize);

pow_impl!(i128, u8, u32, i128::pow);
pow_impl!(i128, u16, u32, i128::pow);
pow_impl!(i128, u32, u32, i128::pow);
pow_impl!(i128, usize);

pow_impl!(usize, u8, u32, usize::pow);
pow_impl!(usize, u16, u32, usize::pow);
pow_impl!(usize, u32, u32, usize::pow);
pow_impl!(usize, usize);
pow_impl!(isize, u8, u32, isize::pow);
pow_impl!(isize, u16, u32, isize::pow);
pow_impl!(isize, u32, u32, isize::pow);
pow_impl!(isize, usize);
pow_impl!(Wrapping<u8>);
pow_impl!(Wrapping<i8>);
pow_impl!(Wrapping<u16>);
pow_impl!(Wrapping<i16>);
pow_impl!(Wrapping<u32>);
pow_impl!(Wrapping<i32>);
pow_impl!(Wrapping<u64>);
pow_impl!(Wrapping<i64>);
pow_impl!(Wrapping<u128>);
pow_impl!(Wrapping<i128>);
pow_impl!(Wrapping<usize>);
pow_impl!(Wrapping<isize>);

// FIXME: these should be possible
// pow_impl!(u8, u64);
// pow_impl!(i16, u64);
// pow_impl!(i8, u64);
// pow_impl!(u16, u64);
// pow_impl!(u32, u64);
// pow_impl!(i32, u64);
// pow_impl!(u64, u64);
// pow_impl!(i64, u64);
// pow_impl!(usize, u64);
// pow_impl!(isize, u64);

#[cfg(any(feature = "std", feature = "libm"))]
mod float_impls {
    use super::Pow;
    use crate::Float;

    pow_impl!(f32, i8, i32, <f32 as Float>::powi);
    pow_impl!(f32, u8, i32, <f32 as Float>::powi);
    pow_impl!(f32, i16, i32, <f32 as Float>::powi);
    pow_impl!(f32, u16, i32, <f32 as Float>::powi);
    pow_impl!(f32, i32, i32, <f32 as Float>::powi);
    pow_impl!(f64, i8, i32, <f64 as Float>::powi);
    pow_impl!(f64, u8, i32, <f64 as Float>::powi);
    pow_impl!(f64, i16, i32, <f64 as Float>::powi);
    pow_impl!(f64, u16, i32, <f64 as Float>::powi);
    pow_impl!(f64, i32, i32, <f64 as Float>::powi);
    pow_impl!(f32, f32, f32, <f32 as Float>::powf);
    pow_impl!(f64, f32, f64, <f64 as Float>::powf);
    pow_impl!(f64, f64, f64, <f64 as Float>::powf);
}

/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.
///
/// # Example
///
/// ```rust
/// use num_traits::pow;
///
/// assert_eq!(pow(2i8, 4), 16);
/// assert_eq!(pow(6u8, 3), 216);
/// assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
/// ```
#[inline]
pub fn pow<T: Clone + One + Mul<T, Output = T>>(mut base: T, mut exp: usize) -> T {
    if exp == 0 {
        return T::one();
    }

    while exp & 1 == 0 {
        base = base.clone() * base;
        exp >>= 1;
    }
    if exp == 1 {
        return base;
    }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.clone() * base;
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
    }
    acc
}

/// Raises a value to the power of exp, returning `None` if an overflow occurred.
///
/// Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.
///
/// Otherwise same as the `pow` function.
///
/// # Example
///
/// ```rust
/// use num_traits::checked_pow;
///
/// assert_eq!(checked_pow(2i8, 4), Some(16));
/// assert_eq!(checked_pow(7i8, 8), None);
/// assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
/// assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you
/// ```
#[inline]
pub fn checked_pow<T: Clone + One + CheckedMul>(mut base: T, mut exp: usize) -> Option<T> {
    if exp == 0 {
        return Some(T::one());
    }

    while exp & 1 == 0 {
        base = base.checked_mul(&base)?;
        exp >>= 1;
    }
    if exp == 1 {
        return Some(base);
    }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.checked_mul(&base)?;
        if exp & 1 == 1 {
            acc = acc.checked_mul(&base)?;
        }
    }
    Some(acc)
}
#[cfg(test)]
mod tests_rug_1024 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static u8 = &42;
        let mut p1: u16 = 10;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1026 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: u8 = 5;
        let mut p1: u32 = 10;

        <u8 as Pow<u32>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1027 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: u8 = 5;
        let p1: &u32 = &10;

        <u8 as crate::pow::Pow<&u32>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1028 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: &'static u8 = &8;
        let p1: u32 = 3;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1029 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &u8 = &0;
        let mut p1: &u32 = &0;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1032 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: &'static u8 = &42;
        let p1: usize = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1034 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: i8 = 5;
        let mut p1: u8 = 3;

        <i8 as Pow<u8>>::pow(p0, p1);
    }
}        
#[cfg(test)]
mod tests_rug_1036 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: &'static i8 = &1;
        let mut p1: u8 = 2;
        
        p0.pow(p1);
    }
}
                            #[cfg(test)]
mod tests_rug_1042 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: i8 = 5;
        let mut p1: u32 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1048 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: i8 = 5;
        let p1: usize = 3;
                
        <&i8 as Pow<usize>>::pow(&p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1055 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: u16 = 10;
        let mut p1: u16 = 2;

        <u16 as Pow<&u16>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1056 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static u16 = &0;
        let mut p1: u16 = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1058 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: u16 = 10;
        let p1: u32 = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1068 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static i16 = &0;
        let mut p1: u8 = 0;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1074 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: i16 = 10;  // Sample value for 1st argument: i16
        let mut p1: u32 = 3;   // Sample value for 2nd argument: u32

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1080 {
    use super::*;
    use crate::pow::Pow;

    #[test]
    fn test_pow() {
        let p0: &'static i16 = &42;
        let p1: usize = 5;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1085 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: u32 = 5;
        let mut p1: u8 = 3;

        <&u32>::pow(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1092 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: &u32 = &5;
        let p1: u32 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1099 {
    use super::*;
    use crate::{Pow};

    #[test]
    fn test_pow() {
        let p0: i32 = 5;
        let p1: u8 = 3;
        
        <i32 as Pow<&u8>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1100 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static i32 = &10;
        let mut p1: u8 = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1104 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static i32 = &42;
        let mut p1: u16 = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1106 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: i32 = 5;
        let mut p1: u32 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1109 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: &'static i32 = &3;
        let p1: &'static u32 = &5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1112 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let mut p0: &'static i32 = &10;
        let mut p1: usize = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1114 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: u64 = 10;
        let p1: u8 = 2;

        <u64 as Pow<u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1122 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: u64 = 10; // sample data
        let p1: u32 = 2; // sample data

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1124 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: u64 = 5;
        let mut p1: u32 = 3;
        p0.pow(p1);

    }
}#[cfg(test)]
mod tests_rug_1127 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: u64 = 10;  // sample data
        let mut p1: usize = 5; // sample data
        
        <u64 as Pow<&usize>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1128 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static u64 = &0; // Sample data for the first argument
        let mut p1: usize = 0; // Sample data for the second argument

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1130 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_pow() {
        let p0: i64 = 2;
        let p1: u8 = 3;
        
        <i64 as Pow<u8>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1133 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: &'static i64 = &10;
        let p1: &'static u8 = &5;

        p0.pow(p1);

    }
}
#[cfg(test)]
mod tests_rug_1138 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: i64 = -10; // Sample data for the first argument
        let mut p1: u32 = 5;   // Sample data for the second argument
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1139 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_pow() {
        let mut p0: i64 = 10;
        let mut p1: u32 = 2;
        
        <i64 as Pow<&u32>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1140 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &i64 = &10;
        let mut p1: u32 = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1141 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: i64 = 5;
        let mut p1: u32 = 3;

        <&i64 as Pow<&u32>>::pow(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1144 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static i64 = &10;
        let mut p1: &'static usize = &5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1148 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: &'static u128 = &1234567890; // Sample data for the first argument
        let p1: u8 = 5; // Sample data for the second argument
        
        p0.pow(p1);
        
        // Add assertions or additional test logic here
        
    }
}        
#[cfg(test)]
mod tests_rug_1150 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: u128 = 10;
        let mut p1: u16 = 5;

        <u128 as Pow<u16>>::pow(p0, p1);
    }
}
                            #[cfg(test)]
mod tests_rug_1151 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: u128 = 1234;  // Sample data for the first argument
        let mut p1: u16 = 5678;   // Sample data for the second argument
      
        <u128 as Pow<&u16>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1154 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: u128 = 1234;
        let p1: u32 = 5;

        <u128 as Pow<u32>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1155 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: u128 = 10;
        let p1: u32 = 2;

        <u128 as Pow<&u32>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1156 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: u128 = 12;
        let mut p1: u32 = 3;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1157 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: u128 = 12345;
        let mut p1: u32 = 5;

        <&u128 as Pow<&u32>>::pow(&mut p0, &p1);
    }
}
#[cfg(test)]
mod tests_rug_1158 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let p0: u128 = 10;
        let p1: usize = 3;

        <u128 as Pow<usize>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1160 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: &'static u128 = &123456789;
        let p1: usize = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1162 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: i128 = 100;
        let mut p1: u8 = 5;

        <i128 as Pow<u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1165 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static i128 = &123; // sample data, replace with actual values
        let mut p1: &'static u8 = &5; // sample data, replace with actual values

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1170 {
    use super::*;
    use crate::Pow;
    #[test]
    fn test_rug() {
        // Sample data for 1st argument
        let p0: i128 = 123;
        
        // Sample data for 2nd argument
        let p1: u32 = 456;
        
        <i128 as Pow<u32>>::pow(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_1180 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static usize = &42;
        let mut p1: u8 = 5;

        p0.pow(p1);

    }
}
                    #[cfg(test)]
mod tests_rug_1181 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static usize = &123;
        let mut p1: &'static u8 = &45;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1184 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static usize = &10;
        let mut p1: &'static u16 = &5;

        p0.pow(p1);
    }
}      
#[cfg(test)]
mod tests_rug_1186 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        // Initialize the arguments
        let p0: usize = 5;
        let p1: u32 = 3;

        // Call the target function
        <usize>::pow(p0, p1);
    }
}
                      #[cfg(test)]
mod tests_rug_1192 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static usize = &0;
        let mut p1: usize = 1;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1196 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static isize = &123; // Sample value for the first argument
        let mut p1: u8 = 4; // Sample value for the second argument
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1202 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: isize = 5;
        let mut p1: u32 = 2;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1204 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static isize = &42;
        let mut p1: u32 = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1208 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &'static isize = &10;
        let mut p1: &'static usize = &5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1210 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<u8> = std::num::Wrapping(15);
        let mut p1: u8 = 5;

        <std::num::Wrapping<u8> as Pow<u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1211 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u8> = Wrapping(15);
        let p1: &u8 = &255;
                
        p0.pow(p1);

    }
}#[cfg(test)]
mod tests_rug_1212 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        // First argument
        let mut p0: Wrapping<u8> = Wrapping(15);

        // Second argument
        let mut p1: u8 = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1213 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u8> = Wrapping(15);
        let mut p1: u8 = 5;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1214 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u8> = Wrapping(15);
        let mut p1: usize = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1215 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u8> = Wrapping(15);
        let mut p1: &usize = &42;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1216_prepare {
    use std::num::Wrapping;
    
    #[test]
    fn sample() {
        let mut v15: Wrapping<u8> = Wrapping(15);
    }
}

#[cfg(test)]
mod tests_rug_1216 {
    use super::*;
    use std::num::Wrapping;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u8> = Wrapping(15);
        let mut p1: usize = 10;

        p0.pow(p1);
    }
}                        
#[cfg(test)]
mod tests_rug_1217 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u8> = Wrapping(15);
        let mut p1: usize = 10;
        
        p0.pow(&p1);
    }
}
                 #[cfg(test)]
mod tests_rug_1218 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: u8 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1219 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: u8 = 7;

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1220 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: &std::num::Wrapping<i8> = &std::num::Wrapping(42);
        let mut p1: u8 = 10;

        <&std::num::Wrapping<i8> as Pow<u8>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1221 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: &u8 = &42;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1222 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: usize = 7;
        
        <std::num::Wrapping<i8> as Pow<usize>>::pow(p0, p1);
    }
}                        
#[cfg(test)]
mod tests_rug_1223 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: &'static usize = &123;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1224 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: usize = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1225 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let mut p0: Wrapping<i8> = Wrapping(42);
        let mut p1: usize = 3;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1226_prepare {
    use std::num::Wrapping;

    #[test]
    fn sample() {
        let mut v16: Wrapping<u16> = Wrapping(42u16);
        let v8: u8 = 3u8;
    }
}

#[cfg(test)]
mod tests_rug_1226 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<u16> = std::num::Wrapping(42u16);
        let p1: u8 = 3u8;

        <std::num::Wrapping<u16> as Pow<u8>>::pow(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_1227 {
    use super::*;
    use crate::Pow;

    #[cfg(test)]
    mod tests_rug_1227_prepare {
        use std::num::Wrapping;
        #[test]
        fn sample() {
            let mut v16: Wrapping<u16> = Wrapping(42u16);
        }
    }

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<u16> = std::num::Wrapping(42u16);
        let mut p1: &u8 = &42u8;

        
        <std::num::Wrapping<u16> as Pow<&u8>>::pow(p0, p1);

        
    }
}
                            #[cfg(test)]
mod tests_rug_1228 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u16> = Wrapping(42u16);
        let mut p1: u8 = 8u8;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1229 {
    use std::num::Wrapping;

    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u16> = Wrapping(42u16);
        let mut p1: u8 = 8u8;

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1230 {
    use super::*;
    use crate::Pow;

    #[cfg(test)]
    mod tests_rug_1230_prepare {
        use std::num::Wrapping;

        #[test]
        fn sample() {
            let mut v16: Wrapping<u16> = Wrapping(42u16);
        }
    }

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u16> = Wrapping(42u16);
        let p1: usize = 10usize;

        <std::num::Wrapping<u16> as Pow<usize>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1231_prepare {
    use std::num::Wrapping;

    #[test]
    fn sample() {
        let mut v16: Wrapping<u16> = Wrapping(42u16);
    }
}

#[cfg(test)]
mod tests_rug_1231 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u16> = Wrapping(42u16);
        let mut p1: usize = 10usize;

        <std::num::Wrapping<u16>>::pow(p0, &p1);

    }
}#[cfg(test)]
mod tests_rug_1232 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u16> = Wrapping(42u16);
        let mut p1: usize = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1233 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u16> = Wrapping(42u16);
        let mut p1: usize = 7;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1234 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let mut p1: u8 = 7;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1235 {
    use super::*;
    use std::num::Wrapping;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let p1: u8 = 8;
        
        <std::num::Wrapping<i16> as Pow<&u8>>::pow(p0, &p1);
    
    }
}
#[cfg(test)]
mod tests_rug_1236 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let p0: Wrapping<i16> = Wrapping(42);
        let p1: u8 = 10;
        
        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1237 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        // 1st argument
        let mut p0: Wrapping<i16> = Wrapping(42);
        
        // 2nd argument
        let mut p1: u8 = 5;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1238_prepare {
    use std::num::Wrapping;
    
    #[test]
    fn sample() {
        let mut v17: Wrapping<i16> = Wrapping(42);
    }
}

#[cfg(test)]
mod tests_rug_1238 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let mut p1: usize = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1239_prepare {
    use std::num::Wrapping;
    
    #[test]
    fn sample() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let mut p1: usize = 10;
    }
}

#[cfg(test)]
mod tests_rug_1239 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let mut p1: usize = 10;
                
        <std::num::Wrapping<i16> as Pow<&usize>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1240 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let mut p1: usize = 4;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1241_prepare {
    use std::num::Wrapping;
    
    #[test]
    fn sample() {
        let mut v17: Wrapping<i16> = Wrapping(42);
        let mut v18: usize = 10;
    }
}

#[cfg(test)]
mod tests_rug_1241 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i16> = Wrapping(42);
        let mut p1: usize = 10;

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1242 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u32> = Wrapping(42);
        let mut p1: u8 = 5;
        <std::num::Wrapping<u32> as Pow<u8>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1243 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<u32> = std::num::Wrapping(42);
        let mut p1: &u8 = &10;

        <std::num::Wrapping<u32> as Pow<&u8>>::pow(p0, p1);
    }
}
                    
#[cfg(test)]
mod tests_rug_1244 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u32> = Wrapping(42);
        let mut p1: u8 = 7;
        
        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1245 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        // Sample data
        let mut v18: Wrapping<u32> = Wrapping(42);
        let v19: u8 = 8;

        v18.pow(&v19);
    }
}#[cfg(test)]
mod tests_rug_1246 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u32> = Wrapping(42);
        let mut p1: usize = 3;
                
        <std::num::Wrapping<u32> as Pow<usize>>::pow(p0, p1);

    }
}#[cfg(test)]
mod tests_rug_1247 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<u32> = std::num::Wrapping(42);
        let mut p1: &usize = &10;

        <std::num::Wrapping<u32> as Pow<&usize>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1248_prepare {
    use std::num::Wrapping;

    #[test]
    fn sample() {
        let mut v18: Wrapping<u32> = Wrapping(42);
    }
}

#[cfg(test)]
mod tests_rug_1248 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u32> = Wrapping(42);
        let p1: usize = 10;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1249 {
    use super::*;
    use std::num::Wrapping;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut v18: Wrapping<u32> = Wrapping(42);
        let mut p0: &Wrapping<u32> = &v18;
        let mut p1: &usize = &42;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1250 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i32> = Wrapping(42);
        let mut p1: u8 = 3;

        p0.pow(p1);

    }
}#[cfg(test)]
mod tests_rug_1251 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i32> = Wrapping(42);
        let mut p1: &'static u8 = &7;
        
        <std::num::Wrapping<i32> as Pow<&u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1252 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        // Constructing the first argument: Wrapping<i32>
        #[cfg(test)]
        mod tests_rug_1252_prepare {
            use std::num::Wrapping;

            #[test]
            fn sample() {
                let mut p0: Wrapping<i32> = Wrapping(42);
            }
        }
        
        let p0: Wrapping<i32> = Wrapping(42);
        
        // Constructing the second argument: u8
        let p1: u8 = 7;
        
        p0.pow(p1);
    }
}        #[cfg(test)]
        mod tests_rug_1253 {
            use super::*;
            use crate::Pow;
            use std::num::Wrapping;

            #[test]
            fn test_rug() {
                let mut p0: Wrapping<i32> = Wrapping(42);
                let mut p1: u8 = 5;

                p0.pow(&p1);
            }
        }
#[cfg(test)]
mod tests_rug_1254 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0 = std::num::Wrapping(42);
        let mut p1: usize = 5;

        <std::num::Wrapping<i32>>::pow(p0, p1);

    }
}
                    
#[cfg(test)]
mod tests_rug_1255 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i32> = Wrapping(42);
        let mut p1: usize = 10;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1256 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let p0: Wrapping<i32> = Wrapping(42);
        let p1: usize = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1257 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i32> = Wrapping(42);
        let mut p1: usize = 3;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1258 {
    use super::*;
    use std::num::Wrapping;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u64> = Wrapping(20);
        let mut p1: u8 = 3;
        
        <std::num::Wrapping<u64> as Pow<u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1259 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<u64> = std::num::Wrapping(20);
        let p1: &u8 = &8;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1260 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u64> = Wrapping(20);
        let mut p1: u8 = 5;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1261 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0 = std::num::Wrapping(20);
        let mut p1: u8 = 5;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1262 {
    use super::*;
    use std::num::Wrapping;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u64> = Wrapping(20);
        let mut p1: usize = 2;
        
        (<std::num::Wrapping<u64> as Pow<usize>>::pow)(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1263_prepare {
    use std::num::Wrapping;

    #[test]
    fn sample() {
        let mut v20: Wrapping<u64> = Wrapping(20);
    }
}

#[cfg(test)]
mod tests_rug_1263 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u64> = Wrapping(20);
        let mut p1: &usize = &10;

        <std::num::Wrapping<u64> as Pow<&usize>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1264 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u64> = Wrapping(20);
        let mut p1: usize = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1265 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let p0: Wrapping<u64> = Wrapping(20);
        let p1: usize = 10;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1266 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i64> = Wrapping(42);
        let mut p1: u8 = 10;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1267 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<i64> = std::num::Wrapping(42);
        let mut p1: u8 = 7;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1268 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i64> = Wrapping(42);
        let mut p1: u8 = 10;
        
        p0.pow(p1);
        
    }
}#[cfg(test)]
mod tests_rug_1270 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let p0: std::num::Wrapping<i64> = Wrapping(42);
        let p1: usize = 10;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1271 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<i64> = std::num::Wrapping(42);
        let mut p1: &usize = &123;

        <std::num::Wrapping<i64> as Pow<&usize>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1272 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i64> = Wrapping(42);
        let mut p1: usize = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1273 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i64> = Wrapping(42);
        let mut p1: usize = 10;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1274 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        // Initialize the first argument
        #[allow(unused_variables)]
        let mut p0: std::num::Wrapping<u128> = std::num::Wrapping(42); // Sample data

        // Initialize the second argument
        #[allow(unused_variables)]
        let mut p1: u8 = 10; // Sample data

        <std::num::Wrapping<u128> as Pow<u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1275 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_pow() {
        let mut p0: std::num::Wrapping<u128> = std::num::Wrapping(42); // sample data
        let mut p1: &u8 = &10; // sample data

        std::num::Wrapping::<u128>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1276 {
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let v6: Wrapping<u128> = Wrapping(42);
        let p1: u8 = 8;
        
        v6.pow(p1);
        
    }
}
#[cfg(test)]
mod tests_rug_1277 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u128> = Wrapping(42);
        let mut p1: u8 = 5;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1278 {
    use std::num::Wrapping;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u128> = Wrapping(42); // Sample data
        let mut p1: usize = 4; // Sample data

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1279 {
    use super::*;
    use crate::Pow;

    #[cfg(test)]
    mod tests_rug_1279_prepare {
        use std::num::Wrapping;

        #[test]
        fn sample() {
            let mut p0: Wrapping<u128> = Wrapping(42); // sample data
            // Create the local variable p0 with type std::num::Wrapping<u128>
        }
    }

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u128> = Wrapping(42); // sample data
        let mut p1: usize = 10; // sample data

        <std::num::Wrapping<u128>>::pow(p0, &p1);
    }
}
#[cfg(test)]
mod tests_rug_1280 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u128> = Wrapping(42);
        let mut p1: usize = 10;
                
        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1281 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<u128> = Wrapping(42);
        let mut p1: usize = 10;

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1282 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: u8 = 7;

        p0.pow(p1);

    }
}
#[cfg(test)]
mod tests_rug_1283 {
    use super::*;
    use crate::Pow;

    #[cfg(test)]
    mod tests_rug_1283_prepare {
        use std::num::Wrapping;

        #[test]
        fn sample() {
            let mut p0: Wrapping<i128> = Wrapping(42);
            let mut p1: u8 = 4;
            // Additional code here, if necessary
        }
    }

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: u8 = 4;

        <std::num::Wrapping<i128> as Pow<&u8>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1284 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: u8 = 7;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1285 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        #[cfg(test)]
        mod tests_rug_1285_prepare {
            use std::num::Wrapping;

            #[test]
            fn sample() {
                let mut v22: Wrapping<i128> = Wrapping(42);
                // Additional code here, if necessary
            }
        }
        
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: u8 = 3;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1286 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: usize = 10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1287 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: &usize = &0;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1288 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: &std::num::Wrapping<i128> = &std::num::Wrapping(42);
        let mut p1: usize = 5;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1289 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<i128> = Wrapping(42);
        let mut p1: usize = 5;
        
        p0.pow(&p1);

    }
}#[cfg(test)]
mod tests_rug_1290 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<usize> = Wrapping(23);
        let mut p1: u8 = 5;

        <Wrapping<usize> as Pow<u8>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1291 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut v23: Wrapping<usize> = Wrapping(23);
        let mut p1: u8 = 5;

        v23.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1292 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<usize> = Wrapping(23);
        let mut p1: u8 = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1293 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        // 1. Construct the first argument
        let mut p0: Wrapping<usize> = Wrapping(23);
        
        // 2. Construct the second argument
        let mut p1: u8 = 42;
        
        // Call the target function
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1294 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<usize> = Wrapping(23);
        let mut p1: usize = 5;

        <std::num::Wrapping<usize> as Pow<usize>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1295 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        // Construct the p0 argument
        let mut v23: Wrapping<usize> = Wrapping(23);

        // Construct the p1 argument
        let p1: &usize = &42;

        // Call the pow function
        <std::num::Wrapping<usize>>::pow(v23, p1);
    }
}#[cfg(test)]
mod tests_rug_1296 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<usize> = Wrapping(23);
        let mut p1: usize = 5;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1297 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<usize> = Wrapping(23);
        let mut p1: usize = 5;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1298 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<isize> = std::num::Wrapping(42);
        let p1: u8 = 5;

        <std::num::Wrapping<isize> as Pow<u8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1299 {
    use std::num::Wrapping;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<isize> = Wrapping(42);
        let mut p1: u8 = 5;
        
        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1300 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<isize> = Wrapping(42);
        let mut p1: u8 = 5;
        
        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1301 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<isize> = Wrapping(42);
        let mut p1: u8 = 10;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1302 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: std::num::Wrapping<isize> = std::num::Wrapping(42);
        let p1: usize = 5;

        <std::num::Wrapping<isize> as Pow<usize>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1303 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;

    #[test]
    fn test_rug() {
        let mut p0: Wrapping<isize> = Wrapping(42);
        let mut p1: &usize = &3;

        <std::num::Wrapping<isize> as Pow<&usize>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1304 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    
    #[test]
    fn test_rug() {
        let mut p0: Wrapping<isize> = Wrapping(42);
        let mut p1: usize = 10;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1305 {
    use super::*;
    use crate::{Pow, Wrapping};

    #[test]
    fn test_pow() {
        let mut p0: Wrapping<isize> = Wrapping(42);
        let mut p1: &usize = &10;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1306 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: i8 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1307 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: i8 = 3;
        
        <f32 as Pow<&i8>>::pow(p0, &p1);
    }
}
#[cfg(test)]
mod tests_rug_1309 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.0;
        let mut p1: i8 = 3;

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1310 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: u8 = 3;

        <f32>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1311 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: u8 = 3;

        <f32 as Pow<&u8>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1312 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: u8 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1313 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: u8 = 3;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1314 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.0;
        let mut p1: i16 = 3;

        f32::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1315 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: f32 = 10.0;
        let p1: i16 = 5;

        <f32>::pow(p0, &p1);
    }
}        
        #[cfg(test)]
        mod tests_rug_1316 {
            use super::*;
            use crate::Pow;
            #[test]
            fn test_rug() {
                let p0: f32 = 3.14;
                let p1: i16 = -2;

                p0.pow(p1);
            }
        }
                            #[cfg(test)]
mod tests_rug_1317 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.0;
        let mut p1: i16 = 3;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1318 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let p0: f32 = 2.5;
        let p1: u16 = 3;

        f32::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1321 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5; // Sample data for the first argument
        let mut p1: u16 = 3; // Sample data for the second argument

        p0.pow(&p1);
    }
}#[cfg(test)]
        mod tests_rug_1322 {
            use super::*;
            use crate::Pow;
            #[test]
            fn test_rug() {
                let mut p0: f32 = 3.14;
                let mut p1: i32 = 2;

                
                <f32 as Pow<i32>>::pow(p0, p1);

            }
        }#[cfg(test)]
mod tests_rug_1323 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 3.5_f32;
        let mut p1: &'static i32 = &5_i32;
        
        <f32 as Pow<&'static i32>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1325 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: f32 = 2.5;
        let p1: i32 = 4;

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1326 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: i8 = 3;

        <f64 as Pow<i8>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1328 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: i8 = 3;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1330 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.5;
        let mut p1: u8 = 3;

        <f64>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1331 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: &'static u8 = &1;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1332 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: u8 = 3;

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1334 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: f64 = 2.0;
        let p1: i16 = 3;

        <f64 as Pow<i16>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1335 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0; // Sample value for f64
        let mut p1: i16 = -1; // Sample value for i16

        <f64 as Pow<&i16>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1336 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: i16 = 3;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1337 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        // Initialize sample data
        let p0: f64 = 2.0;
        let p1: i16 = 3;
        
        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1338 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 3.14; // sample
        let mut p1: u16 = 2; // sample

        p0.pow(p1);
    }
}#[cfg(test)]
        mod tests_rug_1339 {
            use super::*;
            use crate::Pow;
            #[test]
            fn test_rug() {
                let p0: f64 = 10.0;
                let p1: u16 = 2;
                                
                <f64>::pow(p0, &p1);

            }
        }#[cfg(test)]
mod tests_rug_1340 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 10.0; // sample data for the first argument
        let mut p1: u16 = 2; // sample data for the second argument
        
        p0.pow(p1);
    }
}                    
#[cfg(test)]
mod tests_rug_1341 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: u16 = 3;
        
        p0.pow(&p1);

    }
}
#[cfg(test)]
mod tests_rug_1342 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: i32 = 3;
        
        <f64 as Pow<i32>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1343 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: i32 = 3;

        <f64 as Pow<&i32>>::pow(p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1344 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: i32 = 3;

        p0.pow(p1);
    }
}
#[cfg(test)]
mod tests_rug_1345 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 10.5; // Sample data for the first argument
        let mut p1: i32 = 3; // Sample data for the second argument

        p0.pow(&p1);
    }
}
#[cfg(test)]
mod tests_rug_1346 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.5;
        let mut p1: f32 = 1.8;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1347 {
    use super::*;
    use crate::Pow;
    use std::f32;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 2.0;
        let mut p1: f32 = 3.0;
        
        f32::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1348 {
    use super::*;
    use crate::pow::Pow;

    #[test]
    fn test_pow() {
        let mut p0: f32 = 2.0;
        let mut p1: f32 = 3.0;

        <&f32 as Pow<f32>>::pow(&p0, p1);
    }
}
#[cfg(test)]
        mod tests_rug_1349 {
            use super::*;
            use crate::Pow;
            
            #[test]
            fn test_rug() {
                let mut p0: f32 = 2.5;
                let mut p1: f32 = 3.7;
                
                p0.pow(&p1);
            }
        }#[cfg(test)]
mod tests_rug_1350 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0; // sample value
        let mut p1: f32 = 1.5; // sample value
                
        <f64>::pow(p0, p1);
    }
}        
        #[cfg(test)]
        mod tests_rug_1351 {
            use super::*;
            use crate::Pow;
            #[test]
            fn test_rug() {
                let mut p0: f64 = 2.0;
                let mut p1: f32 = 3.0;
                
                <f64 as Pow<&f32>>::pow(p0, &p1);

            }
        }
                                
#[cfg(test)]
mod tests_rug_1352 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let p0: f64 = 10.0;  // Sample data for the first argument
        
        let p1: f32 = 5.0;  // Sample data for the second argument

        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1353 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0 = 2.5;
        let mut p1 = 1.7;

        p0.pow(&p1);
    }
}#[cfg(test)]
mod tests_rug_1354 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: f64 = 3.0;

        <f64 as Pow<f64>>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1355 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.0;
        let mut p1: f64 = 3.0;

        <f64>::pow(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1356 {
    use super::*;
    use crate::Pow;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.5;
        let mut p1: f64 = 3.5;
        
        p0.pow(p1);
    }
}#[cfg(test)]
mod tests_rug_1357 {
    use super::*;
    use crate::Pow;

    #[test]
    fn test_rug() {
        let p0: f64 = 2.0; // Sample data for the first argument
        let p1: f64 = 3.0; // Sample data for the second argument

        <&f64>::pow(&p0, &p1);
    }
}