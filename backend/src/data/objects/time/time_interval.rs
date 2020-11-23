use super::{Time, MIN_TIME_DISCRETIZATION};
use std::cmp::Ordering;

/// Time interval represented as {beginning, end}.
///
/// Made to be small and copyable, 4-bytes long :
/// ```
/// # use backend::data::TimeInterval;
/// # use std::mem::size_of;
/// assert_eq!(size_of::<TimeInterval>(), 4);
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct TimeInterval {
    beginning: Time,
    end: Time,
}

impl TimeInterval {
    /// Creates a new TimeInterval.
    ///
    /// # Panics
    ///
    /// Panics if end - beginning < MIN\_TIME\_DISCRETIZATION.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Time, TimeInterval};
    /// # use std::panic::catch_unwind;
    /// let (eight, nine) = (Time::new(8, 0), Time::new(9, 0));
    /// let time = TimeInterval::new(eight, nine);
    ///
    /// // Invalid time interval : beginning > end
    /// assert!(catch_unwind(|| { TimeInterval::new(nine, eight) }).is_err());
    /// // Invalid time interval : too short
    /// assert!(catch_unwind(|| { TimeInterval::new(eight, Time::new(8, 4)) }).is_err());
    /// ```
    #[must_use]
    pub fn new(beginning: Time, end: Time) -> TimeInterval {
        assert!(end - beginning >= MIN_TIME_DISCRETIZATION);
        TimeInterval { beginning, end }
    }

    /// Simple getter for the beginning.
    #[must_use]
    pub fn beginning(&self) -> Time {
        self.beginning
    }

    /// Simple getter for the end.
    #[must_use]
    pub fn end(&self) -> Time {
        self.end
    }

    /// Calculates and returns the duration.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Time;
    /// # use backend::data::TimeInterval;
    /// let time_interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 30));
    /// let expected_duration = Time::new(1, 30);
    /// assert_eq!(time_interval.duration(), expected_duration);
    /// ```
    #[must_use]
    pub fn duration(&self) -> Time {
        self.end - self.beginning
    }

    /// Returns true if the time intervals overlap with each other.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Time;
    /// # use backend::data::TimeInterval;
    /// let eight_nine = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    /// let eight_ten = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    /// let ten_eleven = TimeInterval::new(Time::new(10, 0), Time::new(11, 0));
    ///
    /// assert!(eight_nine.overlaps_with(&eight_ten));
    /// assert_eq!(eight_nine.overlaps_with(&ten_eleven), false);
    /// ```
    #[must_use]
    pub fn overlaps_with(&self, other: &TimeInterval) -> bool {
        self.beginning < other.end && self.end > other.beginning
    }
}

impl Ord for TimeInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.beginning.cmp(&other.beginning)
    }
}

impl PartialOrd for TimeInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn duration() {
        let interval = TimeInterval::new(Time::new(10, 0), Time::new(11, 0));
        assert_eq!(interval.duration(), Time::new(1, 0));
    }

    #[test]
    fn overlap() {
        let ten_to_eleven = TimeInterval::new(Time::new(10, 0), Time::new(11, 0));
        let tenthirty_to_eleventhirty = TimeInterval::new(Time::new(10, 30), Time::new(11, 30));
        assert!(ten_to_eleven.overlaps_with(&tenthirty_to_eleventhirty));
        let eleven_to_twelve = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
        assert_eq!(ten_to_eleven.overlaps_with(&eleven_to_twelve), false);
    }

    #[test]
    fn invalid_new() {
        // Beginning > end
        assert!(catch_unwind(|| { TimeInterval::new(Time::new(10, 0), Time::new(9, 0)) }).is_err());
        // Too short
        assert!(
            catch_unwind(|| { TimeInterval::new(Time::new(10, 0), Time::new(10, 0)) }).is_err()
        );
        // Not a multiple of MIN_TIME_DISCRETIZATION.minutes()
        assert!(
            catch_unwind(|| { TimeInterval::new(Time::new(10, 0), Time::new(10, 27)) }).is_err()
        );
    }
}
