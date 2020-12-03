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
        if self
            .work_intervals
            .iter()
            .any(|&other| interval.overlaps_with(&other))
        {
            Err("The given interval overlaps with other work intervals.".to_owned())
        } else {
            self.work_intervals.push(interval);
            self.work_intervals.sort();
            Ok(())
        }
    }

    /// Removes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found.
    #[must_use]
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        match self
            .work_intervals
            .iter()
            .position(|&other| interval == other)
        {
            Some(index) => {
                self.work_intervals.remove(index);
                Ok(())
            }
            None => Err("The given time interval was not found.".to_owned()),
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
        match self
            .work_intervals
            .iter()
            .position(|&other| old_interval == other)
        {
            Some(index) => {
                match self.work_intervals.iter().find(|&interval| {
                    interval != &old_interval && interval.overlaps_with(&new_interval)
                }) {
                    Some(_) => {
                        Err("The given interval overlaps with other work intervals.".to_owned())
                    }
                    None => {
                        self.work_intervals[index] = new_interval;
                        self.work_intervals.sort();
                        Ok(())
                    }
                }
            }
            None => Err("The given time interval was not found.".to_owned()),
        }
    }
}

// No tests, functions are tested in tests directory
