//! Helper functions for groups implementation of data.

use crate::data::{Activity, Data, Time};

impl Data {
    /// Returns true if the entity has enough time for the activities of the given group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity name is empty or if the entity is not found.
    #[must_use]
    pub(in super::super::groups) fn has_enough_time_for_group(
        &self,
        group_name: &String,
        entity_name: &String,
    ) -> Result<bool, String> {
        let entity_should_be_added_to_activity = |activity: &Activity| {
            activity.groups_sorted().contains(group_name)
                && activity.entities_sorted().contains(entity_name) == false
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
        Ok(free_time >= duration_of_added_activities)
    }
}
