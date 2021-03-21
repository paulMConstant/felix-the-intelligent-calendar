use crate::data::{Time, MIN_TIME_DISCRETIZATION};
use std::cmp::Ordering;

/// Time interval represented as {beginning, end}.
///
/// For simplicity reasons, this structure is not modifiable.
///
/// Made to be small and copyable, 4-bytes long :
/// ```
/// use felix_backend::data::TimeInterval;
/// use std::mem::size_of;
/// assert_eq!(size_of::<TimeInterval>(), 4);
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct TimeInterval {
    beginning: Time,
    end: Time,
}

impl TimeInterval {
    /// Creates a new TimeInterval.
    ///
    /// # Panics
    ///
    /// Panics if the interval is too short (or beginning is after the end),
    /// which translates to (end - beginning < MIN\_TIME\_DISCRETIZATION).
    ///
    /// # Example
    ///
    /// ```
    /// use felix_backend::data::{Time, TimeInterval};
    /// use std::panic::catch_unwind;
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
        assert!(
            end - beginning >= MIN_TIME_DISCRETIZATION,
            "Either beginning > end or the interval is too short"
        );
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
    /// use felix_backend::data::Time;
    /// use felix_backend::data::TimeInterval;
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
    /// use felix_backend::data::{Time, TimeInterval};
    ///
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

    /// Returns true if the time interval contains the given time.
    ///
    /// # Example
    ///
    /// ```
    /// use felix_backend::data::{Time, TimeInterval};
    ///
    /// let interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    ///
    /// assert_eq!(interval.contains(Time::new(7, 0)), false);
    /// assert!(interval.contains(Time::new(8, 0)));
    /// assert!(interval.contains(Time::new(8, 30)));
    /// assert_eq!(interval.contains(Time::new(9, 0)), false);
    /// ```
    #[must_use]
    pub fn contains(&self, time: Time) -> bool {
        self.beginning <= time && self.end > time
    }

    /// Returns true if the time interval contains the other time interval.
    ///
    /// # Example
    ///
    /// ```
    /// use felix_backend::data::{Time, TimeInterval};
    ///
    /// let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    /// let interval2 = TimeInterval::new(Time::new(8, 0), Time::new(8, 30));
    ///
    /// assert!(interval1.contains_interval(interval2));
    /// assert!(interval1.contains_interval(interval1));
    /// assert_eq!(interval2.contains_interval(interval1), false);
    /// ```
    #[must_use]
    pub fn contains_interval(&self, interval: TimeInterval) -> bool {
        self.beginning <= interval.beginning && self.end >= interval.end
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

impl std::fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.beginning, self.end)
    }
}

// This class is public. It is tested in integration tests, in 'tests' folder.
