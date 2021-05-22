use felix_backend::{Time, TimeInterval};
use felix_test_utils::{Activity, DataBuilder};

#[test]
fn simple_add_custom_work_interval() {
    let entity = "Entity";
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_ok!(data, DataBuilder::new().with_entity(entity), {
        data.add_custom_work_interval_for(entity, interval)
            .expect("Could not add custom work interval");

        let custom_work_hours = data
            .custom_work_hours_of(entity)
            .expect("Could not get entity by name");

        assert_eq!(
            custom_work_hours.len(),
            1,
            "Custom work interval was not added"
        );
        assert_eq!(
            custom_work_hours[0], interval,
            "Custom work interval was not added right"
        );
    });
}

#[test]
fn add_custom_work_interval_nonexistent_entity() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new(),
        data.add_custom_work_interval_for("Name", interval),
        "Name does not exist.",
        "Could add custom work interval for nonexistent entity"
    );
}

#[test]
fn add_overlapping_custom_work_interval() {
    let entity = "Entity";
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let overlap = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval),
        data.add_custom_work_interval_for(entity, overlap),
        "The given interval overlaps with others.",
        "Could add overlapping interval"
    );
}

#[test]
fn add_custom_work_interval_not_enough_free_time() {
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_work_interval_of_duration(4)
            .with_activity(Activity {
                duration: Time::new(4, 0),
                entities: vec![entity],
                ..Default::default()
            }),
        {
            let custom_interval_too_short = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
            assert!(data.activities_sorted()[0].entities_sorted()[0] == entity);
            data.add_custom_work_interval_for(entity, custom_interval_too_short)
        },
        "Entity will not have enough time if their work hours are shortened.",
        "Could add custom work interval which led to entity not having enough time"
    );
}

/// It should be possible to add adjacent work intervals (end == beginning of other)
#[test]
fn add_custom_adjacent_work_interval() {
    let entity = "Entity";
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let adjacent = TimeInterval::new(Time::new(12, 0), Time::new(14, 0));
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval),
        {
            data.add_custom_work_interval_for(entity, adjacent)
                .expect("Could not add custom adjacent work interval");
        }
    );
}

#[test]
fn check_custom_work_intervals_sorted() {
    let entity = "Entity";
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(17, 0));
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_intervals_for(entity, vec![interval1, interval2, interval3]),
        {
            let custom_work_hours = data
                .custom_work_hours_of(entity)
                .expect("Could not get entity by name");
            assert_eq!(custom_work_hours[0], interval1, "Intervals are not sorted");
            assert_eq!(custom_work_hours[1], interval2, "Intervals are not sorted");
            assert_eq!(custom_work_hours[2], interval3, "Intervals are not sorted");
        }
    );
}

#[test]
fn simple_remove_custom_work_interval() {
    let entity = "Entity";
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_intervals_for(entity, vec![interval1, interval2]),
        {
            data.remove_custom_work_interval_for(entity, interval1)
                .expect("Could not remove custom work interval");
            let custom_work_hours = data
                .custom_work_hours_of(entity)
                .expect("Could not get entity by name");
            assert_eq!(custom_work_hours.len(), 1, "Custom work interval was not removed");
            assert_eq!(
                custom_work_hours[0], interval2,
                "The wrong time interval was removed"
            );
        }
    );
}

#[test]
fn remove_custom_work_interval_nonexistent_entity() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_custom_work_interval_for("Name", interval),
        "Name does not exist.",
        "Could add custom work interval for nonexistent entity"
    );
}

#[test]
fn remove_custom_work_interval_not_enough_free_time() {
    let entity = "Entity";
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval)
            .with_activity(Activity {
                entities: vec![entity],
                duration: Time::new(3, 0),
                ..Default::default()
            }),
        data.remove_custom_work_interval_for(entity, interval),
        "Entity will not have enough time if their work hours are shortened.",
        "Could remove custom work interval which led to entity having not enough free time"
    );
}

#[test]
fn remove_custom_work_interval_enough_time_with_global_work_hours() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let custom_interval = interval;
    let entity = "Entity";
    test_ok!(data, DataBuilder::new().with_entity(entity)
             .with_work_interval(interval)
             .with_custom_work_interval_for(entity, custom_interval)
             .with_activity( Activity { entities: vec![entity], duration: Time::new(3, 0),  ..Default::default() }),
    data
    .remove_custom_work_interval_for(entity, custom_interval)
    .expect("Could not remove time interval even though the entity will have enough time with the global work hours"));
}

#[test]
fn remove_nonexistent_custom_work_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.remove_custom_work_interval_for(entity, interval),
        "The interval '08:00 - 12:00' does not exist.",
        "Could remove time interval even though there are none"
    );
}

#[test]
fn remove_custom_work_interval_beginning_different() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval),
        {
            let same_end = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
            data.remove_custom_work_interval_for(entity, same_end)
        },
        "The interval '09:00 - 12:00' does not exist.",
        "Could remove time interval with different beginning"
    );
}

#[test]
fn remove_custom_work_interval_end_different() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval),
        {
            let same_beginning = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
            data.remove_custom_work_interval_for(entity, same_beginning)
        },
        "The interval '08:00 - 11:00' does not exist.",
        "Could remove time interval with different end"
    );
}

#[test]
fn simple_update_custom_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval),
        {
            let new_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
            data.update_custom_work_interval_for(entity, interval, new_interval)
                .expect("Could not update time interval");

            let interval = data
                .work_hours_of(entity)
                .expect("Could not find work hours of entity")[0];
            assert_eq!(interval, new_interval, "Interval was not updated");
        }
    );
}

#[test]
fn update_nonexistent_custom_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.update_custom_work_interval_for(entity, interval, interval),
        "The interval '08:00 - 12:00' does not exist.",
        "Could update nonexistent work interval"
    );
}

#[test]
fn update_custom_interval_nonexistent_entity() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new(),
        data.update_custom_work_interval_for("Does not exist", interval, interval),
        "Does Not Exist does not exist.",
        "Could update custom work interval for nonexistent entity"
    );
}

#[test]
fn update_custom_time_interval_not_enough_time_for_activities() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval)
            .with_activity(Activity {
                entities: vec![entity],
                duration: Time::new(4, 0),
                ..Default::default()
            }),
        {
            let new_interval_too_short = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
            data.update_custom_work_interval_for(entity, interval, new_interval_too_short)
        },
        "Entity will not have enough time if their work hours are shortened.",
        "Could update interval which left entity with not enough time"
    );
}

#[test]
fn update_custom_time_interval_overlaps() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(15, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_intervals_for(entity, vec![interval1, interval2]),
        {
            let new_interval_overlaps = TimeInterval::new(Time::new(10, 0), Time::new(15, 0));
            data.update_custom_work_interval_for(entity, interval1, new_interval_overlaps)
        },
        "The given interval overlaps with others.",
        "Could add overlapping work interval"
    );
}

#[test]
fn update_custom_time_interval_check_sorted() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let entity = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_intervals_for(entity, vec![interval1, interval2]),
        {
            let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(15, 0));

            data.update_custom_work_interval_for(entity, interval1, interval3)
                .expect("Could not add custom interval");

            let intervals = data
                .work_hours_of(entity)
                .expect("Could not get work hours of entity");
            assert_eq!(intervals[0], interval2, "Work hours are not sorted");
            assert_eq!(intervals[1], interval3, "Work hours are not sorted");
        }
    );
}

// *** Get work hours of entity ***
#[test]
fn work_hours_of_without_custom_work_hours() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_work_interval(interval),
        {
            let entity_work_hours = data
                .work_hours_of(entity)
                .expect("Could not fetch work hours of entity");
            let general_work_hours = data.work_hours();

            assert_eq!(
                entity_work_hours, general_work_hours,
                "Work hours of does not return the general work hours where it should"
            );
        }
    );
}

#[test]
fn work_hours_of_with_custom_work_hours() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_interval_for(entity, interval),
        {
            let entity_work_hours = data
                .work_hours_of(entity)
                .expect("Could not fetch work hours of entity");
            let custom_work_hours = data
                .custom_work_hours_of(entity)
                .expect("Could not get entity by name");

            assert_eq!(
                entity_work_hours, custom_work_hours,
                "Work hours of does not return the custom work hours where it should"
            );
        }
    );
}

// *** Free time ***
#[test]
fn free_time_without_activities_with_global_work_hours() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    let entity = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_work_intervals(vec![interval1, interval2]),
        {
            let free_time = data
                .free_time_of(entity)
                .expect("Could not fetch free time of entity");
            let expected_free_time = interval1.duration() + interval2.duration();

            assert_eq!(
                free_time, expected_free_time,
                "Free time was not calculated right"
            );
        }
    );
}

#[test]
fn free_time_without_activities_with_custom_work_hours() {
    let entity = "Entity";
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_intervals_for(entity, vec![interval1, interval2]),
        {
            let free_time = data
                .free_time_of(entity)
                .expect("Could not fetch free time of entity");
            let expected_free_time = interval1.duration() + interval2.duration();

            assert_eq!(
                free_time, expected_free_time,
                "Free time was not calculated right"
            );
        }
    );
}

#[test]
fn free_time_with_activities_with_global_work_hours() {
    let entity = "Entity";
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_work_intervals(vec![interval1, interval2])
            .with_activities(vec![
                Activity {
                    duration: Time::new(1, 0),
                    entities: vec![entity],
                    ..Default::default()
                },
                Activity {
                    duration: Time::new(1, 30),
                    entities: vec![entity],
                    ..Default::default()
                }
            ]),
        {
            let free_time = data
                .free_time_of(entity)
                .expect("Could not fetch free time of entity");
            let activities = data.activities_sorted();
            let expected_free_time = interval1.duration() + interval2.duration()
                - activities[0].duration()
                - activities[1].duration();
            assert_eq!(
                free_time, expected_free_time,
                "Free time was not calculated right"
            );
        }
    );
}

#[test]
fn free_time_with_activities_with_custom_work_hours() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    let entity = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_custom_work_intervals_for(entity, vec![interval1, interval2])
            .with_activities(vec![
                Activity {
                    duration: Time::new(1, 0),
                    entities: vec![entity],
                    ..Default::default()
                },
                Activity {
                    duration: Time::new(1, 30),
                    entities: vec![entity],
                    ..Default::default()
                }
            ]),
        {
            let free_time = data
                .free_time_of(entity)
                .expect("Could not fetch free time of entity");
            let activities = data.activities_sorted();
            let expected_free_time = interval1.duration() + interval2.duration()
                - activities[0].duration()
                - activities[1].duration();

            assert_eq!(
                free_time, expected_free_time,
                "Free time was not calculated right"
            );
        }
    );
}

#[test]
fn free_time_of_wrong_entity() {
    test_err!(
        data,
        DataBuilder::new(),
        data.free_time_of("Does not exist"),
        "Does Not Exist does not exist.",
        "Could get free time of nonexistent entity"
    );
}
