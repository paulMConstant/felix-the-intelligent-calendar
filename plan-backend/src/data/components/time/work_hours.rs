use crate::data::TimeInterval;

/// Contains work hours represented as time intervals.
/// Stays sorted by ascending order and prevents work intervals from overlapping.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WorkHours {
    work_intervals: Vec<TimeInterval>,
}

impl WorkHours {
    /// Creates new work hours.
    #[must_use]
    pub fn new() -> WorkHours {
        WorkHours {
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
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
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
    #[must_use]
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        if let Some(index) = self
            .work_intervals
            .iter()
            .position(|&other| interval == other)
        {
            self.work_intervals.remove(index);
            Ok(())
        } else {
            Err("The given time interval was not found.".to_owned())
        }
    }

    /// Changes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found or if the new interval overlaps with
    /// the work hours.
    #[must_use]
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<(), String> {
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
            Err("The given time interval was not found.".to_owned())
        }
    }

    /// Checks if the given interval overlaps with others, one exception allowed.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps else Ok(()).
    #[must_use]
    fn check_if_interval_overlaps(
        &self,
        interval: TimeInterval,
        except: Option<TimeInterval>,
    ) -> Result<(), String> {
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
            Err("The given interval overlaps with other work intervals.".to_owned())
        } else {
            Ok(())
        }
    }
}
