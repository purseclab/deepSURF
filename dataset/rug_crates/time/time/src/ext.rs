//! Extension traits.

use core::time::Duration as StdDuration;

use crate::convert::*;
use crate::Duration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}
    impl Sealed for i64 {}
    impl Sealed for u64 {}
    impl Sealed for f64 {}
}

// region: NumericalDuration
/// Create [`Duration`]s from numeric literals.
///
/// # Examples
///
/// Basic construction of [`Duration`]s.
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
/// assert_eq!(5.microseconds(), Duration::microseconds(5));
/// assert_eq!(5.milliseconds(), Duration::milliseconds(5));
/// assert_eq!(5.seconds(), Duration::seconds(5));
/// assert_eq!(5.minutes(), Duration::minutes(5));
/// assert_eq!(5.hours(), Duration::hours(5));
/// assert_eq!(5.days(), Duration::days(5));
/// assert_eq!(5.weeks(), Duration::weeks(5));
/// ```
///
/// Signed integers work as well!
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Duration::microseconds(-5));
/// assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
/// assert_eq!((-5).seconds(), Duration::seconds(-5));
/// assert_eq!((-5).minutes(), Duration::minutes(-5));
/// assert_eq!((-5).hours(), Duration::hours(-5));
/// assert_eq!((-5).days(), Duration::days(-5));
/// assert_eq!((-5).weeks(), Duration::weeks(-5));
/// ```
///
/// Just like any other [`Duration`], they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalDuration;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point value will be
/// truncated. Keep in mind that floating point numbers are inherently imprecise and have limited
/// capacity.
pub trait NumericalDuration: sealed::Sealed {
    /// Create a [`Duration`] from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a [`Duration`] from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a [`Duration`] from the number of hours.
    fn hours(self) -> Duration;
    /// Create a [`Duration`] from the number of days.
    fn days(self) -> Duration;
    /// Create a [`Duration`] from the number of weeks.
    fn weeks(self) -> Duration;
}

impl NumericalDuration for i64 {
    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self)
    }

    fn microseconds(self) -> Duration {
        Duration::microseconds(self)
    }

    fn milliseconds(self) -> Duration {
        Duration::milliseconds(self)
    }

    fn seconds(self) -> Duration {
        Duration::seconds(self)
    }

    fn minutes(self) -> Duration {
        Duration::minutes(self)
    }

    fn hours(self) -> Duration {
        Duration::hours(self)
    }

    fn days(self) -> Duration {
        Duration::days(self)
    }

    fn weeks(self) -> Duration {
        Duration::weeks(self)
    }
}

impl NumericalDuration for f64 {
    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self as _)
    }

    fn microseconds(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Microsecond) as Self) as _)
    }

    fn milliseconds(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Millisecond) as Self) as _)
    }

    fn seconds(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Second) as Self) as _)
    }

    fn minutes(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Minute) as Self) as _)
    }

    fn hours(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Hour) as Self) as _)
    }

    fn days(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Day) as Self) as _)
    }

    fn weeks(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Week) as Self) as _)
    }
}
// endregion NumericalDuration

// region: NumericalStdDuration
/// Create [`std::time::Duration`]s from numeric literals.
///
/// # Examples
///
/// Basic construction of [`std::time::Duration`]s.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// # use core::time::Duration;
/// assert_eq!(5.std_nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.std_microseconds(), Duration::from_micros(5));
/// assert_eq!(5.std_milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.std_seconds(), Duration::from_secs(5));
/// assert_eq!(5.std_minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.std_hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.std_days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.std_weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other [`std::time::Duration`], they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// assert_eq!(
///     2.std_seconds() + 500.std_milliseconds(),
///     2_500.std_milliseconds()
/// );
/// assert_eq!(
///     2.std_seconds() - 500.std_milliseconds(),
///     1_500.std_milliseconds()
/// );
/// ```
///
/// When called on floating point values, any remainder of the floating point value will be
/// truncated. Keep in mind that floating point numbers are inherently imprecise and have limited
/// capacity.
pub trait NumericalStdDuration: sealed::Sealed {
    /// Create a [`std::time::Duration`] from the number of nanoseconds.
    fn std_nanoseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of microseconds.
    fn std_microseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of milliseconds.
    fn std_milliseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of seconds.
    fn std_seconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of minutes.
    fn std_minutes(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of hours.
    fn std_hours(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of days.
    fn std_days(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of weeks.
    fn std_weeks(self) -> StdDuration;
}

impl NumericalStdDuration for u64 {
    fn std_nanoseconds(self) -> StdDuration {
        StdDuration::from_nanos(self)
    }

    fn std_microseconds(self) -> StdDuration {
        StdDuration::from_micros(self)
    }

    fn std_milliseconds(self) -> StdDuration {
        StdDuration::from_millis(self)
    }

    fn std_seconds(self) -> StdDuration {
        StdDuration::from_secs(self)
    }

    fn std_minutes(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Minute) as Self)
    }

    fn std_hours(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Hour) as Self)
    }

    fn std_days(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Day) as Self)
    }

    fn std_weeks(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Week) as Self)
    }
}

impl NumericalStdDuration for f64 {
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as _)
    }

    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Microsecond) as Self) as _)
    }

    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Millisecond) as Self) as _)
    }

    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Second) as Self) as _)
    }

    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Minute) as Self) as _)
    }

    fn std_hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Hour) as Self) as _)
    }

    fn std_days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Day) as Self) as _)
    }

    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Week) as Self) as _)
    }
}
// endregion NumericalStdDuration
#[cfg(test)]
mod tests_rug_124 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::Duration;
    
    #[test]
    fn test_microseconds() {
        let p0: i64 = 100; // Sample data
        
        let _ = <i64 as NumericalDuration>::microseconds(p0);
    }
}#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::ext::NumericalDuration;

    #[test]
    fn test_milliseconds() {
        let mut p0: i64 = 1000;
        p0.milliseconds();
    }
}#[cfg(test)]
        mod tests_rug_126 {
            use super::*;
            use crate::ext::NumericalDuration;

            use crate::Duration;

            #[test]
            fn test_rug() {
                let mut p0: i64 = 10;

                p0.seconds();
            }
        }#[cfg(test)]
mod tests_rug_127 {
    use super::*;
    use crate::ext::NumericalDuration;

    use std::time::Duration;

    #[test]
    fn test_minutes() {
        let p0: i64 = 60;

        p0.minutes();
        
        // Add assertion here if needed
    }
}#[cfg(test)]
mod tests_rug_128 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::Duration;
    
    #[test]
    fn test_rug() {
        let mut p0: i64 = 10;
        
        p0.hours();
    
    }
}
#[cfg(test)]
mod tests_rug_130 {
    use super::*;
    use crate::ext::NumericalDuration;
    use std::time::Duration;
    
    #[test]
    fn test_weeks() {
        let p0: i64 = 2;
        <i64 as NumericalDuration>::weeks(p0);
    }
}
    #[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use crate::ext::NumericalDuration;
    use std::time::Duration;

    #[test]
    fn test_rug() {
        let p0: f64 = 10.5;
        
        <f64>::nanoseconds(p0);
    }
}        
#[cfg(test)]
mod tests_rug_132 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::ext::Duration;
    use crate::ext::Nanosecond;
    use crate::ext::Microsecond;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 1.23;
        p0.microseconds();
    }
}
#[cfg(test)]
mod tests_rug_133 {
    use super::*;
    use crate::ext::NumericalDuration;
    
    #[test]
    fn test_rug() {
        let p0: f64 = 2.5;
        
        p0.milliseconds();
    }
}#[cfg(test)]
mod tests_rug_134 {
    use super::*;
    use crate::ext::NumericalDuration;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 1.23;
        <f64>::seconds(p0);
    }
}#[cfg(test)]
mod tests_rug_135 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::ext::Duration;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.5;

        p0.minutes();
    }
}#[cfg(test)]
mod tests_rug_136 {
    use super::*;
    use crate::ext::NumericalDuration;

    #[test]
    fn test_hours() {
        let p0: f64 = 2.5;

        <f64>::hours(p0);
    }
}#[cfg(test)]
mod tests_rug_137 {
    use super::*;
    use crate::ext::NumericalDuration;

    #[test]
    fn test_days() {
        let p0: f64 = 2.5;

        <f64>::days(p0);
    }
}#[cfg(test)]
mod tests_rug_138 {
    use super::*;
    use crate::ext::NumericalDuration;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.5;

        p0.weeks();
    }
}#[cfg(test)]
mod tests_rug_139 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_std_nanoseconds() {
        let p0: u64 = 1234567890;
        
        p0.std_nanoseconds();
    }
}
#[cfg(test)]
mod tests_rug_140 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;

    #[test]
    fn test_std_microseconds() {
        let p0: u64 = 100;

        let result = p0.std_microseconds();

        assert_eq!(result, Duration::from_micros(100));
    }
}
#[cfg(test)]
mod tests_rug_141 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_rug() {
        let p0: u64 = 1000; // Sample data
        
        <u64 as NumericalStdDuration>::std_milliseconds(p0);
    }
}#[cfg(test)]
mod tests_rug_142 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_rug() {
        let p0: u64 = 12345;
        
        assert_eq!(p0.std_seconds(), Duration::from_secs(p0));
    }
}        
#[cfg(test)]
mod tests_rug_143 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    
    #[test]
    fn test_rug() {
        let p0: u64 = 500;  // Sample data
        
        <u64>::std_minutes(p0);
    }
}#[cfg(test)]
mod tests_rug_144 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_std_hours() {
        let p0: u64 = 3600;
        
        let result = p0.std_hours();
        
        assert_eq!(result, Duration::from_secs(p0 * 3600));
    }
}
#[cfg(test)]
mod tests_rug_145 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_std_days() {
        let p0: u64 = 10;
        
        p0.std_days();
    }
}
#[cfg(test)]
mod tests_rug_146 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;

    #[test]
    fn test_std_weeks() {
        let p0: u64 = 7;

        assert_eq!(p0.std_weeks(), Duration::from_secs(604800));
    }
}#[cfg(test)]
mod tests_rug_147 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_rug() {
        let p0: f64 = 2.5;
        
        <f64 as NumericalStdDuration>::std_nanoseconds(p0);
    }
}        
#[cfg(test)]
mod tests_rug_148 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 2.5; // Sample data initialization
        
        p0.std_microseconds();
    }
}
    #[cfg(test)]
mod tests_rug_149 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    
    #[test]
    fn test_std_milliseconds() {
        let p0: f64 = 1.234;
        
        p0.std_milliseconds();

        let p1: f64 = 0.0;
        
        p1.std_milliseconds();

        let p2: f64 = 100.567;
        
        p2.std_milliseconds();
    }
}                    
        #[cfg(test)]
        mod tests_rug_150 {
            use super::*;
            use crate::ext::NumericalStdDuration;
            
            #[test]
            fn test_rug() {
                let mut p0: f64 = 3.14;
                
                <f64>::std_seconds(p0);

            }
        }
                            #[cfg(test)]
mod tests_rug_151 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use crate::ext::{Nanosecond, Minute};
    use std::time::Duration as StdDuration;
    
    #[test]
    fn test_rug() {
        let mut p0: f64 = 1.5;

        <f64>::std_minutes(p0);
    }
}#[cfg(test)]
mod tests_rug_152 {
    use super::*;
    use crate::ext::NumericalStdDuration;

    #[test]
    fn test_rug() {
        let mut p0: f64 = 10.5;

        p0.std_hours();

    }
}#[cfg(test)]
mod tests_rug_153 {
    use super::*;
    use crate::ext::NumericalStdDuration;

    #[test]
    fn test_rug() {
        let p0: f64 = 1.5;

        <f64>::std_days(p0);
    }
}        
#[cfg(test)]
mod tests_rug_154 {
    use super::*;
    use crate::ext::NumericalStdDuration;

    #[test]
    fn test_std_weeks() {
        let p0: f64 = 2.0;
        <f64>::std_weeks(p0);
    }
}
                            