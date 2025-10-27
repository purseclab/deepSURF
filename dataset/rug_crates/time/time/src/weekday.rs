//! Days of the week.

use core::fmt::{self, Display};
use core::str::FromStr;

use Weekday::*;

use crate::error;

/// Days of the week.
///
/// As order is dependent on context (Sunday could be either two days after or five days before
/// Friday), this type does not implement `PartialOrd` or `Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    #[allow(clippy::missing_docs_in_private_items)]
    Monday,
    #[allow(clippy::missing_docs_in_private_items)]
    Tuesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Wednesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Thursday,
    #[allow(clippy::missing_docs_in_private_items)]
    Friday,
    #[allow(clippy::missing_docs_in_private_items)]
    Saturday,
    #[allow(clippy::missing_docs_in_private_items)]
    Sunday,
}

impl Weekday {
    /// Get the previous weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Tuesday.previous(), Weekday::Monday);
    /// ```
    pub const fn previous(self) -> Self {
        match self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        }
    }

    /// Get the next weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
    /// ```
    pub const fn next(self) -> Self {
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }

    /// Get n-th next day.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.nth_next(1), Weekday::Tuesday);
    /// assert_eq!(Weekday::Sunday.nth_next(10), Weekday::Wednesday);
    /// ```
    pub const fn nth_next(self, n: u8) -> Self {
        match (self.number_days_from_monday() + n % 7) % 7 {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            val => {
                debug_assert!(val == 6);
                Sunday
            }
        }
    }

    /// Get the one-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_monday(), 1);
    /// ```
    #[doc(alias = "iso_weekday_number")]
    pub const fn number_from_monday(self) -> u8 {
        self.number_days_from_monday() + 1
    }

    /// Get the one-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_sunday(), 2);
    /// ```
    pub const fn number_from_sunday(self) -> u8 {
        self.number_days_from_sunday() + 1
    }

    /// Get the zero-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_monday(), 0);
    /// ```
    pub const fn number_days_from_monday(self) -> u8 {
        self as _
    }

    /// Get the zero-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_sunday(), 1);
    /// ```
    pub const fn number_days_from_sunday(self) -> u8 {
        match self {
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
            Sunday => 0,
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        })
    }
}

impl FromStr for Weekday {
    type Err = error::InvalidVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Monday),
            "Tuesday" => Ok(Tuesday),
            "Wednesday" => Ok(Wednesday),
            "Thursday" => Ok(Thursday),
            "Friday" => Ok(Friday),
            "Saturday" => Ok(Saturday),
            "Sunday" => Ok(Sunday),
            _ => Err(error::InvalidVariant),
        }
    }
}
#[cfg(test)]
mod tests_rug_482 {
    use super::*;
    use crate::Weekday;
    
    #[test]
    fn test_previous() {
        let mut p0: Weekday = Weekday::from_str("Tuesday").unwrap();

        p0.previous();
    }
}        
#[cfg(test)]
mod tests_rug_483 {
    use super::*;
    use crate::Weekday;
    
    #[test]
    fn test_rug() {
        let mut p0: Weekday = Weekday::Monday;

        Weekday::next(p0);

    }
}
                            #[cfg(test)]
mod tests_rug_484 {
    use super::*;
    use crate::Weekday;

    #[test]
    fn test_nth_next() {
        let p0 = Weekday::Monday; // Sample value
        let p1: u8 = 1; // Sample value

        assert_eq!(Weekday::nth_next(p0, p1), Weekday::Tuesday);
    }
}#[cfg(test)]
mod tests_rug_485 {
    use super::*;
    use crate::Weekday;
    
    #[test]
    fn test_rug() {
        let mut p0 = Weekday::from_str("Monday").unwrap();

        Weekday::number_from_monday(p0);

    }
}
#[cfg(test)]
mod tests_rug_486 {
    use super::*;
    use crate::Weekday;

    #[test]
    fn test_rug() {
        let mut p0: Weekday = Weekday::Monday;

        Weekday::number_from_sunday(p0);
    }
}

#[cfg(test)]
mod tests_rug_487 {
    use super::*;
    use crate::Weekday;
    
    #[test]
    fn test_number_days_from_monday() {
        let p0 = Weekday::Monday;
            
        assert_eq!(Weekday::number_days_from_monday(p0), 0);
    }
}#[cfg(test)]
mod tests_rug_488 {
    use super::*;
    use crate::Weekday;

    #[test]
    fn test_number_days_from_sunday() {
        let p0: Weekday = Weekday::Monday;

        assert_eq!(p0.number_days_from_sunday(), 1);
    }
}