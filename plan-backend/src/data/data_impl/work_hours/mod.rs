use crate::data::{Data, TimeInterval};

/// Operations on work hours
impl Data {
    /// Returns an immutable reference to the work hours.
    #[must_use]
    pub fn work_hours(&self) -> Vec<TimeInterval> {
        self.work_hours.work_intervals().clone()
    }

    /// Adds the given time interval to the work hours.
    ///
    /// Work hours are always sorted.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps with the existing work intervals.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    /// assert!(data.add_work_interval(interval).is_ok());
    ///
    /// let overlapping_interval = TimeInterval::new(Time::new(7, 0), Time::new(9, 0));
    /// assert!(data.add_work_interval(overlapping_interval).is_err());
    /// ```
    #[must_use]
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        self.work_hours.add_work_interval(interval)
        // TODO update possible insertion times
    }

    /// Removes the given time interval from the work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the time interval is not found or if the time interval can't be removed
    /// because an entity no longer has any time left.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(interval).unwrap();
    ///
    /// assert!(data.remove_work_interval(interval).is_ok());
    /// assert!(data.work_hours().is_empty());
    ///
    /// let nonexistent_interval = interval;
    /// assert!(data.remove_work_interval(interval).is_err());
    /// ```
    #[must_use]
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        // Check if every entity has enough time left
        let duration = interval.duration();
        if let Some(entity_name) = self
            .entities_sorted()
            .iter()
            .map(|entity| entity.name())
            // Call to expect(): we are sure that the entity exists
            .find(|entity_name| {
                self.free_time_of(entity_name.clone())
                    .expect("Could not get entity listed in data.entities_sorted()")
                    < duration
            })
        {
            return Err(format!(
                "{} does not have enough time left. Free up their time before removing the work interval.",
                entity_name
            ));
        }

        self.work_hours.remove_work_interval(interval)
        // TODO update possible insertion times
    }

    /// Replaces the given time interval with a new one.
    ///
    /// # Errors
    ///
    /// Returns Err if the time interval is not found, if the time interval can't be updated
    /// because an entity does not have enough time left or if the new interval overlaps with others.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(interval).unwrap();
    ///
    /// let new_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// assert!(data.update_work_interval(interval, new_interval).is_ok());
    /// assert_eq!(data.work_hours()[0], new_interval);
    /// ```
    #[must_use]
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<(), String> {
        // If the interval is shorter, check that entities still have time left
        if new_interval.duration() < old_interval.duration() {
            let required_free_time = old_interval.duration() - new_interval.duration();
            if let Some(entity_name) = self
                .entities_sorted()
                .iter()
                .map(|entity| entity.name())
                // Call to expect() : we are sure that the entity exists
                .find(|entity_name| {
                    self.free_time_of(entity_name)
                        .expect("Could not get entity listed in data.entities_sorted()")
                        < required_free_time
                })
            {
                return Err(format!(
                    "{} does not have enough free time to reduce this interval.",
                    entity_name
                ));
            }
        }
        self.work_hours
            .update_work_interval(old_interval, new_interval)
        // TODO update possible insertion times
    }
}
