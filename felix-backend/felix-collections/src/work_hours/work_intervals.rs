use felix_datatypes::TimeInterval;
use felix_errors::{does_not_exist::DoesNotExist, interval_overlaps::IntervalOverlaps, Result};

use serde::{Deserialize, Serialize};

/// Contains work hours represented as time intervals.
/// Stays sorted by ascending order and prevents work intervals from overlapping.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorkIntervals {
    work_intervals: Vec<TimeInterval>,
}

impl WorkIntervals {
    /// Creates new work hours.
    #[must_use]
    pub fn new() -> Self {
        Self {
            work_intervals: Vec::<TimeInterval>::new(),
        }
    }

    /// Returns immutable reference to the work hours.
    #[must_use]
    pub fn work_intervals(&self) -> &Vec<TimeInterval> {
        &self.work_intervals
    }

    /// Adds the given time interval to the work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps with the current work intervals.
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.check_if_interval_overlaps(interval, None)?;
        self.work_intervals.push(interval);
        self.work_intervals.sort();
        Ok(())
    }

    /// Removes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found.
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        if let Some(index) = self
            .work_intervals
            .iter()
            .position(|&other| interval == other)
        {
            self.work_intervals.remove(index);
            Ok(())
        } else {
            Err(DoesNotExist::interval_does_not_exist(interval))
        }
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
        if let Some(index) = self
            .work_intervals
            .iter()
            .position(|&other| old_interval == other)
        {
            self.check_if_interval_overlaps(new_interval, Some(old_interval))?;
            self.work_intervals[index] = new_interval;
            self.work_intervals.sort();
            Ok(())
        } else {
            Err(DoesNotExist::interval_does_not_exist(old_interval))
        }
    }

    /// Checks if the given interval overlaps with others, one exception allowed.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps else Ok(()).
    fn check_if_interval_overlaps(
        &self,
        interval: TimeInterval,
        except: Option<TimeInterval>,
    ) -> Result<()> {
        let equal_to_except = |&other_interval: &&TimeInterval| match except {
            Some(except_value) => *other_interval != except_value,
            None => true,
        };

        if self
            .work_intervals
            .iter()
            .filter(equal_to_except)
            .any(|&other_interval| interval.overlaps_with(&other_interval))
        {
            Err(IntervalOverlaps::new())
        } else {
            Ok(())
        }
    }
}
