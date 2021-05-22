//! Helper functions for work_hours implementation of data.

use crate::data::{Data, TimeInterval};
use crate::errors::{does_not_exist::DoesNotExist, not_enough_time::NotEnoughTime, Result};
use crate::Time;

impl Data {
    /// Checks if an entity has not enough time to update a work interval.
    ///
    /// # Errors
    ///
    /// Returns Err if an entity is found.
    pub(super) fn check_entity_without_enough_time_to_update_interval(
        &self,
        old_duration: Time,
        new_duration: Time,
    ) -> Result<()> {
        if new_duration >= old_duration {
            Ok(())
        } else {
            let required_free_time = old_duration - new_duration;
            if let Some(entity_name) = self.entity_with_free_time_less_than(required_free_time) {
                Err(NotEnoughTime::work_hours_shortened_for(entity_name))
            } else {
                Ok(())
            }
        }
    }

    /// Checks if an entity does not have enough time to remove a work interval.
    ///
    /// # Errors
    ///
    /// Returns Err if an entity is found.
    pub(super) fn check_entity_without_enough_time_to_remove_interval(
        &self,
        interval_duration: Time,
    ) -> Result<()> {
        if let Some(entity_name) = self.entity_with_free_time_less_than(interval_duration) {
            Err(NotEnoughTime::work_hours_shortened_for(entity_name))
        } else {
            Ok(())
        }
    }

    /// Given a required duration, returns the first entity which has less free time.
    #[must_use]
    fn entity_with_free_time_less_than(&self, required_free_time: Time) -> Option<String> {
        self.entities_sorted()
            .iter()
            .map(|entity| entity.name())
            // Call to expect(): we are sure that the entity exists
            .find(|entity_name| {
                self.free_time_of(entity_name.clone())
                    .expect("Could not get entity listed in data.entities_sorted()")
                    < required_free_time
            })
    }

    /// Checks if the entity will have enough time for their activities
    /// after adding a custom work hour.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if it will not have enough time.
    pub(super) fn check_entity_will_have_enough_time_with_custom_interval(
        &self,
        entity_name: &str,
        interval_duration: Time,
    ) -> Result<()> {
        if self
            .custom_work_hours_of(entity_name)? // Check if entity exists here
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
    pub(super) fn check_entity_has_custom_interval(
        &self,
        entity_name: &str,
        interval: &TimeInterval,
    ) -> Result<()> {
        // First, check if the entity has a corresponding custom work interval
        if self.custom_work_hours_of(entity_name)?.contains(interval) {
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
    pub(super) fn check_entity_will_have_enough_time_after_deletion_of_interval(
        &self,
        entity_name: &str,
        interval_duration: Time,
    ) -> Result<()> {
        // Check if the entity has enough free time
        let custom_work_hours = self.custom_work_hours_of(entity_name)?;
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
    pub(super) fn check_entity_will_have_enough_time_after_update(
        &self,
        entity_name: &str,
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
