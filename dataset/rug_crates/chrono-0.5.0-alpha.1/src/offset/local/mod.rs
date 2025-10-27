// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! The local (system) time zone.

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};

use super::fixed::FixedOffset;
use super::{LocalResult, TimeZone};
use crate::naive::{NaiveDate, NaiveDateTime};
#[allow(deprecated)]
use crate::{Date, DateTime};

// we don't want `stub.rs` when the target_os is not wasi or emscripten
// as we use js-sys to get the date instead
#[cfg(all(
    not(unix),
    not(windows),
    not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))
))]
#[path = "stub.rs"]
mod inner;

#[cfg(unix)]
#[path = "unix.rs"]
mod inner;

#[cfg(windows)]
#[path = "windows.rs"]
mod inner;

#[cfg(unix)]
mod tz_info;

/// The local timescale. This is implemented via the standard `time` crate.
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on the Local struct is the preferred way to construct `DateTime<Local>`
/// instances.
///
/// # Example
///
/// ```
/// use chrono::{Local, DateTime, TimeZone};
///
/// let dt: DateTime<Local> = Local::now();
/// let dt: DateTime<Local> = Local.timestamp(0, 0);
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Local;

impl Local {
    /// Returns a `Date` which corresponds to the current date.
    #[deprecated(since = "0.4.23", note = "use `Local::now()` instead")]
    #[allow(deprecated)]
    #[must_use]
    pub fn today() -> Date<Local> {
        Local::now().date()
    }

    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))]
    #[must_use]
    pub fn now() -> DateTime<Local> {
        inner::now()
    }

    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    #[must_use]
    pub fn now() -> DateTime<Local> {
        use super::Utc;
        let now: DateTime<Utc> = super::Utc::now();

        // Workaround missing timezone logic in `time` crate
        let offset =
            FixedOffset::west_opt((js_sys::Date::new_0().get_timezone_offset() as i32) * 60)
                .unwrap();
        DateTime::from_utc(now.naive_utc(), offset)
    }
}

impl TimeZone for Local {
    type Offset = FixedOffset;

    fn from_offset(_offset: &FixedOffset) -> Local {
        Local
    }

    // they are easier to define in terms of the finished date and time unlike other offsets
    #[allow(deprecated)]
    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<FixedOffset> {
        self.from_local_date(local).map(|date| *date.offset())
    }

    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<FixedOffset> {
        self.from_local_datetime(local).map(|datetime| *datetime.offset())
    }

    #[allow(deprecated)]
    fn offset_from_utc_date(&self, utc: &NaiveDate) -> FixedOffset {
        *self.from_utc_date(utc).offset()
    }

    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> FixedOffset {
        *self.from_utc_datetime(utc).offset()
    }

    // override them for avoiding redundant works
    #[allow(deprecated)]
    fn from_local_date(&self, local: &NaiveDate) -> LocalResult<Date<Local>> {
        // this sounds very strange, but required for keeping `TimeZone::ymd` sane.
        // in the other words, we use the offset at the local midnight
        // but keep the actual date unaltered (much like `FixedOffset`).
        let midnight = self.from_local_datetime(&local.and_hms_opt(0, 0, 0).unwrap());
        midnight.map(|datetime| Date::from_utc(*local, *datetime.offset()))
    }

    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    fn from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<DateTime<Local>> {
        let mut local = local.clone();
        // Get the offset from the js runtime
        let offset =
            FixedOffset::west_opt((js_sys::Date::new_0().get_timezone_offset() as i32) * 60)
                .unwrap();
        local -= crate::TimeDelta::seconds(offset.local_minus_utc() as i64);
        LocalResult::Single(DateTime::from_utc(local, offset))
    }

    #[cfg(not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))]
    fn from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<DateTime<Local>> {
        inner::naive_to_local(local, true)
    }

    #[allow(deprecated)]
    fn from_utc_date(&self, utc: &NaiveDate) -> Date<Local> {
        let midnight = self.from_utc_datetime(&utc.and_hms_opt(0, 0, 0).unwrap());
        Date::from_utc(*utc, *midnight.offset())
    }

    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Local> {
        // Get the offset from the js runtime
        let offset =
            FixedOffset::west_opt((js_sys::Date::new_0().get_timezone_offset() as i32) * 60)
                .unwrap();
        DateTime::from_utc(*utc, offset)
    }

    #[cfg(not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Local> {
        // this is OK to unwrap as getting local time from a UTC
        // timestamp is never ambiguous
        inner::naive_to_local(utc, false).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Local;
    use crate::offset::TimeZone;
    use crate::{Datelike, TimeDelta, Utc};

    #[test]
    fn verify_correct_offsets() {
        let now = Local::now();
        let from_local = Local.from_local_datetime(&now.naive_local()).unwrap();
        let from_utc = Local.from_utc_datetime(&now.naive_utc());

        assert_eq!(now.offset().local_minus_utc(), from_local.offset().local_minus_utc());
        assert_eq!(now.offset().local_minus_utc(), from_utc.offset().local_minus_utc());

        assert_eq!(now, from_local);
        assert_eq!(now, from_utc);
    }

    #[test]
    fn verify_correct_offsets_distant_past() {
        // let distant_past = Local::now() - Duration::days(365 * 100);
        let distant_past = Local::now() - TimeDelta::days(250 * 31);
        let from_local = Local.from_local_datetime(&distant_past.naive_local()).unwrap();
        let from_utc = Local.from_utc_datetime(&distant_past.naive_utc());

        assert_eq!(distant_past.offset().local_minus_utc(), from_local.offset().local_minus_utc());
        assert_eq!(distant_past.offset().local_minus_utc(), from_utc.offset().local_minus_utc());

        assert_eq!(distant_past, from_local);
        assert_eq!(distant_past, from_utc);
    }

    #[test]
    fn verify_correct_offsets_distant_future() {
        let distant_future = Local::now() + TimeDelta::days(250 * 31);
        let from_local = Local.from_local_datetime(&distant_future.naive_local()).unwrap();
        let from_utc = Local.from_utc_datetime(&distant_future.naive_utc());

        assert_eq!(
            distant_future.offset().local_minus_utc(),
            from_local.offset().local_minus_utc()
        );
        assert_eq!(distant_future.offset().local_minus_utc(), from_utc.offset().local_minus_utc());

        assert_eq!(distant_future, from_local);
        assert_eq!(distant_future, from_utc);
    }

    #[test]
    fn test_local_date_sanity_check() {
        // issue #27
        assert_eq!(Local.with_ymd_and_hms(2999, 12, 28, 0, 0, 0).unwrap().day(), 28);
    }

    #[test]
    fn test_leap_second() {
        // issue #123
        let today = Utc::now().date_naive();

        if let Some(dt) = today.and_hms_milli_opt(15, 2, 59, 1000) {
            let timestr = dt.time().to_string();
            // the OS API may or may not support the leap second,
            // but there are only two sensible options.
            assert!(
                timestr == "15:02:60" || timestr == "15:03:00",
                "unexpected timestr {:?}",
                timestr
            );
        }

        if let Some(dt) = today.and_hms_milli_opt(15, 2, 3, 1234) {
            let timestr = dt.time().to_string();
            assert!(
                timestr == "15:02:03.234" || timestr == "15:02:04.234",
                "unexpected timestr {:?}",
                timestr
            );
        }
    }
}
#[cfg(test)]
mod tests_rug_600 {
    use super::*;
    use crate::{Date, Local};

    #[test]
    fn test_rug() {
        Local::today();
    }
}#[cfg(test)]
mod tests_rug_601 {
    use super::*;
    use crate::{DateTime, Local};
    
    #[test]
    fn test_rug() {
        let _: DateTime<Local> = Local::now();
    }
}
#[cfg(test)]
mod tests_rug_603 {
    use crate::{NaiveDate, Local, DateTime, FixedOffset};
    use crate::offset::TimeZone;
    
    #[test]
    fn test_offset_from_local_date() {
        let mut p0: Local = Local {};
        let p1: NaiveDate = NaiveDate::from_ymd(2021, 1, 1);
        
        p0.offset_from_local_date(&p1);
    }
}#[cfg(test)]
mod tests_rug_604 {
    use super::*;
    use crate::TimeZone;
    use crate::{Local, DateTime, NaiveDateTime, FixedOffset};

    #[test]
    fn test_offset_from_local_datetime() {
        let local: DateTime<Local> = Local::now();
        let naive: NaiveDateTime = local.naive_local();

        let tz: Local = Local;
        let local_result: LocalResult<FixedOffset> = tz.offset_from_local_datetime(&naive);
    }
}
#[cfg(test)]
mod tests_rug_605 {
    use super::*;
    use crate::TimeZone;
    use crate::{Local, DateTime, FixedOffset, NaiveDate};
    
    #[test]
    fn test_offset_from_utc_date() {
        let local: DateTime<Local> = Local::now();
        let utc: DateTime<FixedOffset> = local.into();
        let naive_date: NaiveDate = utc.naive_local().date();

        let result = Local.offset_from_utc_date(&naive_date);

        // add assertions here
    }
}

#[cfg(test)]
mod tests_rug_608 {
    use super::*;
    use crate::TimeZone;
    use crate::{Local, DateTime, FixedOffset, NaiveDateTime};

    #[test]
    fn test_rug() {
        let mut p0: Local = Local;
        let mut p1: NaiveDateTime = NaiveDateTime::default();
                
        p0.from_local_datetime(&p1);
    }
}
#[cfg(test)]
mod tests_rug_609 {

    use super::*;
    use crate::{TimeZone, NaiveDate, Date, NaiveTime};

    #[test]
    fn test_rug() {
        let p0 = Local;
        let p1 = NaiveDate::from_ymd(2022, 10, 1);

        p0.from_utc_date(&p1);
    }
}
