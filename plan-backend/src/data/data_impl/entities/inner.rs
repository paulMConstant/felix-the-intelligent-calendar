//! Helper functions for entities implementation of data.

use crate::data::{Data, Time};

impl Data {
    /// Returns the time taken by the activities of an entity.
    ///
    /// If the entity does not exist, returns Time(0, 0).
    #[must_use]
    pub(in super::super::entities) fn time_taken_by_activities(
        &self,
        entity_name: &String,
    ) -> Time {
        self.activities
            .sorted_by_name()
            .iter()
            .filter_map(|activity| {
                if activity.entities_sorted().contains(&entity_name) {
                    Some(activity.duration())
                } else {
                    None
                }
            })
            .sum()
    }

    /// Returns the total time available for an entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    #[must_use]
    pub(in super::super::entities) fn total_available_time(
        &self,
        entity_name: &String,
    ) -> Result<Time, String> {
        Ok(self
            .work_hours_of(entity_name)? // Here, check if entity exists
            .iter()
            .map(|interval| interval.duration())
            .sum())
    }

    /// Checks if the given name is taken by a group.
    ///
    /// # Errors
    ///
    /// Returns Err if the group exists.
    #[must_use]
    pub(in super::super::entities) fn check_name_taken_by_group(
        &self,
        name: &String,
    ) -> Result<(), String> {
        if let Some(group_name) = self
            .groups_sorted()
            .iter()
            .map(|group| group.name())
            .find(|group_name| group_name == name)
        {
            Err(format!(
                "The name '{}' is already taken by a group.",
                group_name
            ))
        } else {
            Ok(())
        }
    }
}
