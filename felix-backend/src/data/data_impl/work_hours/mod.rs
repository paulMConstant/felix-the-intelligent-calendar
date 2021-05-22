mod inner;
mod error_checks;

use crate::data::{Time, Data, TimeInterval, clean_string};
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

    // Entity-specific
    /// Returns the free time of an entity (total time in work hours - time taken by activities).
    ///
    /// The activities should never take more time than the total time ; should that happen,
    /// Time::new(0, 0) is returned.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    pub fn free_time_of<S>(&self, entity_name: S) -> Result<Time>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;

        // total_available_time checks if the entity exists
        let total_duration = self.total_available_time(&entity_name)?;
        let activity_duration = self.time_taken_by_activities(&entity_name);
        Ok(if total_duration < activity_duration {
            Time::new(0, 0)
        } else {
            total_duration - activity_duration
        })
    }

    /// Returns the custom work hours of the entity with the formatted given name.
    /// 
    /// If the entity does not have custom work hours, the resulting vector will be empty.
    /// 
    /// # Errors
    ///
    /// Returns Err if the entity with given name is not found.
    pub fn custom_work_hours_of<S>(&self, entity_name: S) -> Result<Vec<TimeInterval>> where S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;

        self.work_hours.custom_work_intervals_of(&entity_name)
    }

    /// Returns the work hours of the entity with the formatted given name.
    ///
    /// If the entity has custom work hours, returns them, else returns the global work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity with given name is not found.
    pub fn work_hours_of<S>(&self, entity_name: S) -> Result<Vec<TimeInterval>>
    where
        S: Into<String>,
    {
        let custom_work_hours = self.custom_work_hours_of(entity_name)?;
        Ok(if custom_work_hours.is_empty() {
            self.work_hours()
        } else {
            custom_work_hours
        })
    }

    /// Adds a custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval overlaps with others
    /// or if the entity does not have enough free time.
    pub fn add_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        // If this intervals overrides the global work hours,
        // check if the entity has enough free time
        let entity_name = clean_string(entity_name)?;
        self.check_no_activity_inserted()?;
        self.check_entity_will_have_enough_time_with_custom_interval(
            &entity_name,
            interval.duration(),
        )?;
        self.entities
            .add_custom_work_interval_for(&entity_name, interval)?;
        self.notify_work_hours_changed();

        Ok(())
    }

    /// Removes the given custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval is not found.
    pub fn remove_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;

        self.check_entity_has_custom_interval(&entity_name, &interval)?;
        self.check_no_activity_inserted()?;
        self.check_entity_will_have_enough_time_after_deletion_of_interval(
            &entity_name,
            interval.duration(),
        )?;
        self.entities
            .remove_custom_work_interval_for(&entity_name, interval)?;

        self.notify_work_hours_changed();
        Ok(())
    }

    /// Replaces the given time interval with the new one for the entity with the given formatted
    /// name
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found, if the interval is not found, if the
    /// time interval can't be updated because the entity does not have enough time left
    /// or if the updated interval overlaps with other intervals.
    pub fn update_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        self.check_no_activity_inserted()?;
        self.check_entity_has_custom_interval(&entity_name, &old_interval)?;
        self.check_entity_will_have_enough_time_after_update(
            &entity_name,
            old_interval.duration(),
            new_interval.duration(),
        )?;

        self.entities
            .update_custom_work_interval_for(&entity_name, old_interval, new_interval)?;

        self.notify_work_hours_changed();
        Ok(())
    }
}
