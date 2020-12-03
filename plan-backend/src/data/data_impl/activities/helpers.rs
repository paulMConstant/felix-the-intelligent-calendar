//! Helper functions for activity implementation of data.

use crate::data::{Data, Time};
use std::collections::HashSet;
use std::iter::FromIterator;

impl Data {
    /// Returns the first entity which does not have enough time to change the duration of the
    /// activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub(in super::super::activities) fn entity_without_enough_time_to_set_duration(
        &self,
        id: u16,
        new_duration: Time,
    ) -> Result<Option<String>, String> {
        let activity = self.activity(id)?;
        let current_duration = activity.duration();

        Ok(if new_duration <= current_duration {
            None
        } else {
            // Duration is longer - check if it conflicts with entity's schedule
            let required_free_time = new_duration - current_duration; // > 0
            activity
                .entities_sorted()
                .iter()
                // Call to expect() : we are sure that all entities in the activity exist.
                .find(|entity_name| {
                    self.free_time_of(entity_name.clone())
                        .expect("Could not get entity participating in an activity")
                        < required_free_time
                })
                .cloned()
        })
    }

    /// Returns true if the entity has enough time for the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity or activity does not exist.
    #[must_use]
    pub(in super::super::activities) fn has_enough_time_for_activity(
        &self,
        activity_id: u16,
        entity_name: &String,
    ) -> Result<bool, String> {
        let free_time = self.free_time_of(entity_name)?;
        Ok(free_time >= self.activity(activity_id)?.duration())
    }

    /// Returns the first entity in the list which does not have enough time for the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if any entity or the activity does not exist.
    #[must_use]
    pub(in super::super::activities) fn entity_without_enough_time_for_activity(
        &self,
        activity_id: u16,
        entities: &Vec<String>,
    ) -> Result<Option<String>, String> {
        // TODO use try_find
        for entity_name in entities {
            if self.has_enough_time_for_activity(activity_id, entity_name)? == false {
                return Ok(Some(entity_name.clone()));
            }
        }
        Ok(None)
    }

    /// Returns all entities which participate in the given activity in more than one group.
    ///
    /// # Errors
    ///
    /// Returns Err if the given activity id is not valid or the group does not exist.
    #[must_use]
    pub(in super::super::activities) fn entities_participating_through_this_group_only(
        &self,
        activity_id: u16,
        group_name: &String,
    ) -> Result<HashSet<String>, String> {
        let all_participating_groups = self.activity(activity_id)?.groups_sorted();
        let entities_of_group =
            HashSet::from_iter(self.group(group_name)?.entities_sorted().into_iter());

        let entities_in_other_groups = all_participating_groups
            .iter()
            .filter(|&other_group_name| other_group_name != group_name)
            .flat_map(|group_name|
                // Expect is safe to use here: we are sure that the activtiy contains valid groups
                self.group(group_name).expect("Could not get group which is in an activity").entities_sorted()
                )
            .collect::<HashSet<&String>>();

        Ok(entities_of_group
            .difference(&entities_in_other_groups)
            .map(|&entity| entity.clone())
            .collect())
    }
}
