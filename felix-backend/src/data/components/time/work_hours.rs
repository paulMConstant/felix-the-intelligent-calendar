use crate::data::TimeInterval;
use crate::errors::Result;
use super::work_intervals::WorkIntervals;

use serde::{Deserialize, Serialize};

/// Contains work hours represented as time intervals.
/// Stays sorted by ascending order and prevents work intervals from overlapping.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorkHours {
    global_work_intervals: WorkIntervals,
}

impl WorkHours {
    /// Creates new work hours.
    #[must_use]
    pub fn new() -> WorkHours {
        WorkHours {
            global_work_intervals: WorkIntervals::new(),
        }
    }

    /// Returns immutable reference to the work hours.
    #[must_use]
    pub fn work_intervals(&self) -> &Vec<TimeInterval> {
        &self.global_work_intervals.work_intervals()
    }

    /// Adds the given time interval to the work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps with the current work intervals.
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.global_work_intervals.add_work_interval(interval)
    }

    /// Removes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found.
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.global_work_intervals.remove_work_interval(interval)
    }

    /// Changes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found or if the new interval overlaps with
    /// the work hours.
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<()> {
        self.global_work_intervals.update_work_interval(old_interval, new_interval)
    }
}
