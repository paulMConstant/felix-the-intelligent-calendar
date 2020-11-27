//! Contains helper functions which do not need to be in the public data API.
//! However the functions need to be public because the data implementation is split in many files.
//! The functions would fit really well as methods in impl Data.
//! Therefore data is passed as an argument to the function :
//! call C-style 'time_taken_by_activities(&self, &entity_name)' instead of
//! 'self.time_taken_by_activities(&entity_name)'

use crate::data::{Data, Time};

/// Returns the time taken by the activities of an entity.
///
/// If the entity does not exist, returns Time(0, 0).
///
/// This function should be considered like a method of impl Data.
/// It is not a method of Data because it should be public (accessible by all files
/// implementing Data) but not part of the public API.
pub fn time_taken_by_activities(this: &Data, entity_name: &String) -> Time {
    this.activities
        .sorted_by_name()
        .iter()
        .filter_map(|activity| {
            if activity.participants_sorted().contains(&entity_name) {
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
/// Returns err if the entity does not exist.
///
/// This function should be considered like a method of impl Data.
/// It is not a method of Data because it should be public (accessible by all files
/// implementing Data) but not part of the public API.
pub fn total_available_time(this: &Data, entity_name: &String) -> Result<Time, String> {
    Ok(this
        .work_hours_of(entity_name.clone())? // Here, check if entity exists
        .iter()
        .map(|interval| interval.duration())
        .sum())
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

        data.add_participant_to_activity(id1, name.clone())
            .expect("Could not add participant");
        data.add_participant_to_activity(id2, name.clone())
            .expect("Could not add participant");

        let duration1 = data
            .activity(id1)
            .expect("Could not get activity by id")
            .duration();
        let duration2 = data
            .activity(id2)
            .expect("Could not get activity by id")
            .duration();
        assert_eq!(
            time_taken_by_activities(&data, &name),
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
        let res =
            total_available_time(&data, &name).expect("Could not compute total available time");
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
        let res =
            total_available_time(&data, &name).expect("Could not compute total available time");
        assert_eq!(
            expected, res,
            "Total available time was not computed correctly"
        );
    }
}
