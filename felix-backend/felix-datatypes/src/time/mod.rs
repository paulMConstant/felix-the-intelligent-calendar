pub const MIN_TIME_DISCRETIZATION_MINUTES: u16 = 5;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryInto;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const MIN_TIME_DISCRETIZATION: Time = Time {
    hours: 0,
    minutes: MIN_TIME_DISCRETIZATION_MINUTES as i8,
};

/// Minimal time structure with minute precision.
///
/// Any Time structure should be kept in [00:00, 24:00] and be a multiple of
/// MIN\_TIME\_DISCRETIZATION. Any operation (addition/substraction) which violates these
/// constraints will panic.
///
/// Made to be small and copyable, 2-bytes long :
/// ```
/// use felix_datatypes::Time;
/// use std::mem::size_of;
/// assert_eq!(size_of::<Time>(), 2);
/// ```
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Time {
    hours: i8,
    minutes: i8,
}

impl Time {
    /// Creates a new time object.
    ///
    /// # Panics
    ///
    /// Panics if the time is invalid, i.e. not in [00:00, 24:00]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    #[must_use]
    pub fn new(hours: i8, minutes: i8) -> Time {
        assert!(
            (hours < 24 || hours == 24 && minutes == 0)
                && hours >= 0
                && (0..60).contains(&minutes)
                && minutes % MIN_TIME_DISCRETIZATION.minutes() == 0,
            "Time must be kept in [00:00, 24:00] and be a multiple of MIN_TIME_DISCRETIZATION"
        );
        Time { hours, minutes }
    }

    /// Creates a new time object composed of the sum of n times the MIN\TIME\_DISCRETIZATION
    /// constant.
    ///
    /// # Panics
    ///
    /// Panics if the resulting time is invalid, i.e. not in [00:00, 24:00]
    #[must_use]
    pub fn from_n_times_min_discretization(n_times_min_discretization: i32) -> Time {
        let total_minutes = n_times_min_discretization * MIN_TIME_DISCRETIZATION_MINUTES as i32;
        Time::from_total_minutes(total_minutes.try_into().expect("Overflow error !"))
    }

    /// Creates a new time object from a total number of minutes.
    ///
    /// # Panics
    ///
    /// Panics if the resulting time is invalid, i.e. not in [00:00, 24:00]
    #[must_use]
    pub fn from_total_minutes(total_minutes: u16) -> Time {
        let hours = (total_minutes / 60) as i8;
        let minutes = (total_minutes % 60) as i8;
        Time::new(hours, minutes)
    }

    /// Simple getter for the hours.
    #[must_use]
    pub fn hours(&self) -> i8 {
        self.hours
    }

    /// Simple getter for the minutes.
    #[must_use]
    pub fn minutes(&self) -> i8 {
        self.minutes
    }

    /// Returns the total number of minutes in the given time.
    #[must_use]
    pub fn total_minutes(&self) -> u16 {
        self.minutes as u16 + self.hours as u16 * 60
    }

    /// Returns the number of times MIN\_TIME\_DISCRETIZATION fits in the Time.
    #[must_use]
    pub fn n_times_min_discretization(&self) -> i32 {
        self.minutes as i32 / MIN_TIME_DISCRETIZATION.minutes() as i32
            + self.hours as i32 * (60 / MIN_TIME_DISCRETIZATION.minutes() as i32)
    }

    /// Adds or substracts hours.
    /// Hours can be negative to substract.
    ///
    /// # Panics
    ///
    /// Panics if the result is invalid, i.e. not in [00:00, 24:00]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    pub fn add_hours(&mut self, hours: i8) {
        let new_hours = self.hours + hours;
        assert!(
            (0..24).contains(&new_hours),
            "The resulting time must be in [00:00, 24:00]"
        );
        self.hours = new_hours;
    }

    /// Adds or substracts minutes.
    /// Minutes can be negative for substraction but should not go over 60.
    ///
    /// # Panics
    ///
    /// Panics if minutes.abs() >= 60 or if the result is invalid, i.e. not in [00:00, 24:00]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    pub fn add_minutes(&mut self, minutes: i8) {
        assert!(
            minutes.abs() < 60,
            "If you wish to add more than 60 minutes, use add_hours_and_minutes"
        );
        let mut sum_minutes = self.minutes + minutes;
        if sum_minutes >= 60 {
            sum_minutes -= 60;
            self.add_hours(1);
        } else if sum_minutes < 0 {
            sum_minutes += 60;
            self.add_hours(-1);
        }
        self.minutes = sum_minutes;
    }

    /// Convenience method which adds or substracts hours and minutes.
    ///
    /// # Panics
    ///
    /// Panics if minutes.abs() >= 60 or if the result is invalid, i.e. [00:00, 24:00]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    pub fn add_hours_and_minutes(&mut self, hours: i8, minutes: i8) {
        self.add_minutes(minutes);
        self.add_hours(hours);
    }
}

impl Add for Time {
    type Output = Time;
    fn add(self, other: Time) -> Time {
        let sum_minutes = self.minutes + other.minutes;
        let sum_hours = self.hours + other.hours + sum_minutes / 60;
        Time::new(sum_hours, sum_minutes % 60)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Time) {
        let sum_minutes = self.minutes + other.minutes;
        let sum_hours = self.hours + other.hours + sum_minutes / 60;
        *self = Time::new(sum_hours, sum_minutes % 60);
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(self, other: Time) -> Time {
        let mut diff_minutes = self.minutes - other.minutes;
        let mut diff_hours = self.hours - other.hours;
        while diff_minutes < 0 {
            diff_minutes += 60;
            diff_hours -= 1;
        }
        assert!(
            diff_hours >= 0,
            "Cannot perform a - b where a < b : negative results are not allowed"
        );
        Time::new(diff_hours, diff_minutes)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Time) {
        let mut diff_minutes = self.minutes - other.minutes;
        let mut diff_hours = self.hours - other.hours;
        while diff_minutes < 0 {
            diff_minutes += 60;
            diff_hours -= 1;
        }
        assert!(
            diff_hours >= 0,
            "Cannot perform a - b where a < b : negative results are not allowed"
        );
        *self = Time::new(diff_hours, diff_minutes);
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hours.cmp(&other.hours) {
            Ordering::Equal => self.minutes.cmp(&other.minutes),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Sum<&'a Self> for Time {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(
            Self {
                hours: 0,
                minutes: 0,
            },
            |a, &b| a + b,
        )
    }
}

impl Sum<Self> for Time {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Self {
                hours: 0,
                minutes: 0,
            },
            |a, b| a + b,
        )
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
