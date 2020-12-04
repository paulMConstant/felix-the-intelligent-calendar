//! Helper functions for work_hours implementation of data.

use crate::data::{Data, Time};

impl Data {
    /// Checks if an entity has not enough time to update a work interval.
    ///
    /// # Errors
    ///
    /// Returns Err if an entity is found.
    #[must_use]
    pub(in super::super::work_hours) fn check_entity_without_enough_time_to_update_interval(
        &self,
        old_duration: Time,
        new_duration: Time,
    ) -> Result<(), String> {
        if new_duration >= old_duration {
            Ok(())
        } else {
            let required_free_time = old_duration - new_duration;
            if let Some(entity_name) = self.entity_with_free_time_less_than(required_free_time) {
                Err(format!(
                    "{} does not have enough free time to reduce this interval.",
                    entity_name
                ))
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
    #[must_use]
    pub(in super::super::work_hours) fn check_entity_without_enough_time_to_remove_interval(
        &self,
        interval_duration: Time,
    ) -> Result<(), String> {
        if let Some(entity_name) = self.entity_with_free_time_less_than(interval_duration) {
            Err(format!(
                "{} does not have enough time left. Free up their time before removing the work interval.",
                entity_name
            ))
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
}
