//! Helper functions for activity implementation of data.

use crate::errors::{
    add_entity_to_inserted_activity_invalid_spot::AddEntityToInsertedActivityInvalidSpot,
    not_enough_time::NotEnoughTime, Result,
};
use crate::Time;
use crate::{ActivityId, Data};

impl Data {
    /// Returns the first entity which does not have enough time to change the duration of the
    /// activity.
    ///
    /// # Errors
    ///
    /// Returns Err if an entity does not have enough time to change the duration of the entity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub(super) fn check_entity_without_enough_time_to_set_duration(
        &self,
        id: ActivityId,
        new_duration: Time,
    ) -> Result<()> {
        let activity = self.activity(id);
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
                    self.free_time_of(&(*entity_name).clone())
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
    /// Returns Err if the entity does not exist
    /// or the entity does not have enough time.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub(super) fn check_has_enough_time_for_activity(
        &self,
        activity_id: ActivityId,
        entity_name: &str,
    ) -> Result<()> {
        let free_time = self.free_time_of(entity_name)?;

        if free_time >= self.activity(activity_id).duration() {
            Ok(())
        } else {
            let activity = self.activity(activity_id);
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
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub(super) fn check_no_activity_of_the_entity_is_overlapping(
        &self,
        activity_id: ActivityId,
        entity_name: &str,
    ) -> Result<()> {
        let activity = self.activity(activity_id);
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
                Err(AddEntityToInsertedActivityInvalidSpot::blocking_activity(
                    entity_name,
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

    /// Checks that the activity, if inserted, fits into the entity's work hours.
    ///
    /// # Errors
    ///
    /// Returns Err:
    /// if the entity does not exist,
    /// if the activity is inserted and does not fit in the entity's work hours.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub(super) fn check_activity_inside_of_work_hours(
        &self,
        activity_id: ActivityId,
        entity_name: &str,
    ) -> Result<()> {
        let activity = self.activity(activity_id);
        let maybe_insertion_interval = activity.insertion_interval();
        let activity_name = activity.name();

        if let Some(insertion_interval) = maybe_insertion_interval {
            let work_hours = self.work_hours_of(entity_name)?;
            // Check if the activity can fit inside the work hours
            if work_hours
                .iter()
                .any(|time_interval| time_interval.contains_interval(insertion_interval))
            {
                // The activity is inserted inside the work hours of the entity
                Ok(())
            } else {
                // The activity is inserted at least partly outside of the work hours of the entity
                Err(
                    AddEntityToInsertedActivityInvalidSpot::outside_of_work_hours(
                        entity_name,
                        activity_name,
                    ),
                )
            }
        } else {
            // The activity is not inserted
            Ok(())
        }
    }
}
