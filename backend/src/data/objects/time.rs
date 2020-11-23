pub mod time_interval;
pub mod work_hours;

use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const MIN_TIME_DISCRETIZATION: Time = Time::unsafe_const_new(0, 5);

/// Minimal time structure with minute precision.
///
/// Made to be small and copyable, 2-bytes long :
/// ```
/// # use backend::data::Time;
/// # use std::mem::size_of;
/// assert_eq!(size_of::<Time>(), 2);
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Time {
    hours: i8,
    minutes: i8,
}

impl Time {
    /// Creates a new time object.
    ///
    /// # Panics
    ///
    /// Panics if the time is invalid, i.e. not in [00:00, 23:59]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Time;
    /// let time_earliest = Time::new(0, 0);
    /// let time_latest = Time::new(23, 55);
    /// assert!(std::panic::catch_unwind(|| { Time::new(24, 0) }).is_err());
    /// ```
    #[must_use]
    pub fn new(hours: i8, minutes: i8) -> Time {
        assert!(
            hours < 24
                && minutes < 60
                && hours >= 0
                && minutes >= 0
                && minutes % MIN_TIME_DISCRETIZATION.minutes() == 0
        );
        Time { hours, minutes }
    }

    /// Creates a new time object. Used for const initialization only.
    ///
    /// # Safety
    ///
    /// This can be used to initialize an invalid time !
    /// All Time objects must stay in [00:00, 23:59].
    #[must_use]
    const fn unsafe_const_new(hours: i8, minutes: i8) -> Time {
        Time { hours, minutes }
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

    /// Adds or substracts hours.
    /// Hours can be negative to substract.
    ///
    /// # Panics
    ///
    /// Panics if the result is invalid, i.e. not in [00:00, 23:59]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Time;
    /// let mut time = Time::new(1, 0);
    /// time.add_hours(2);
    /// assert_eq!(time, Time::new(3, 0));
    /// ```
    pub fn add_hours(&mut self, hours: i8) {
        let new_hours = self.hours + hours;
        assert!(new_hours >= 0 && new_hours < 24);
        self.hours = new_hours;
    }

    /// Adds or substracts minutes.
    /// Minutes can be negative for substraction but should not go over 60.
    ///
    /// # Panics
    ///
    /// Panics if minutes.abs() >= 60 or if the result is invalid, i.e. not in [00:00, 23:59]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Time;
    /// let mut time = Time::new(1, 50);
    /// time.add_minutes(20);
    /// assert_eq!(time, Time::new(2, 10));
    /// ```
    pub fn add_minutes(&mut self, minutes: i8) {
        assert!(minutes.abs() < 60);
        let mut sum_minutes = self.minutes + minutes;
        if sum_minutes > 60 {
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
    /// Panics if minutes.abs() >= 60 or if the result is invalid, i.e. [00:00, 23:59]
    /// and not a multiple of MIN\_TIME\_DISCRETIZATION.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Time;
    /// let mut time = Time::new(1, 50);
    /// time.add_hours_and_minutes(1, 20);
    /// assert_eq!(time, Time::new(3, 10));
    /// ```
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
        assert!(diff_hours >= 0);
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
        assert!(diff_hours >= 0);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn std_eq() {
        assert!(Time::new(2, 30) == Time::new(2, 30));
        assert!(Time::new(2, 30) != Time::new(1, 0));
    }

    #[test]
    fn std_ord() {
        // Compare with hours
        assert!(Time::new(2, 30) < Time::new(3, 0));
        // Compare with seconds
        assert!(Time::new(2, 30) < Time::new(2, 35));
    }

    #[test]
    fn std_op() {
        // Add without wrap
        assert_eq!(Time::new(1, 0) + Time::new(1, 30), Time::new(2, 30));
        // Add with wrap
        assert_eq!(Time::new(1, 40) + Time::new(1, 30), Time::new(3, 10));
        // Sub without wrap
        assert_eq!(Time::new(2, 40) - Time::new(1, 30), Time::new(1, 10));
        // Sub with wrap
        assert_eq!(Time::new(10, 0) - Time::new(9, 20), Time::new(0, 40));

        // AddAssign without wrap
        let mut time = Time::new(1, 0);
        time += Time::new(1, 30);
        assert_eq!(time, Time::new(2, 30));
        // AddAssign with wrap
        time = Time::new(1, 40);
        time += Time::new(1, 30);
        assert_eq!(time, Time::new(3, 10));
        // SubAssign without wrap
        time = Time::new(2, 40);
        time -= Time::new(1, 30);
        assert_eq!(time, Time::new(1, 10));
        // SubAssign with wrap
        time = Time::new(10, 0);
        time -= Time::new(9, 20);
        assert_eq!(time, Time::new(0, 40));

        // Invalid operations
        assert!(catch_unwind(|| { Time::new(2, 35) + Time::new(23, 0) }).is_err());
        assert!(catch_unwind(|| { Time::new(2, 35) - Time::new(3, 0) }).is_err());
    }

    #[test]
    fn std_sum() {
        let vec = vec![Time::new(0, 30); 3];
        assert_eq!(vec.iter().sum::<Time>(), Time::new(1, 30));
        assert_eq!(vec.iter().cloned().sum::<Time>(), Time::new(1, 30));
    }

    #[test]
    fn add_hours() {
        let mut time = Time::new(1, 0);
        time.add_hours(3);
        assert!(time == Time::new(4, 0));
        time.add_hours(-1);
        assert!(time == Time::new(3, 0));

        // Invalid operations
        assert!(catch_unwind(|| { Time::new(1, 0).add_hours(-2) }).is_err());
        assert!(catch_unwind(|| { Time::new(1, 0).add_hours(23) }).is_err());
    }

    #[test]
    fn add_minutes() {
        let mut time = Time::new(1, 0);
        time.add_minutes(55);
        assert_eq!(time, Time::new(1, 55));
        time.add_minutes(-25);
        assert_eq!(time, Time::new(1, 30));

        // Invalid operations
        assert!(catch_unwind(|| { Time::new(1, 0).add_minutes(61) }).is_err());
        assert!(catch_unwind(|| { Time::new(3, 0).add_minutes(-61) }).is_err());
    }

    #[test]
    fn invalid_new() {
        assert!(catch_unwind(|| { Time::new(25, 0) }).is_err());
        assert!(catch_unwind(|| { Time::new(0, 60) }).is_err());
        assert!(catch_unwind(|| { Time::new(-1, 0) }).is_err());
        assert!(catch_unwind(|| { Time::new(0, -1) }).is_err());
    }
}
