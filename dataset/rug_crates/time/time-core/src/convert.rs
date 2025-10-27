#![allow(clippy::missing_docs_in_private_items)] // TODO temporary

macro_rules! declare_structs {
    ($($t:ident)*) => {$(
        #[derive(Debug, Copy, Clone)]
        pub struct $t;

        impl $t {
            pub const fn per<T>(self, _: T) -> <(Self, T) as Per>::Output
            where
                (Self, T): Per,
                T: Copy,
            {
                <(Self, T)>::VALUE
            }
        }
    )*};
}

declare_structs! {
    Nanosecond
    Microsecond
    Millisecond
    Second
    Minute
    Hour
    Day
    Week
}

mod sealed {
    pub trait Sealed {}
}

pub trait Per: sealed::Sealed {
    type Output;

    const VALUE: Self::Output;
}

macro_rules! impl_per {
    ($($t:ty : $x:ident in $y:ident = $val:expr)*) => {$(
        impl sealed::Sealed for ($x, $y) {}

        impl Per for ($x, $y) {
            type Output = $t;

            const VALUE: $t = $val;
        }
    )*};
}

impl_per! {
    u16: Nanosecond in Microsecond = 1_000
    u32: Nanosecond in Millisecond = 1_000_000
    u32: Nanosecond in Second = 1_000_000_000
    u64: Nanosecond in Minute = 60_000_000_000
    u64: Nanosecond in Hour = 3_600_000_000_000
    u64: Nanosecond in Day = 86_400_000_000_000
    u64: Nanosecond in Week = 604_800_000_000_000

    u16: Microsecond in Millisecond = 1_000
    u32: Microsecond in Second = 1_000_000
    u32: Microsecond in Minute = 60_000_000
    u32: Microsecond in Hour = 3_600_000_000
    u64: Microsecond in Day = 86_400_000_000
    u64: Microsecond in Week = 604_800_000_000

    u16: Millisecond in Second = 1_000
    u16: Millisecond in Minute = 60_000
    u32: Millisecond in Hour = 3_600_000
    u32: Millisecond in Day = 86_400_000
    u32: Millisecond in Week = 604_800_000

    u8: Second in Minute = 60
    u16: Second in Hour = 3_600
    u32: Second in Day = 86_400
    u32: Second in Week = 604_800

    u8: Minute in Hour = 60
    u16: Minute in Day = 1_440
    u16: Minute in Week = 10_080

    u8: Hour in Day = 24
    u8: Hour in Week = 168

    u8: Day in Week = 7
}

#[cfg(test)]
mod tests_rug_493 {
    use super::*;
    use convert::{Nanosecond, Millisecond, Minute, Day, Microsecond, Second, Hour, Week};

    #[test]
    fn test_rug() {
        let mut p0 = Nanosecond;
        let mut p1 = Millisecond;
        
        p0.per(p1);
    }
}
#[cfg(test)]
mod tests_rug_494 {
    use super::*;
    use convert::{Microsecond, Day, Millisecond, Hour, Minute, Nanosecond, Second, Week};

    #[test]
    fn test_rug() {
        let p0 = Microsecond;
        let p1 = Day;

        <Microsecond>::per(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_495 {
    use super::*;
    use time_core::convert::{Millisecond,Nanosecond,Week,Microsecond,Day,Hour,Minute,Second};

    #[test]
    fn test_rug() {
        let mut p0: Millisecond = Millisecond;
        let mut p1: Nanosecond = Nanosecond;

        Millisecond::per::<Nanosecond>(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_496 {
    use super::*;
    use time_core::convert::{Second, Microsecond, Day, Nanosecond, Minute, Hour, Millisecond, Week};
    
    #[test]
    fn test_rug() {
        let mut p0 = Second;
        let mut p1 = Nanosecond;
        
        Second::per(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_497 {
    use super::*;
    use convert::{Minute, Day, Second, Microsecond, Millisecond, Hour, Week, Nanosecond};
    
    #[test]
    fn test_rug() {
        let p0: Minute = Minute;
        let p1: Day = Day;

        p0.per(p1);
    }
}

#[cfg(test)]
mod tests_rug_498 {
    use super::*;
    use convert::{Hour, Nanosecond};

    #[test]
    fn test_rug() {
        let p0: Hour = Hour {};
        let p1: Nanosecond = Nanosecond {};

        <Hour>::per(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_499 {
    use super::*;
    use convert::{
        Day, Minute, Week, Millisecond, Second, Nanosecond, Hour, Microsecond,
        Per,
    };

    #[test]
    fn test_rug() {
        let mut p0 = Day;
        let mut p1 = Minute;

        <Day>::per(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_500 {
    use super::*;
    use crate::convert::{
        Minute, Hour, Microsecond, Millisecond, Day, Second, Nanosecond
    };

    #[test]
    fn test_rug() {
        let mut p0 = convert::Week;
        let mut p1 = convert::Minute;
        
        <convert::Week>::per(p0, p1);
    }
}
