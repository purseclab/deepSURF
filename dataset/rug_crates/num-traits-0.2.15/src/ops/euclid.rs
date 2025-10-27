use core::ops::{Div, Rem};

pub trait Euclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> {
    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that
    /// `self = n * v + self.rem_euclid(v)`.
    /// In other words, the result is `self / v` rounded to the integer `n`
    /// such that `self >= n * v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Euclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(Euclid::div_euclid(&a, &b), 1); // 7 > 4 * 1
    /// assert_eq!(Euclid::div_euclid(&-a, &b), -2); // -7 >= 4 * -2
    /// assert_eq!(Euclid::div_euclid(&a, &-b), -1); // 7 >= -4 * -1
    /// assert_eq!(Euclid::div_euclid(&-a, &-b), 2); // -7 >= -4 * 2
    /// ```
    fn div_euclid(&self, v: &Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod v)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < v.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == v.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `v.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(v) * v + self.rem_euclid(v)`
    /// approximatively.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Euclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(Euclid::rem_euclid(&a, &b), 3);
    /// assert_eq!(Euclid::rem_euclid(&-a, &b), 1);
    /// assert_eq!(Euclid::rem_euclid(&a, &-b), 3);
    /// assert_eq!(Euclid::rem_euclid(&-a, &-b), 1);
    /// ```
    fn rem_euclid(&self, v: &Self) -> Self;
}

macro_rules! euclid_forward_impl {
    ($($t:ty)*) => {$(
        #[cfg(has_div_euclid)]
        impl Euclid for $t {
            #[inline]
            fn div_euclid(&self, v: &$t) -> Self {
                <$t>::div_euclid(*self, *v)
            }

            #[inline]
            fn rem_euclid(&self, v: &$t) -> Self {
                <$t>::rem_euclid(*self, *v)
            }
        }
    )*}
}

macro_rules! euclid_int_impl {
    ($($t:ty)*) => {$(
        euclid_forward_impl!($t);

        #[cfg(not(has_div_euclid))]
        impl Euclid for $t {
            #[inline]
            fn div_euclid(&self, v: &$t) -> Self {
                let q = self / v;
                if self % v < 0 {
                    return if *v > 0 { q - 1 } else { q + 1 }
                }
                q
            }

            #[inline]
            fn rem_euclid(&self, v: &$t) -> Self {
                let r = self % v;
                if r < 0 {
                    if *v < 0 {
                        r - v
                    } else {
                        r + v
                    }
                } else {
                    r
                }
            }
        }
    )*}
}

macro_rules! euclid_uint_impl {
    ($($t:ty)*) => {$(
        euclid_forward_impl!($t);

        #[cfg(not(has_div_euclid))]
        impl Euclid for $t {
            #[inline]
            fn div_euclid(&self, v: &$t) -> Self {
                self / v
            }

            #[inline]
            fn rem_euclid(&self, v: &$t) -> Self {
                self % v
            }
        }
    )*}
}

euclid_int_impl!(isize i8 i16 i32 i64 i128);
euclid_uint_impl!(usize u8 u16 u32 u64 u128);

#[cfg(all(has_div_euclid, feature = "std"))]
euclid_forward_impl!(f32 f64);

#[cfg(not(all(has_div_euclid, feature = "std")))]
impl Euclid for f32 {
    #[inline]
    fn div_euclid(&self, v: &f32) -> f32 {
        let q = <f32 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    fn rem_euclid(&self, v: &f32) -> f32 {
        let r = self % v;
        if r < 0.0 {
            r + <f32 as crate::float::FloatCore>::abs(*v)
        } else {
            r
        }
    }
}

#[cfg(not(all(has_div_euclid, feature = "std")))]
impl Euclid for f64 {
    #[inline]
    fn div_euclid(&self, v: &f64) -> f64 {
        let q = <f64 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    fn rem_euclid(&self, v: &f64) -> f64 {
        let r = self % v;
        if r < 0.0 {
            r + <f64 as crate::float::FloatCore>::abs(*v)
        } else {
            r
        }
    }
}

pub trait CheckedEuclid: Euclid {
    /// Performs euclid division that returns `None` instead of panicking on division by zero
    /// and instead of wrapping around on underflow and overflow.
    fn checked_div_euclid(&self, v: &Self) -> Option<Self>;

    /// Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and
    /// division by zero. If any of that happens, `None` is returned.
    fn checked_rem_euclid(&self, v: &Self) -> Option<Self>;
}

macro_rules! checked_euclid_forward_impl {
    ($($t:ty)*) => {$(
        #[cfg(has_div_euclid)]
        impl CheckedEuclid for $t {
            #[inline]
            fn checked_div_euclid(&self, v: &$t) -> Option<Self> {
                <$t>::checked_div_euclid(*self, *v)
            }

            #[inline]
            fn checked_rem_euclid(&self, v: &$t) -> Option<Self> {
                <$t>::checked_rem_euclid(*self, *v)
            }
        }
    )*}
}

macro_rules! checked_euclid_int_impl {
    ($($t:ty)*) => {$(
        checked_euclid_forward_impl!($t);

        #[cfg(not(has_div_euclid))]
        impl CheckedEuclid for $t {
            #[inline]
            fn checked_div_euclid(&self, v: &$t) -> Option<$t> {
                if *v == 0 || (*self == Self::min_value() && *v == -1) {
                    None
                } else {
                    Some(Euclid::div_euclid(self, v))
                }
            }

            #[inline]
            fn checked_rem_euclid(&self, v: &$t) -> Option<$t> {
                if *v == 0 || (*self == Self::min_value() && *v == -1) {
                    None
                } else {
                    Some(Euclid::rem_euclid(self, v))
                }
            }
        }
    )*}
}

macro_rules! checked_euclid_uint_impl {
    ($($t:ty)*) => {$(
        checked_euclid_forward_impl!($t);

        #[cfg(not(has_div_euclid))]
        impl CheckedEuclid for $t {
            #[inline]
            fn checked_div_euclid(&self, v: &$t) -> Option<$t> {
                if *v == 0 {
                    None
                } else {
                    Some(Euclid::div_euclid(self, v))
                }
            }

            #[inline]
            fn checked_rem_euclid(&self, v: &$t) -> Option<$t> {
                if *v == 0 {
                    None
                } else {
                    Some(Euclid::rem_euclid(self, v))
                }
            }
        }
    )*}
}

checked_euclid_int_impl!(isize i8 i16 i32 i64 i128);
checked_euclid_uint_impl!(usize u8 u16 u32 u64 u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclid_unsigned() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 10;
                        let y: $t = 3;
                        assert_eq!(Euclid::div_euclid(&x, &y), 3);
                        assert_eq!(Euclid::rem_euclid(&x, &y), 1);
                    }
                )+
            };
        }

        test_euclid!(usize u8 u16 u32 u64);
    }

    #[test]
    fn euclid_signed() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 10;
                        let y: $t = -3;
                        assert_eq!(Euclid::div_euclid(&x, &y), -3);
                        assert_eq!(Euclid::div_euclid(&-x, &y), 4);
                        assert_eq!(Euclid::rem_euclid(&x, &y), 1);
                        assert_eq!(Euclid::rem_euclid(&-x, &y), 2);
                        let x: $t = $t::min_value() + 1;
                        let y: $t = -1;
                        assert_eq!(Euclid::div_euclid(&x, &y), $t::max_value());
                    }
                )+
            };
        }

        test_euclid!(isize i8 i16 i32 i64 i128);
    }

    #[test]
    fn euclid_float() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 12.1;
                        let y: $t = 3.2;
                        assert!(Euclid::div_euclid(&x, &y) * y + Euclid::rem_euclid(&x, &y) - x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert!(Euclid::div_euclid(&x, &-y) * -y + Euclid::rem_euclid(&x, &-y) - x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert!(Euclid::div_euclid(&-x, &y) * y + Euclid::rem_euclid(&-x, &y) + x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert!(Euclid::div_euclid(&-x, &-y) * -y + Euclid::rem_euclid(&-x, &-y) + x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                    }
                )+
            };
        }

        test_euclid!(f32 f64);
    }

    #[test]
    fn euclid_checked() {
        macro_rules! test_euclid_checked {
            ($($t:ident)+) => {
                $(
                    {
                        assert_eq!(CheckedEuclid::checked_div_euclid(&$t::min_value(), &-1), None);
                        assert_eq!(CheckedEuclid::checked_rem_euclid(&$t::min_value(), &-1), None);
                        assert_eq!(CheckedEuclid::checked_div_euclid(&1, &0), None);
                        assert_eq!(CheckedEuclid::checked_rem_euclid(&1, &0), None);
                    }
                )+
            };
        }

        test_euclid_checked!(isize i8 i16 i32 i64 i128);
    }
}
#[cfg(test)]
mod tests_rug_1599 {
    use super::*;
    use crate::Euclid;

    #[test]
    fn test_rug() {
        let mut p0: isize = 10;
        let mut p1: isize = 3;

        <isize>::div_euclid(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_1600 {
    use super::*;
    use crate::Euclid;
    
    #[test]
    fn test_rug() {
        let mut p0 = 10;
        let mut p1 = 3;
        
        <isize>::rem_euclid(p0, p1);
    }
}#[cfg(test)]
        mod tests_rug_1603 {
            use super::*;
            use crate::Euclid;
            #[test]
            fn test_rug() {
                let mut p0: i16 = 10;
                let mut p1: i16 = 3;

                
                <i16>::div_euclid(p0, p1);

            }
        }#[cfg(test)]
mod tests_rug_1606 {
    use super::*;
    use crate::Euclid;

    #[test]
    fn test_rug() {
        let mut p0 = 10;
        let mut p1 = 3;

        <i32>::rem_euclid(p0, p1);

    }
}        
#[cfg(test)]
mod tests_rug_1608 {
    use super::*;
    use crate::Euclid;
    
    #[test]
    fn test_rug() {
        let mut p0: i64 = 10;
        let mut p1: i64 = 3;
        
        <i64>::rem_euclid(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_1609 {
    use super::*;
    use crate::ops::euclid::Euclid;

    #[test]
    fn test_rug() {
        let mut p0: i128 = 10;
        let mut p1: i128 = 3;

        <i128 as Euclid>::div_euclid(&p0, &p1);
        
    }
}#[cfg(test)]
mod tests_rug_1611 {
    use super::*;
    use crate::Euclid;
    
    #[test]
    fn test_rug() {
        let mut p0: usize = 10;
        let mut p1: usize = 3;

        <usize>::div_euclid(p0, p1);

    }
}#[cfg(test)]
mod tests_rug_1623 {
    use super::*;
    use crate::Euclid;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 10.5;
        let mut p1: f32 = 2.5;

        <f32 as Euclid>::div_euclid(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1624 {
    use super::*;
    use crate::Euclid;

    #[test]
    fn test_rug() {
        let mut p0: f32 = 10.5;
        let mut p1: f32 = 3.2;

        <f32>::rem_euclid(p0, p1);
    }
}                        
#[cfg(test)]
mod tests_rug_1628 {
    use super::*;
    use crate::CheckedEuclid;
    
    #[test]
    fn test_checked_rem_euclid() {
        
        let p0: isize = 10;
        let p1: isize = 3;
        
        let result = isize::checked_rem_euclid(p0, p1);
        
        assert_eq!(result, Some(1));
    }
}
            #[cfg(test)]
mod tests_rug_1635 {
    use super::*;
    use crate::CheckedEuclid;

    #[test]
    fn test_rug() {
        let mut p0 = 42i64;
        let mut p1 = 6i64;

        <i64 as CheckedEuclid>::checked_div_euclid(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1643 {
    use super::*;
    use crate::CheckedEuclid;

    #[test]
    fn test_rug() {
        let p0: u16 = 42;
        let p1: u16 = 7;

        <u16 as CheckedEuclid>::checked_div_euclid(&p0, &p1);
    }
}#[cfg(test)]
mod tests_rug_1645 {
    use super::*;
    use crate::ops::euclid::CheckedEuclid;

    #[test]
    fn test_checked_div_euclid() {
        let p0: u32 = 20;
        let p1: u32 = 5;
        
        <u32 as CheckedEuclid>::checked_div_euclid(&p0, &p1);
    }
}        
#[cfg(test)]
mod tests_rug_1650 {
    use super::*;
    use crate::CheckedEuclid;

    #[test]
    fn test_rug() {
        // Sample data
        let p0: u128 = 10;
        let p1: u128 = 3;

        <u128 as CheckedEuclid>::checked_rem_euclid(&p0, &p1);
    }
}