//! Helper functions for activity implementation of data.

use crate::data::{Activity, ActivityId, Data, Time, TimeInterval};
use crate::errors::Result;

use std::collections::HashSet;

impl Data {
    /// Returns all entities which participate in the given activity in more than one group.
    ///
    /// # Errors
    ///
    /// Returns Err if the given activity id is not valid or the group does not exist.
    pub(super) fn entities_participating_through_this_group_only(
        &self,
        activity_id: ActivityId,
        group_name: &str,
    ) -> Result<HashSet<String>> {
        let all_participating_groups = self.activity(activity_id)?.groups_sorted();
        let entities_of_group = self
            .group(group_name)?
            .entities_sorted()
            .into_iter()
            .collect::<HashSet<_>>();

        let entities_in_other_groups = all_participating_groups
            .iter()
            .filter(|&other_group_name| other_group_name != group_name)
            .flat_map(|group_name|
                // Expect is safe to use here: we are sure that the activtiy contains valid groups
                self.group(group_name).expect("Could not get group which is in an activity").entities_sorted()
                )
            .collect::<HashSet<String>>();

        Ok(entities_of_group
            .difference(&entities_in_other_groups)
            .cloned()
            .collect())
    }

    /// Returns the first activity which is incompatible with the activity with given id
    /// and whose insertion interval includes the given time, if it exists.
    #[must_use]
    pub(super) fn incompatible_activity_inserted_at_time(
        &self,
        activity: &Activity,
        time: Time,
    ) -> Option<Activity> {
        let hypothetical_insertion_iterval = TimeInterval::new(time, time + activity.duration());
        activity
            .incompatible_activity_ids()
            .iter()
            .filter_map(|&id| {
                let activity = self
                    .activity(id)
                    .expect("Found incompatible activity which does not exist");

                if let Some(interval) = activity.insertion_interval() {
                    if interval.overlaps_with(&hypothetical_insertion_iterval) {
                        Some(activity)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .next()
    }
}
