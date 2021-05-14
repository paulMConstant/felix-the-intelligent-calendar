mod common;
mod error_checks;

use crate::data::{Data, TimeInterval};
use crate::errors::Result;

/// Operations on work hours
impl Data {
    /// Returns a copy of the work hours.
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
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.check_no_activity_inserted()?;
        self.work_hours.add_work_interval(interval)?;
        self.notify_work_hours_changed();
        Ok(())
    }

    /// Removes the given time interval from the work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the time interval is not found or if the time interval can't be removed
    /// because an entity no longer has any time left.
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.check_no_activity_inserted()?;
        self.check_entity_without_enough_time_to_remove_interval(interval.duration())?;
        self.work_hours.remove_work_interval(interval)?;
        self.notify_work_hours_changed();
        Ok(())
    }

    /// Replaces the given time interval with a new one.
    ///
    /// # Errors
    ///
    /// Returns Err if the time interval is not found, if the time interval can't be updated
    /// because an entity does not have enough time left or if the new interval overlaps with others.
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<()> {
        self.check_no_activity_inserted()?;
        // If the interval is shorter, check that entities still have time left
        self.check_entity_without_enough_time_to_update_interval(
            old_interval.duration(),
            new_interval.duration(),
        )?;
        self.work_hours
            .update_work_interval(old_interval, new_interval)?;
        self.notify_work_hours_changed();
        Ok(())
    }
}
