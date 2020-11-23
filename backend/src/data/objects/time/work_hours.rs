use super::time_interval::TimeInterval;

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
            == false
        {
            self.work_intervals.push(interval);
            self.work_intervals.sort();
            Ok(())
        } else {
            Err("The given interval overlaps with other work intervals.".to_owned())
        }
    }

    /// Removes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found.
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
            None => Err("Could not find the given time interval.".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Time;
    use super::*;

    #[test]
    fn add_work_interval() {
        let mut work_hours = WorkHours::new();

        // Add first interval
        let nine_eleven = TimeInterval::new(Time::new(9, 0), Time::new(11, 0));
        let res = work_hours.add_work_interval(nine_eleven);
        assert!(res.is_ok());
        assert_eq!(work_hours.work_intervals().len(), 1);
        assert_eq!(work_hours.work_intervals()[0], nine_eleven);

        // Try adding overlapping interval
        let nine_ten = TimeInterval::new(Time::new(9, 0), Time::new(10, 0));
        let res = work_hours.add_work_interval(nine_ten);
        assert!(res.is_err());
        assert_eq!(work_hours.work_intervals().len(), 1);

        // Add second valid interval
        let eight_nine = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
        let res = work_hours.add_work_interval(eight_nine);
        assert!(res.is_ok());
        assert_eq!(work_hours.work_intervals().len(), 2);

        // Make sure intervals were successfully sorted
        assert_eq!(work_hours.work_intervals()[0], eight_nine);
        assert_eq!(work_hours.work_intervals()[1], nine_eleven);
    }

    #[test]
    fn remove_work_interval() {
        let mut work_hours = WorkHours::new();
        let interval = TimeInterval::new(Time::new(10, 0), Time::new(11, 0));
        // Remove on empty vector
        let res = work_hours.remove_work_interval(interval);
        assert!(res.is_err());

        // Fill vector
        work_hours.add_work_interval(interval).unwrap();

        // Remove where interval is incorrect
        let wrong_interval = TimeInterval::new(Time::new(7, 0), Time::new(8, 0));
        let res = work_hours.remove_work_interval(wrong_interval);
        assert!(res.is_err());
        assert_eq!(work_hours.work_intervals().len(), 1);

        // Remove where interval is correct
        let res = work_hours.remove_work_interval(interval);
        assert!(res.is_ok());
        assert!(work_hours.work_intervals().is_empty());
    }
}
