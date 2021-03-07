//! Helper functions for activity implementation of data.

use crate::data::{ActivityID, Data, Time};
use crate::errors::{
    add_entity_to_inserted_activity_spot_taken::AddEntityToInsertedActivitySpotTaken,
    not_enough_time::NotEnoughTime, Result,
};

impl Data {
    /// Returns the first entity which does not have enough time to change the duration of the
    /// activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub(super) fn check_entity_without_enough_time_to_set_duration(
        &self,
        id: ActivityID,
        new_duration: Time,
    ) -> Result<()> {
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
                Err(NotEnoughTime::activity_duration_too_long_for(
                    entity_name,
                    activity.name(),
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
    pub(super) fn check_has_enough_time_for_activity(
        &self,
        activity_id: ActivityID,
        entity_name: &String,
    ) -> Result<()> {
        if self.has_enough_time_for_activity(activity_id, &entity_name)? {
            Ok(())
        } else {
            let activity = self.activity(activity_id)?;
            Err(NotEnoughTime::activity_added_for(
                entity_name,
                activity.name(),
            ))
        }
    }

    /// Checks that no activity of the entity overlaps with the given activity's insertion slot.
    ///
    /// # Errors
    ///
    /// Returns Err if any activity of the entity is inserted in a slot which overlaps with the
    /// given activity's insertion slot.
    #[must_use]
    pub(super) fn check_no_activity_is_overlapping(
        &self,
        activity_id: ActivityID,
        entity_name: &String,
    ) -> Result<()> {
        let activity = self.activity(activity_id)?;
        let maybe_insertion_interval = activity.insertion_interval();
        let activity_name = activity.name();

        if let Some(insertion_interval) = maybe_insertion_interval {
            if let Some(blocking_activity) = self
                .activities_of(entity_name)?
                .into_iter()
                .filter(|other_activity| {
                    other_activity.id() != activity_id
                        && other_activity.insertion_interval().is_some()
                })
                .find(|other_activity| {
                    insertion_interval.overlaps_with(
                        &other_activity
                            .insertion_interval()
                            .expect("Filtering only inserted activities did not work"),
                    )
                })
            {
                Err(AddEntityToInsertedActivitySpotTaken::new(
                    entity_name.clone(),
                    activity_name,
                    blocking_activity.name(),
                ))
            } else {
                Ok(())
            }
        } else {
            // The activity is not inserted
            Ok(())
        }
    }

    /// Checks if an entity in the list  does not have enough time for the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if any entity or the activity does not exist or if the entity does not
    /// have enough time for the activity.
    #[must_use]
    pub(super) fn check_entity_without_enough_time_for_activity(
        &self,
        activity_id: ActivityID,
        entities: &Vec<String>,
    ) -> Result<()> {
        // Check that each entity has enough time
        for entity_name in entities {
            if self.has_enough_time_for_activity(activity_id, entity_name)? == false {
                let activity = self.activity(activity_id)?;
                return Err(NotEnoughTime::activity_added_for(
                    entity_name,
                    activity.name(),
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
        activity_id: ActivityID,
        entity_name: &String,
    ) -> Result<bool> {
        let free_time = self.free_time_of(entity_name)?;
        Ok(free_time >= self.activity(activity_id)?.duration())
    }
}
