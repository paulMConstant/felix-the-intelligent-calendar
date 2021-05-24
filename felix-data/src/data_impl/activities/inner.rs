//! Helper functions for activity implementation of data.

use crate::errors::Result;
use crate::Time;
use crate::{Activity, ActivityId, Data, TimeInterval, WorkHoursAndActivityDurationsSorted};

use std::collections::HashSet;

impl Data {
    /// Returns all entities which participate in the given activity in more than one group.
    ///
    /// # Errors
    ///
    /// Returns Err if the group does not exist.
    pub(super) fn entities_participating_through_this_group_only(
        &self,
        activity_id: ActivityId,
        group_name: &str,
    ) -> Result<HashSet<String>> {
        let all_participating_groups = self.activity(activity_id).groups_sorted();
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
                let activity = self.activity(id);

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

    /// Given an entity, outputs their work hours and activity durations.
    ///
    /// # Panics
    ///
    /// Panics if one of the entity name is empty.
    pub(super) fn work_hours_and_activity_durations_from_entity(
        &self,
        entity: &str,
    ) -> WorkHoursAndActivityDurationsSorted {
        let work_hours = self.work_hours_of(entity).expect("Entity does not exist");
        let activity_durations = self
            .activities_of(entity)
            .expect("The entity name is empty")
            .iter()
            .map(|activity| activity.duration())
            .collect::<Vec<_>>();

        WorkHoursAndActivityDurationsSorted::new(work_hours, activity_durations)
    }
}
