//! Helper functions for groups implementation of data.

use crate::errors::{name_taken::NameTaken, not_enough_time::NotEnoughTime, Result};
use crate::Time;
use crate::{Activity, Data};

impl Data {
    /// Checks that the given entity has enough time to be added to the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity name is empty, if the entity is not found
    /// or if the entity will not have enough time for the group's activities.
    pub(super) fn check_has_enough_time_for_group(
        &self,
        group_name: &str,
        entity_name: &str,
    ) -> Result<()> {
        let entity_should_be_added_to_activity = |activity: &Activity| {
            activity.groups_sorted().contains(&group_name.into())
                && !activity.entities_sorted().contains(&entity_name.into())
        };

        let duration_of_added_activities: Time = self
            .activities_sorted()
            .iter()
            .filter_map(|activity| {
                if entity_should_be_added_to_activity(activity) {
                    Some(activity.duration())
                } else {
                    None
                }
            })
            .sum();

        let free_time = self.free_time_of(entity_name)?;
        if free_time >= duration_of_added_activities {
            Ok(())
        } else {
            Err(NotEnoughTime::added_to_group(entity_name, group_name))
        }
    }

    /// Checks if the given name is taken by an entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the name is taken.
    pub(super) fn check_name_taken_by_entity(&self, name: &str) -> Result<()> {
        if let Some(entity_name) = self
            .entities_sorted()
            .iter()
            .map(|entity| entity.name())
            .find(|entity_name| entity_name == name)
        {
            Err(NameTaken::name_taken_by_entity(entity_name))
        } else {
            Ok(())
        }
    }
}
