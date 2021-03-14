//! Basic operations on entities.
//! Does not check interaction with work hours, groups or entities.
//!
//! Includes:
//! - Addition of activities
//! - Deletion of activities
//! - Edition (name, duration of activities)
//! - Getter for activity
//! - Set color
//! - Activity insertion

use felix_backend::data::{Rgba, Time, TimeInterval};
use test_utils::{Activity, DataBuilder};

use std::collections::BTreeSet;

// *** Add ***
#[test]
fn simple_add_activity() {
    let name = "Activity";
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity {
            name,
            ..Default::default()
        }),
        {
            let activities = data.activities_sorted();

            assert_eq!(activities.len(), 1, "Activity was not added");
            assert_eq!(activities[0].name(), name, "Name was not returned right");
        }
    );
}

#[test]
fn add_activity_check_formatting() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity {
            name: "MY AcTiviTY \t",
            ..Default::default()
        }),
        assert_eq!(
            data.activities_sorted()[0].name(),
            "My Activity",
            "Activity name was not formatted right"
        )
    );
}

#[test]
fn add_activity_check_sorting() {
    let (name1, name2, name3) = ("Name1", "Name2", "Name3");
    test_ok!(
        data,
        DataBuilder::new().with_activities(vec![
            Activity {
                name: name1,
                ..Default::default()
            },
            Activity {
                name: name3,
                ..Default::default()
            },
            Activity {
                name: name2,
                ..Default::default()
            }
        ]),
        {
            let activities = data.activities_sorted();
            assert_eq!(activities.len(), 3, "Activities were not added");
            assert_eq!(activities[0].name(), name1, "Activities are not sorted");
            assert_eq!(activities[1].name(), name2, "Activities are not sorted");
            assert_eq!(activities[2].name(), name3, "Activities are not sorted");
        }
    );
}

#[test]
fn add_activity_empty_name() {
    test_err!(
        data,
        DataBuilder::new(),
        data.add_activity(" \t"),
        "The given name is empty.",
        "Could add entity with empty name"
    );
}

// *** Remove ***
#[test]
fn simple_remove_activity() {
    test_ok!(
        data,
        DataBuilder::new().with_activities(vec![Activity::default(), Activity::default()]),
        {
            let activities = data.activities_sorted();
            let (id1, id2) = (activities[0].id(), activities[1].id());
            data.remove_activity(id1)
                .expect("Could not remove activity");

            let activities = data.activities_sorted();
            assert_eq!(activities.len(), 1, "Activity was not removed");
            assert_eq!(activities[0].id(), id2, "The wrong activity was removed");
        }
    );
}

#[test]
fn remove_invalid_activity() {
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_activity(0),
        "The activity with id '0' does not exist.",
        "Could remove an activity with wrong id"
    );
}

// *** Get individual activity ***
#[test]
fn simple_get_activity() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            let activity = data.activity(id).expect("Could not fetch activity by id");
            assert_eq!(activity.id(), id, "Fetched activity with wrong id");
        }
    );
}

#[test]
fn get_activity_with_wrong_id() {
    test_err!(
        data,
        DataBuilder::new(),
        data.activity(0),
        "The activity with id '0' does not exist.",
        "Could get activity with wrong id"
    );
}

// *** Set name ***
#[test]
fn simple_set_activity_name() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            let new_name = data
                .set_activity_name(id, "New name")
                .expect("Could not set activity name");
            let actual_name = data.activity(id).expect("Could not get activity").name();

            assert_eq!(new_name, actual_name, "Activity was not renamed");
        }
    );
}

#[test]
fn set_activity_name_check_formatting() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            let unformatted_name = "new  NAmE \t";
            let formatted_name = "New Name";
            data.set_activity_name(id, unformatted_name)
                .expect("Could not set activity name");
            let name = data
                .activity(id)
                .expect("Could not get activity by id")
                .name();
            assert_eq!(name, formatted_name, "Activity name was not formatted");
        }
    );
}

#[test]
fn set_activity_name_invalid_id() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        data.set_activity_name(3, "New Name"),
        "The activity with id '3' does not exist.",
        "Could set name of activity with invalid id"
    );
}

#[test]
fn set_activity_name_empty_name() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.set_activity_name(id, "\t ")
        },
        "The given name is empty.",
        "Could set empty activity name"
    );
}

// *** Set duration ***
#[test]
fn simple_set_activity_duration() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            let duration = Time::new(1, 0);
            let activity_duration = data
                .activity(id)
                .expect("Could not get activity by id")
                .duration();
            assert_ne!(
                activity_duration, duration,
                "Test is pointless: duration is the same"
            );

            data.set_activity_duration(id, duration)
                .expect("Could not set activity duration");
            let activity_duration = data
                .activity(id)
                .expect("Could not get activity by id")
                .duration();
            assert_eq!(activity_duration, duration, "Duration was not set properly");
        }
    );
}

#[test]
fn set_activity_duration_invalid_id() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        data.set_activity_duration(2, Time::new(1, 0)),
        "The activity with id '2' does not exist.",
        "Could set the duration of nonexistent activity"
    );
}

#[test]
fn set_activity_duration_too_short() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.set_activity_duration(id, Time::new(0, 0))
        },
        "The given duration is too short.",
        "Could add activity with duration 0"
    );
}

#[test]
fn decrease_activity_duration_check_insertion_interval_updated() {
    let name = "Déborah";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(name)
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(9, 0)))
            .with_activity(Activity {
                duration: Time::new(1, 0),
                name: "Activity",
                groups: Vec::new(),
                entities: vec![name],
                insertion_time: Some(Time::new(8, 00)),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            let activity = data.activity(id).expect("Could not get activity by ID");
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 00), Time::new(9, 0));
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval)
            );

            // Decrease the duration, check that the insertion interval is still valid
            data.set_activity_duration(id, Time::new(0, 30)).unwrap();
            let activity = data.activity(id).expect("Could not get activity by ID");
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 00), Time::new(8, 30));
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval)
            );
        }
    );
}

#[test]
fn increase_activity_duration_check_insertion_interval_removed() {
    let name = "Déborah";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(name)
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(9, 0)))
            .with_activity(Activity {
                duration: Time::new(0, 30),
                name: "Activity",
                groups: Vec::new(),
                entities: vec![name],
                insertion_time: Some(Time::new(8, 00)),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            let activity = data.activity(id).expect("Could not get activity by ID");
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 00), Time::new(8, 30));
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval)
            );

            // Change the duration, check that the insertion interval is removed
            data.set_activity_duration(id, Time::new(1, 0)).unwrap();
            let activity = data.activity(id).expect("Could not get activity by ID");
            let expected_insertion_interval = None;
            assert_eq!(activity.insertion_interval(), expected_insertion_interval);
        }
    );
}

#[test]
fn increase_activity_duration_then_insert_activity_automatically_in_closest_spot() {
    let name = "Déborah";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(name)
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(9, 0)))
            .with_activity(Activity {
                duration: Time::new(0, 30),
                name: "Activity",
                groups: Vec::new(),
                entities: vec![name],
                insertion_time: Some(Time::new(8, 30)),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            let activity = data.activity(id).expect("Could not get activity by ID");
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 30), Time::new(9, 00));
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval)
            );

            // Change the duration, check that the insertion interval is removed
            data.set_activity_duration(id, Time::new(1, 0)).unwrap();
            let activity = data.activity(id).expect("Could not get activity by ID");
            let expected_insertion_interval = None;
            assert_eq!(activity.insertion_interval(), expected_insertion_interval);

            // Wait for computation result
            while data
                .possible_insertion_times_of_activity(id)
                .unwrap()
                .is_none()
            {
                // For the purpose of this test, wait for asynchronous computation of possible beginnings.
            }

            // Ask data to find the closest spot for the activity
            data.insert_activities_removed_because_duration_increased_in_closest_spot();
            let insertion_beginning = data
                .activity(id)
                .expect("Could not get activity by ID")
                .insertion_interval()
                .expect("Activity was not reinserted in the schedule")
                .beginning();
            assert_eq!(insertion_beginning, Time::new(8, 0));
        }
    );
}

// *** Set activity color ***
#[test]
fn basic_set_color() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            let color = Rgba {
                red: 0.8,
                green: 0.7,
                blue: 0.5,
                alpha: 1.0,
            };
            data.set_activity_color(id, color).unwrap();
            let activity = data.activity(id).expect("Could not get activity by id!");
            assert_eq!(activity.color(), color);
        }
    );
}

// *** Activity insertion ***
#[test]
fn basic_insert_activity() {
    let (name1, name2) = ("Paul", "Antoine");
    let activity_duration = Time::new(0, 30);
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1, name2])
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_activity(Activity {
                name: "Activity",
                entities: vec![name1, name2],
                duration: activity_duration,
                groups: Vec::new(),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            while data
                .possible_insertion_times_of_activity(id)
                .expect("Could not get activity by ID")
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            let beginning = Time::new(10, 0);
            let expected_insertion_interval =
                TimeInterval::new(beginning, beginning + activity_duration);
            assert!(data.insert_activity(id, Some(beginning)).is_ok());

            let activity = data.activity(id).expect("Could not get activity by ID");
            assert_eq!(
                activity.insertion_interval().unwrap(),
                expected_insertion_interval
            );
        }
    );
}

#[test]
fn basic_insert_activity_invalid_time() {
    let (name1, name2) = ("Paul", "Antoine");
    let activity_duration = Time::new(0, 30);
    test_err!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1, name2])
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_activity(Activity {
                name: "Activity",
                entities: vec![name1, name2],
                duration: activity_duration,
                groups: Vec::new(),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            while data
                .possible_insertion_times_of_activity(id)
                .expect("Could not get activity by ID")
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            let beginning = Time::new(14, 0);
            data.insert_activity(id, Some(beginning))
        },
        "Activity cannot be inserted with beginning 14:00 because this beginning is invalid or will cause problems in the future.",
        "Could insert activity in invalid interval"
    );
}

#[test]
fn insert_activity_invalid_time_overlaps() {
    let (name1, name2) = ("Paul", "Antoine");
    let activity_duration = Time::new(0, 30);
    let beginning1 = Time::new(10, 20);
    let beginning2 = Time::new(10, 0);
    test_err!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1, name2])
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_activities(vec![
                Activity {
                    name: "Blocking Activity",
                    entities: vec![name1, name2],
                    duration: activity_duration,
                    groups: Vec::new(),
                    insertion_time: Some(beginning1),
                    ..Default::default()
                }, Activity {
                name: "Activity",
                entities: vec![name1, name2],
                duration: activity_duration,
                groups: Vec::new(),
                ..Default::default()
            }]),
        {
            let id = data.activities_sorted()[0].id();
            while data
                .possible_insertion_times_of_activity(id)
                .expect("Could not get activity by ID")
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            data.insert_activity(id, Some(beginning2))
        },
        "Activity cannot be inserted with beginning 10:00 because it would overlap with 'Blocking Activity'.",
        "Could insert activity in invalid interval"
    );
}

#[test]
fn possible_insertion_times_takes_insertion_conflict_into_account() {
    let name = "Paul";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(name)
            .with_work_interval(TimeInterval::new(Time::new(10, 0), Time::new(13, 0)))
            .with_activities(vec![
                Activity {
                    name: "Activity",
                    entities: vec![name],
                    duration: Time::new(1, 0),
                    groups: Vec::new(),
                    insertion_time: Some(Time::new(11, 0)),
                    ..Default::default()
                },
                Activity {
                    name: "Activity2",
                    entities: vec![name],
                    duration: Time::new(1, 0),
                    groups: Vec::new(),
                    ..Default::default()
                }
            ]),
        {
            let id2 = data.activities_sorted()[1].id();
            while data
                .possible_insertion_times_of_activity(id2)
                .expect("Could not get activity by ID")
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }
            // The only beginnings left are 10:00 and 12:00
            // (work hours are [10:00 - 13:00] with [11:00 - 12:00] taken by activity 1)
            assert_eq!(data.possible_insertion_times_of_activity(id2).unwrap().unwrap(),
                       [Time::new(10, 0), Time::new(12, 0)].iter().copied().collect::<BTreeSet<_>>(),
               "Insertion times with conflicts with inserted activities were not calculated right.");
        }
    );
}

#[test]
fn possible_insertion_times_takes_heterogeneous_work_hours_of_participants_into_account() {
    let (name1, name2) = ("Paul", "Jeanne");
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1, name2])
            .with_custom_work_interval_for(
                name1,
                TimeInterval::new(Time::new(9, 0), Time::new(11, 0))
            )
            .with_work_interval(TimeInterval::new(Time::new(10, 0), Time::new(13, 0)))
            .with_activity(Activity {
                name: "Activity",
                entities: vec![name1, name2],
                duration: Time::new(1, 0),
                groups: Vec::new(),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            while data
                .possible_insertion_times_of_activity(id)
                .expect("Could not get activity by ID")
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }
            // The only beginnings is 10:00
            // Activity duration is 01:00 and intersection of work hours is [10:00 - 11:00]
            assert_eq!(data.possible_insertion_times_of_activity(id).unwrap().unwrap(),
                      [Time::new(10, 0)].iter().copied().collect::<BTreeSet<_>>(),
              "Insertion times with conflicts with inserted activities were not calculated right.");
        }
    );
}
