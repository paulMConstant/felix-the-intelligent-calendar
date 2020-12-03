//! Contains helper functions which do not need to be in the public data API.

use crate::data::{Activity, Data, Time};

impl Data {
    /// Returns the time taken by the activities of an entity.
    ///
    /// If the entity does not exist, returns Time(0, 0).
    #[must_use]
    pub(crate) fn time_taken_by_activities(&self, entity_name: &String) -> Time {
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
    pub(crate) fn total_available_time(&self, entity_name: &String) -> Result<Time, String> {
        Ok(self
            .work_hours_of(entity_name)? // Here, check if entity exists
            .iter()
            .map(|interval| interval.duration())
            .sum())
    }

    /// Returns true if the entity has enough time for the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    #[must_use]
    pub(crate) fn has_enough_time_for_activity(
        &self,
        activity_id: u16,
        entity_name: &String,
    ) -> Result<bool, String> {
        let free_time = self.free_time_of(entity_name)?;
        Ok(free_time >= self.activity(activity_id)?.duration())
    }

    /// Returns true if the entity has enough time for the activities of the given group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity name is empty or if the entity is not found.
    #[must_use]
    pub(crate) fn has_enough_time_for_group(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::TimeInterval;

    #[test]
    fn test_time_taken_by_activities() {
        let mut data = Data::new();

        let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
        data.add_work_interval(interval)
            .expect("Could not add work interval");
        let name = data
            .add_entity("Name")
            .expect("Could not add entity")
            .name();
        let id1 = data
            .add_activity("Name")
            .expect("Could not add activity")
            .id();
        let id2 = data
            .add_activity("Name")
            .expect("Could not add activity")
            .id();

        data.add_entity_to_activity(id1, name.clone())
            .expect("Could not add entity");
        data.add_entity_to_activity(id2, name.clone())
            .expect("Could not add entity");

        let duration1 = data
            .activity(id1)
            .expect("Could not get activity by id")
            .duration();
        let duration2 = data
            .activity(id2)
            .expect("Could not get activity by id")
            .duration();
        assert_eq!(
            data.time_taken_by_activities(&name),
            duration1 + duration2,
            "Total duration was not computed correctly"
        );
    }

    #[test]
    fn test_total_available_time_global_work_hours() {
        let mut data = Data::new();

        let name = data
            .add_entity("Name")
            .expect("Could not add entity")
            .name();

        let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
        let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(14, 0));
        data.add_work_interval(interval1)
            .expect("Could not add work interval");
        data.add_work_interval(interval2)
            .expect("Could not add work interval");

        let expected = interval1.duration() + interval2.duration();
        let res = data
            .total_available_time(&name)
            .expect("Could not compute total available time");
        assert_eq!(
            expected, res,
            "Total available time was not computed correctly"
        );
    }

    #[test]
    fn test_total_available_time_custom_work_hours() {
        let mut data = Data::new();

        let name = data
            .add_entity("Name")
            .expect("Could not add entity")
            .name();

        let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
        let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(14, 0));
        data.add_custom_work_interval_for(name.clone(), interval1)
            .expect("Could not add custom work interval");
        data.add_custom_work_interval_for(name.clone(), interval2)
            .expect("Could not add custom work interval");

        let expected = interval1.duration() + interval2.duration();
        let res = data
            .total_available_time(&name)
            .expect("Could not compute total available time");
        assert_eq!(
            expected, res,
            "Total available time was not computed correctly"
        );
    }

    #[test]
    fn test_has_enough_time_for_activity() {
        let mut data = Data::new();

        let name = data
            .add_entity("Name")
            .expect("Could not add entity")
            .name();

        let id = data
            .add_activity("Activity")
            .expect("Could not add activity")
            .id();
        assert_eq!(
            data.has_enough_time_for_activity(id, &name),
            Ok(false),
            "An entity with no work interval never has enough time for an activity"
        );
        let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
        data.add_work_interval(interval)
            .expect("Could not add work interval");
        assert_eq!(
            data.has_enough_time_for_activity(id, &name),
            Ok(true),
            "The entity should have enough time for an activity with default duration"
        );
    }

    #[test]
    fn test_has_enough_time_for_group() {
        let mut data = Data::new();

        let entity_name = data
            .add_entity("Name")
            .expect("Could not add entity")
            .name();

        let group_name = data.add_group("Group").expect("Could not add group");

        assert_eq!(
            data.has_enough_time_for_group(&group_name, &entity_name),
            Ok(true),
            "An entity always has enough time for a group without activities"
        );

        let id = data
            .add_activity("Activity")
            .expect("Could not add activity")
            .id();

        data.add_group_to_activity(id, group_name.clone())
            .expect("Could not add group to activity");

        assert_eq!(
            data.has_enough_time_for_group(&group_name, &entity_name),
            Ok(false),
            "The entity could be added to group without having enough time for its activities"
        );
    }
}
