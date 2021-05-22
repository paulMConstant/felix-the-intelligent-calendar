use crate::{Time, MIN_TIME_DISCRETIZATION};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Time interval represented as {beginning, end}.
///
/// For simplicity reasons, this structure is not modifiable.
///
/// Made to be small and copyable, 4-bytes long :
/// ```
/// use felix_datatypes::TimeInterval;
/// use std::mem::size_of;
/// assert_eq!(size_of::<TimeInterval>(), 4);
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
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
    #[must_use]
    pub fn duration(&self) -> Time {
        self.end - self.beginning
    }

    /// Returns true if the time intervals overlap with each other.
    #[must_use]
    pub fn overlaps_with(&self, other: &TimeInterval) -> bool {
        self.beginning < other.end && self.end > other.beginning
    }

    /// Returns true if the time interval contains the given time.
    #[must_use]
    pub fn contains(&self, time: Time) -> bool {
        self.beginning <= time && self.end > time
    }

    /// Returns true if the time interval contains the other time interval.
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
