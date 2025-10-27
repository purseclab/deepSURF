// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! The time zone, which calculates offsets from the local time to UTC.
//!
//! There are four operations provided by the `TimeZone` trait:
//!
//! 1. Converting the local `NaiveDateTime` to `DateTime<Tz>`
//! 2. Converting the UTC `NaiveDateTime` to `DateTime<Tz>`
//! 3. Converting `DateTime<Tz>` to the local `NaiveDateTime`
//! 4. Constructing `DateTime<Tz>` objects from various offsets
//!
//! 1 is used for constructors. 2 is used for the `with_timezone` method of date and time types.
//! 3 is used for other methods, e.g. `year()` or `format()`, and provided by an associated type
//! which implements `Offset` (which then passed to `TimeZone` for actual implementations).
//! Technically speaking `TimeZone` has a total knowledge about given timescale,
//! but `Offset` is used as a cache to avoid the repeated conversion
//! and provides implementations for 1 and 3.
//! An `TimeZone` instance can be reconstructed from the corresponding `Offset` instance.

use core::fmt;

use crate::format::{parse, ParseResult, Parsed, StrftimeItems};
use crate::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::Weekday;
#[allow(deprecated)]
use crate::{Date, DateTime};

mod fixed;
pub use self::fixed::FixedOffset;

#[cfg(feature = "clock")]
mod local;
#[cfg(feature = "clock")]
pub use self::local::Local;

mod utc;
pub use self::utc::Utc;

/// The conversion result from the local time to the timezone-aware datetime types.
#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum LocalResult<T> {
    /// Given local time representation is invalid.
    /// This can occur when, for example, the positive timezone transition.
    None,
    /// Given local time representation has a single unique result.
    Single(T),
    /// Given local time representation has multiple results and thus ambiguous.
    /// This can occur when, for example, the negative timezone transition.
    Ambiguous(T /*min*/, T /*max*/),
}

impl<T> LocalResult<T> {
    /// Returns `Some` only when the conversion result is unique, or `None` otherwise.
    #[must_use]
    pub fn single(self) -> Option<T> {
        match self {
            LocalResult::Single(t) => Some(t),
            _ => None,
        }
    }

    /// Returns `Some` for the earliest possible conversion result, or `None` if none.
    #[must_use]
    pub fn earliest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(t, _) => Some(t),
            _ => None,
        }
    }

    /// Returns `Some` for the latest possible conversion result, or `None` if none.
    #[must_use]
    pub fn latest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(_, t) => Some(t),
            _ => None,
        }
    }

    /// Maps a `LocalResult<T>` into `LocalResult<U>` with given function.
    #[must_use]
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> LocalResult<U> {
        match self {
            LocalResult::None => LocalResult::None,
            LocalResult::Single(v) => LocalResult::Single(f(v)),
            LocalResult::Ambiguous(min, max) => LocalResult::Ambiguous(f(min), f(max)),
        }
    }
}

#[allow(deprecated)]
impl<Tz: TimeZone> LocalResult<Date<Tz>> {
    /// Makes a new `DateTime` from the current date and given `NaiveTime`.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_time(self, time: NaiveTime) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d.and_time(time).map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the current date, hour, minute and second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_opt(self, hour: u32, min: u32, sec: u32) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d.and_hms_opt(hour, min, sec).map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
    /// The millisecond part can exceed 1,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_milli_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => d
                .and_hms_milli_opt(hour, min, sec, milli)
                .map_or(LocalResult::None, LocalResult::Single),
            _ => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
    /// The microsecond part can exceed 1,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_micro_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => d
                .and_hms_micro_opt(hour, min, sec, micro)
                .map_or(LocalResult::None, LocalResult::Single),
            _ => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.
    /// The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_nano_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => d
                .and_hms_nano_opt(hour, min, sec, nano)
                .map_or(LocalResult::None, LocalResult::Single),
            _ => LocalResult::None,
        }
    }
}

impl<T: fmt::Debug> LocalResult<T> {
    /// Returns the single unique conversion result, or panics accordingly.
    #[must_use]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            LocalResult::None => panic!("No such local time"),
            LocalResult::Single(t) => t,
            LocalResult::Ambiguous(t1, t2) => {
                panic!("Ambiguous local time, ranging from {:?} to {:?}", t1, t2)
            }
        }
    }
}

/// The offset from the local time to UTC.
pub trait Offset: Sized + Clone + fmt::Debug {
    /// Returns the fixed offset from UTC to the local time stored.
    fn fix(&self) -> FixedOffset;
}

/// The time zone.
///
/// The methods here are the primarily constructors for [`Date`](../struct.Date.html) and
/// [`DateTime`](../struct.DateTime.html) types.
pub trait TimeZone: Sized + Clone {
    /// An associated offset type.
    /// This type is used to store the actual offset in date and time types.
    /// The original `TimeZone` value can be recovered via `TimeZone::from_offset`.
    type Offset: Offset;

    /// Make a new `DateTime` from year, month, day, time components and current time zone.
    ///
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// Returns `LocalResult::None` on invalid input data.
    fn with_ymd_and_hms(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> LocalResult<DateTime<Self>> {
        match NaiveDate::from_ymd_opt(year, month, day).and_then(|d| d.and_hms_opt(hour, min, sec))
        {
            Some(dt) => self.from_local_datetime(&dt),
            None => LocalResult::None,
        }
    }

    /// Makes a new `Date` from year, month, day and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date, invalid month and/or day.
    #[deprecated(since = "0.4.23", note = "use `with_ymd_and_hms()` instead")]
    #[allow(deprecated)]
    fn ymd(&self, year: i32, month: u32, day: u32) -> Date<Self> {
        self.ymd_opt(year, month, day).unwrap()
    }

    /// Makes a new `Date` from year, month, day and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date, invalid month and/or day.
    #[deprecated(since = "0.4.23", note = "use `with_ymd_and_hms()` instead")]
    #[allow(deprecated)]
    fn ymd_opt(&self, year: i32, month: u32, day: u32) -> LocalResult<Date<Self>> {
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }

    /// Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date and/or invalid DOY.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn yo(&self, year: i32, ordinal: u32) -> Date<Self> {
        self.yo_opt(year, ordinal).unwrap()
    }

    /// Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date and/or invalid DOY.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn yo_opt(&self, year: i32, ordinal: u32) -> LocalResult<Date<Self>> {
        match NaiveDate::from_yo_opt(year, ordinal) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }

    /// Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and
    /// the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    /// The resulting `Date` may have a different year from the input year.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date and/or invalid week number.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn isoywd(&self, year: i32, week: u32, weekday: Weekday) -> Date<Self> {
        self.isoywd_opt(year, week, weekday).unwrap()
    }

    /// Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and
    /// the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    /// The resulting `Date` may have a different year from the input year.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date and/or invalid week number.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn isoywd_opt(&self, year: i32, week: u32, weekday: Weekday) -> LocalResult<Date<Self>> {
        match NaiveDate::from_isoywd_opt(year, week, weekday) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the number of non-leap seconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// Panics on the out-of-range number of seconds and/or invalid nanosecond,
    /// for a non-panicking version see [`timestamp_opt`](#method.timestamp_opt).
    #[deprecated(since = "0.4.23", note = "use `timestamp_opt()` instead")]
    fn timestamp(&self, secs: i64, nsecs: u32) -> DateTime<Self> {
        self.timestamp_opt(secs, nsecs).unwrap()
    }

    /// Makes a new `DateTime` from the number of non-leap seconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// Returns `LocalResult::None` on out-of-range number of seconds and/or
    /// invalid nanosecond, otherwise always returns `LocalResult::Single`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp_opt(1431648000, 0).unwrap().to_string(), "2015-05-15 00:00:00 UTC");
    /// ```
    fn timestamp_opt(&self, secs: i64, nsecs: u32) -> LocalResult<DateTime<Self>> {
        match NaiveDateTime::from_timestamp_opt(secs, nsecs) {
            Some(dt) => LocalResult::Single(self.from_utc_datetime(&dt)),
            None => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the number of non-leap milliseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// Panics on out-of-range number of milliseconds for a non-panicking
    /// version see [`timestamp_millis_opt`](#method.timestamp_millis_opt).
    #[deprecated(since = "0.4.23", note = "use `timestamp_millis_opt()` instead")]
    fn timestamp_millis(&self, millis: i64) -> DateTime<Self> {
        self.timestamp_millis_opt(millis).unwrap()
    }

    /// Makes a new `DateTime` from the number of non-leap milliseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    ///
    /// Returns `LocalResult::None` on out-of-range number of milliseconds
    /// and/or invalid nanosecond, otherwise always returns
    /// `LocalResult::Single`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone, LocalResult};
    /// match Utc.timestamp_millis_opt(1431648000) {
    ///     LocalResult::Single(dt) => assert_eq!(dt.timestamp(), 1431648),
    ///     _ => panic!("Incorrect timestamp_millis"),
    /// };
    /// ```
    fn timestamp_millis_opt(&self, millis: i64) -> LocalResult<DateTime<Self>> {
        let (mut secs, mut millis) = (millis / 1000, millis % 1000);
        if millis < 0 {
            secs -= 1;
            millis += 1000;
        }
        self.timestamp_opt(secs, millis as u32 * 1_000_000)
    }

    /// Makes a new `DateTime` from the number of non-leap nanoseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// Unlike [`timestamp_millis`](#method.timestamp_millis), this never
    /// panics.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp_nanos(1431648000000000).timestamp(), 1431648);
    /// ```
    fn timestamp_nanos(&self, nanos: i64) -> DateTime<Self> {
        let (mut secs, mut nanos) = (nanos / 1_000_000_000, nanos % 1_000_000_000);
        if nanos < 0 {
            secs -= 1;
            nanos += 1_000_000_000;
        }
        self.timestamp_opt(secs, nanos as u32).unwrap()
    }

    /// Parses a string with the specified format string and returns a
    /// `DateTime` with the current offset.
    ///
    /// See the [`crate::format::strftime`] module on the
    /// supported escape sequences.
    ///
    /// If the to-be-parsed string includes an offset, it *must* match the
    /// offset of the TimeZone, otherwise an error will be returned.
    ///
    /// See also [`DateTime::parse_from_str`] which gives a [`DateTime`] with
    /// parsed [`FixedOffset`].
    fn datetime_from_str(&self, s: &str, fmt: &str) -> ParseResult<DateTime<Self>> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.to_datetime_with_timezone(self)
    }

    /// Reconstructs the time zone from the offset.
    fn from_offset(offset: &Self::Offset) -> Self;

    /// Creates the offset(s) for given local `NaiveDate` if possible.
    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Self::Offset>;

    /// Creates the offset(s) for given local `NaiveDateTime` if possible.
    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Self::Offset>;

    /// Converts the local `NaiveDate` to the timezone-aware `Date` if possible.
    #[allow(clippy::wrong_self_convention)]
    #[deprecated(since = "0.4.23", note = "use `from_local_datetime()` instead")]
    #[allow(deprecated)]
    fn from_local_date(&self, local: &NaiveDate) -> LocalResult<Date<Self>> {
        self.offset_from_local_date(local).map(|offset| {
            // since FixedOffset is within +/- 1 day, the date is never affected
            Date::from_utc(*local, offset)
        })
    }

    /// Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.
    #[allow(clippy::wrong_self_convention)]
    fn from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<DateTime<Self>> {
        self.offset_from_local_datetime(local)
            .map(|offset| DateTime::from_utc(*local - offset.fix(), offset))
    }

    /// Creates the offset for given UTC `NaiveDate`. This cannot fail.
    fn offset_from_utc_date(&self, utc: &NaiveDate) -> Self::Offset;

    /// Creates the offset for given UTC `NaiveDateTime`. This cannot fail.
    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Self::Offset;

    /// Converts the UTC `NaiveDate` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    #[allow(clippy::wrong_self_convention)]
    #[deprecated(since = "0.4.23", note = "use `from_utc_datetime()` instead")]
    #[allow(deprecated)]
    fn from_utc_date(&self, utc: &NaiveDate) -> Date<Self> {
        Date::from_utc(*utc, self.offset_from_utc_date(utc))
    }

    /// Converts the UTC `NaiveDateTime` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    #[allow(clippy::wrong_self_convention)]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Self> {
        DateTime::from_utc(*utc, self.offset_from_utc_datetime(utc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_millis() {
        let dt = Utc.timestamp_millis_opt(-1000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59 UTC");
        let dt = Utc.timestamp_millis_opt(-7000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:53 UTC");
        let dt = Utc.timestamp_millis_opt(-7001).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:52.999 UTC");
        let dt = Utc.timestamp_millis_opt(-7003).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:52.997 UTC");
        let dt = Utc.timestamp_millis_opt(-999).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.001 UTC");
        let dt = Utc.timestamp_millis_opt(-1).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.999 UTC");
        let dt = Utc.timestamp_millis_opt(-60000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:00 UTC");
        let dt = Utc.timestamp_millis_opt(-3600000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:00:00 UTC");

        for (millis, expected) in &[
            (-7000, "1969-12-31 23:59:53 UTC"),
            (-7001, "1969-12-31 23:59:52.999 UTC"),
            (-7003, "1969-12-31 23:59:52.997 UTC"),
        ] {
            match Utc.timestamp_millis_opt(*millis) {
                LocalResult::Single(dt) => {
                    assert_eq!(dt.to_string(), *expected);
                }
                e => panic!("Got {:?} instead of an okay answer", e),
            }
        }
    }

    #[test]
    fn test_negative_nanos() {
        let dt = Utc.timestamp_nanos(-1_000_000_000);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59 UTC");
        let dt = Utc.timestamp_nanos(-999_999_999);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.000000001 UTC");
        let dt = Utc.timestamp_nanos(-1);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.999999999 UTC");
        let dt = Utc.timestamp_nanos(-60_000_000_000);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:00 UTC");
        let dt = Utc.timestamp_nanos(-3_600_000_000_000);
        assert_eq!(dt.to_string(), "1969-12-31 23:00:00 UTC");
    }

    #[test]
    fn test_nanos_never_panics() {
        Utc.timestamp_nanos(i64::max_value());
        Utc.timestamp_nanos(i64::default());
        Utc.timestamp_nanos(i64::min_value());
    }
}
#[cfg(test)]
mod tests_rug_469 {
    use super::*;
    use crate::{TimeZone, offset::FixedOffset};

    #[test]
    fn test_with_ymd_and_hms() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let p1: i32 = 2022;
        let p2: u32 = 1;
        let p3: u32 = 1;
        let p4: u32 = 12;
        let p5: u32 = 30;
        let p6: u32 = 0;

        p0.with_ymd_and_hms(p1, p2, p3, p4, p5, p6);
    }
}
#[cfg(test)]
mod tests_rug_470 {
    use super::*;
    use crate::offset::FixedOffset;

    #[test]
    fn test_rug() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let mut p1: i32 = 2021;
        let mut p2: u32 = 2;
        let mut p3: u32 = 28;

        crate::offset::TimeZone::ymd(&mut p0, p1, p2, p3);
    }
}#[cfg(test)]
mod tests_rug_471 {
    use super::*;
    use crate::offset::FixedOffset;

    #[test]
    fn test_rug() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let mut p1: i32 = 2021;
        let mut p2: u32 = 12;
        let mut p3: u32 = 31;

        crate::offset::TimeZone::ymd_opt(&p0, p1, p2, p3);
    }
}#[cfg(test)]
mod tests_rug_472 {
    use super::*;
    use crate::offset::FixedOffset;

    #[test]
    fn test_rug() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let mut p1 = 2021i32;
        let mut p2 = 365u32;

        crate::offset::TimeZone::yo(&p0, p1, p2);
    }
}
#[cfg(test)]
mod tests_rug_473 {
    use super::*;
    use crate::{TimeZone, NaiveDate, LocalResult};

    #[test]
    #[allow(deprecated)]
    fn test_yo_opt() {
        let mut p0 = Local;
        let mut p1: i32 = 2021;
        let mut p2: u32 = 365;

        p0.yo_opt(p1, p2);

    }
}
#[cfg(test)]
mod tests_rug_474 {
    use super::*;
    use crate::Utc;
    use crate::Weekday;
    use std::str::FromStr;

    #[test]
    fn test_rug() {
        let mut p0: Utc = Utc;
        let mut p1: i32 = 2021; // Sample value, please modify accordingly
        let mut p2: u32 = 5; // Sample value, please modify accordingly
        let mut p3: Weekday = Weekday::from_str("Monday").unwrap();

        crate::offset::TimeZone::isoywd(&p0, p1, p2, p3);

    }
}

#[cfg(test)]
mod tests_rug_475 {
    use super::*;
    use crate::offset::utc::Utc;
    use crate::Weekday;
    use std::str::FromStr;
    
    #[test]
    fn test_rug() {
        let mut p0: Utc = Utc;
        let mut p1: i32 = 2021; // Sample year value
        let mut p2: u32 = 35; // Sample week value
        let mut p3: Weekday = Weekday::from_str("Monday").unwrap();
        
        crate::offset::TimeZone::isoywd_opt(&p0, p1, p2, p3);
        
    }
}
#[cfg(test)]
mod tests_rug_476 {
    use super::*;
    use crate::{offset::TimeZone, DateTime, offset::FixedOffset};
    
    #[test]
    fn test_rug() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let mut p1: i64 = 1630459200;
        let mut p2: u32 = 0;
        
        p0.timestamp(p1, p2);
    }
}
#[cfg(test)]
mod tests_rug_477 {
    use super::*;
    use crate::{Utc, TimeZone, offset::FixedOffset};

    #[test]
    fn test_timestamp_opt() {
        let p0 = FixedOffset::east(5 * 3600);
        let p1: i64 = 1431648000;
        let p2: u32 = 0;
        
        assert_eq!(p0.timestamp_opt(p1, p2).unwrap().to_string(), "2015-05-15 00:00:00 UTC");
    }
}#[cfg(test)]
mod tests_rug_478 {
    use super::*;
    use crate::{offset::TimeZone, DateTime, offset::FixedOffset};

    #[test]
    fn test_rug() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let mut p1: i64 = 1577836800000;

        p0.timestamp_millis(p1);

    }
}#[cfg(test)]
mod tests_rug_479 {
    use super::*;
    use crate::{DateTime, TimeZone, LocalResult, Utc};

    #[test]
    fn test_timestamp_millis_opt() {
        let p0: Utc = Utc;
        let p1: i64 = 1431648000;

        match p0.timestamp_millis_opt(p1) {
            LocalResult::Single(dt) => assert_eq!(dt.timestamp(), 1431648),
            _ => panic!("Incorrect timestamp_millis"),
        };
    }
}#[cfg(test)]
mod tests_rug_481 {
    use super::*;
    use crate::{DateTime, FixedOffset, TimeZone};

    #[test]
    fn test_datetime_from_str() {
        let p0 = FixedOffset::east(5 * 3600);
        let p1: &str = "2021-01-01 12:00:00";
        let p2: &str = "%Y-%m-%d %H:%M:%S";

        let result: ParseResult<DateTime<FixedOffset>> = p0.datetime_from_str(p1, p2);
    }
}#[cfg(test)]
mod tests_rug_482 {
    use super::*;
    use crate::{Utc, NaiveDate, Date, FixedOffset, offset};

    #[test]
    fn test_from_local_date() {
        let p0: offset::Utc = Utc;
        let p1: NaiveDate = NaiveDate::from_ymd(2021, 1, 1);

        p0.from_local_date(&p1);
    }
}                        
#[cfg(test)]
mod tests_rug_483 {
    use super::*;
    use crate::{DateTime, Local, TimeZone};
    use crate::naive::datetime::NaiveDateTime;
    
    #[test]
    fn test_from_local_datetime() {
        let local: DateTime<Local> = Local::now();
        let native: NaiveDateTime = local.naive_local();
        
        let tz = Local;
        
        tz.from_local_datetime(&native);
    }
}#[cfg(test)]
mod tests_rug_484 {
    use super::*;
    use crate::NaiveDate;
    use crate::offset::fixed::FixedOffset;

    #[test]
    fn test_rug() {
        let mut p0 = FixedOffset::east(5 * 3600);
        let mut p1 = NaiveDate::from_yo(2022, 1);

        p0.from_utc_date(&p1);
    }
}#[cfg(test)]
mod tests_rug_485 {
    use super::*;
    use crate::{offset::TimeZone, naive::datetime::NaiveDateTime, DateTime, FixedOffset};

    #[test]
    fn test_from_utc_datetime() {
        let p0: FixedOffset = FixedOffset::east(5 * 3600);
        let p1: NaiveDateTime = NaiveDateTime::from_timestamp(1626211200, 0);

        let result: DateTime<FixedOffset> = TimeZone::from_utc_datetime(&p0, &p1);
        
        // Add your assertions here
    }
}#[cfg(test)]
mod tests_rug_486 {
    use super::*;
    use crate::offset::LocalResult;

    #[test]
    fn test_single() {
        let p0: LocalResult<i32> = LocalResult::Single(42);

        p0.single();
    }
}#[cfg(test)]
mod tests_rug_487 {
    use super::*;
    use crate::offset::LocalResult;

    #[test]
    fn test_rug() {
        let p0: LocalResult<i32> = LocalResult::Single(42);

        LocalResult::<i32>::earliest(p0);
    }
}
#[cfg(test)]
mod tests_rug_488 {
    use super::*;
    use crate::offset::LocalResult;

    #[test]
    fn test_latest() {
        let mut p0: LocalResult<u32> = LocalResult::Single(42);

        LocalResult::<u32>::latest(p0);
    }
}
#[cfg(test)]
mod tests_rug_489 {
    use super::*;
    use crate::offset::LocalResult;

    #[test]
    fn test_rug() {
        let p0: LocalResult<i32> = LocalResult::Single(5);
        let mut p1 = |x: i32| x + 1;

        LocalResult::<i32>::map(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_490 {
    use super::*;
    use crate::offset::LocalResult;
    use crate::{Date, DateTime, Local, NaiveTime, TimeZone};

    #[test]
    fn test_and_time() {
        let mut p0: LocalResult<Date<Local>> = LocalResult::Single(Local::today());
        let mut p1 = NaiveTime::from_hms(12, 0, 0);

        LocalResult::<Date<Local>>::and_time(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_492 {
    use super::*;
    use crate::{offset, Local, Date, DateTime, TimeZone};

    #[test]
    fn test_and_hms_milli_opt() {
        let p0: offset::LocalResult<Date<Local>> = offset::LocalResult::None; // Replace with actual value
        let p1: u32 = 12; // Replace with actual value
        let p2: u32 = 30; // Replace with actual value
        let p3: u32 = 45; // Replace with actual value
        let p4: u32 = 500; // Replace with actual value

        p0.and_hms_milli_opt(p1, p2, p3, p4);
    }
}
#[cfg(test)]
mod tests_rug_493 {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_and_hms_micro_opt() {
        let p0 = LocalResult::Single(Utc.ymd(2021, 1, 1).with_timezone(&FixedOffset::east(0)));
        let p1: u32 = 12;
        let p2: u32 = 34;
        let p3: u32 = 56;
        let p4: u32 = 789;

        p0.and_hms_micro_opt(p1, p2, p3, p4);
    }
}
#[cfg(test)]
mod tests_rug_495 {
    use super::*;
    use crate::offset::LocalResult;

    #[test]
    fn test_rug() {
        let p0: LocalResult<u32> = LocalResult::Single(42);

        LocalResult::<u32>::unwrap(p0);
    }
}