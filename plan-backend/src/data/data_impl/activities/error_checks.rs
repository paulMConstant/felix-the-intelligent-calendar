//! Helper functions for activity implementation of data.

use crate::data::{Data, Time};

impl Data {
    /// Returns the first entity which does not have enough time to change the duration of the
    /// activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub(in super::super::activities) fn check_entity_without_enough_time_to_set_duration(
        &self,
        id: u16,
        new_duration: Time,
    ) -> Result<(), String> {
        let activity = self.activity(id)?;
        let current_duration = activity.duration();

        if new_duration <= current_duration {
            Ok(())
        } else {
            // Duration is longer - check if it conflicts with entity's schedule
            let required_free_time = new_duration - current_duration; // > 0
            if let Some(entity_name) = activity
                .entities_sorted()
                .iter()
                // Call to expect() : we are sure that all entities in the activity exist.
                .find(|entity_name| {
                    self.free_time_of(entity_name.clone())
                        .expect("Could not get entity participating in an activity")
                        < required_free_time
                })
                .cloned()
            {
                Err(format!(
                    "{} does not have enough time for the new duration.",
                    entity_name
                ))
            } else {
                Ok(())
            }
        }
    }

    /// Checks if the entity has enough time for the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist, the id is invalid or the entity
    /// does not have enough time.
    #[must_use]
    pub(in super::super::activities) fn check_has_enough_time_for_activity(
        &self,
        activity_id: u16,
        entity_name: &String,
    ) -> Result<(), String> {
        if self.has_enough_time_for_activity(activity_id, &entity_name)? {
            Ok(())
        } else {
            Err(format!(
                "{} does not have enough time left for this activity.",
                entity_name
            ))
        }
    }

    /// Checks if an entity in the list  does not have enough time for the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if any entity or the activity does not exist or if the entity does not
    /// have enough time for the activity.
    #[must_use]
    pub(in super::super::activities) fn check_entity_without_enough_time_for_activity(
        &self,
        activity_id: u16,
        entities: &Vec<String>,
    ) -> Result<(), String> {
        // TODO use try_find
        // Check that each entity has enough time
        for entity_name in entities {
            if self.has_enough_time_for_activity(activity_id, entity_name)? == false {
                return Err(format!(
                    "'{}' does not have enough time left for this activity.",
                    entity_name
                ));
            }
        }
        Ok(())
    }

    /// Returns true if the entity has enough time for the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity or activity does not exist.
    #[must_use]
    fn has_enough_time_for_activity(
        &self,
        activity_id: u16,
        entity_name: &String,
    ) -> Result<bool, String> {
        let free_time = self.free_time_of(entity_name)?;
        Ok(free_time >= self.activity(activity_id)?.duration())
    }
}
