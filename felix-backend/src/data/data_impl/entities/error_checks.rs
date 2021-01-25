use crate::data::{Data, Time, TimeInterval};
use crate::errors::{
    does_not_exist::DoesNotExist, name_taken::NameTaken, not_enough_time::NotEnoughTime, Result,
};

impl Data {
    /// Checks if the given name is taken by a group.
    ///
    /// # Errors
    ///
    /// Returns Err if the group exists.
    #[must_use]
    pub(in super::super::entities) fn check_name_taken_by_group(
        &self,
        name: &String,
    ) -> Result<()> {
        if let Some(group_name) = self
            .groups_sorted()
            .iter()
            .map(|group| group.name())
            .find(|group_name| group_name == name)
        {
            Err(NameTaken::name_taken_by_group(group_name))
        } else {
            Ok(())
        }
    }

    /// Checks if the entity will have enough time for their activities
    /// after adding a custom work hour.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if it will not have enough time.
    #[must_use]
    pub(in super::super::entities) fn check_entity_will_have_enough_time_with_custom_interval(
        &self,
        entity_name: &String,
        interval_duration: Time,
    ) -> Result<()> {
        if self
            .entity(entity_name)? // Check if entity exists here
            .custom_work_hours()
            .is_empty()
        {
            let activity_duration = self.time_taken_by_activities(&entity_name);
            if interval_duration < activity_duration {
                return Err(NotEnoughTime::work_hours_shortened_for(entity_name));
            }
        }
        Ok(())
    }

    /// Checks if the entity has a custom work interval corresponding to the given one.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not have a custom work interval corresponding to the given
    /// one.
    #[must_use]
    pub(in super::super::entities) fn check_entity_has_custom_interval(
        &self,
        entity_name: &String,
        interval: &TimeInterval,
    ) -> Result<()> {
        // First, check if the entity has a corresponding custom work interval
        if self
            .entity(entity_name)?
            .custom_work_hours()
            .contains(interval)
        {
            Ok(())
        } else {
            Err(DoesNotExist::interval_does_not_exist(*interval))
        }
    }

    /// Checks that the entity will still have enough free time after deleting a custom work
    /// interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity name is empty, the entity is not found or the entity will not
    /// have enough free time.
    ///
    /// # Panics
    ///
    /// Panics if interval\_duration is greater than the custom work hours's total duration.
    #[must_use]
    pub(in super::super::entities) fn check_entity_will_have_enough_time_after_deletion_of_interval(
        &self,
        entity_name: &String,
        interval_duration: Time,
    ) -> Result<()> {
        // Check if the entity has enough free time
        let custom_work_hours = self.entity(entity_name)?.custom_work_hours();
        let entity_time = if custom_work_hours.len() == 1 {
            // This is the last custom work hours.
            // We should check that the global work hours will suffice.
            self.work_hours()
                .iter()
                .map(|interval| interval.duration())
                .sum()
        } else {
            // We should check that the remaining custom work hours will suffice.
            let time_before_deletion: Time = custom_work_hours
                .iter()
                .map(|interval| interval.duration())
                .sum();
            time_before_deletion - interval_duration
        };
        if entity_time < self.time_taken_by_activities(entity_name) {
            Err(NotEnoughTime::work_hours_shortened_for(entity_name))
        } else {
            Ok(())
        }
    }

    /// Checks if the entity will have enough time after time interval update.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity name is empty, the entity is not found
    /// or the entity will not have enough time after update.
    #[must_use]
    pub(in super::super::entities) fn check_entity_will_have_enough_time_after_update(
        &self,
        entity_name: &String,
        old_duration: Time,
        new_duration: Time,
    ) -> Result<()> {
        if new_duration >= old_duration {
            Ok(())
        } else {
            let required_free_time = old_duration - new_duration;
            if self.free_time_of(entity_name)? < required_free_time {
                Err(NotEnoughTime::work_hours_shortened_for(entity_name))
            } else {
                Ok(())
            }
        }
    }
}
