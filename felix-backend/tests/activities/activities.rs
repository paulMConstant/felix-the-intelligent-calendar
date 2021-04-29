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
            data.remove_activity(id1);

            let activities = data.activities_sorted();
            assert_eq!(activities.len(), 1, "Activity was not removed");
            assert_eq!(activities[0].id(), id2, "The wrong activity was removed");
        }
    );
}

#[test]
#[should_panic]
fn remove_invalid_activity() {
    let mut data = DataBuilder::new().into_data();
    data.remove_activity(0);
}

// *** Get individual activity ***
#[test]
fn simple_get_activity() {
    test_ok!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            let activity = data.activity(id);
            assert_eq!(activity.id(), id, "Fetched activity with wrong id");
        }
    );
}

#[test]
fn get_activity_with_wrong_id() {
    std::panic::catch_unwind(|| {
        let data = DataBuilder::new().into_data();
        data.activity(0);
    })
    .expect_err("Could get activity with wrong id");
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
            let actual_name = data.activity(id).name();

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
            let name = data.activity(id).name();
            assert_eq!(name, formatted_name, "Activity name was not formatted");
        }
    );
}

#[test]
fn set_activity_name_invalid_id() {
    std::panic::catch_unwind(|| {
        let mut data = DataBuilder::new()
            .with_activity(Activity::default())
            .into_data();
        data.set_activity_name(3, "New Name")
            .expect("Sould panic: id is invalid");
    })
    .expect_err("Could set name of activity with invalid id");
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
            let activity_duration = data.activity(id).duration();
            assert_ne!(
                activity_duration, duration,
                "Test is pointless: duration is the same"
            );

            data.set_activity_duration(id, duration)
                .expect("Could not set activity duration");
            let activity_duration = data.activity(id).duration();
            assert_eq!(activity_duration, duration, "Duration was not set properly");
        }
    );
}

#[test]
fn set_activity_duration_invalid_id() {
    std::panic::catch_unwind(|| {
        let mut data = DataBuilder::new()
            .with_activity(Activity::default())
            .into_data();
        data.set_activity_duration(2, Time::new(1, 0)).unwrap();
    })
    .expect_err("Could set the duration of nonexistent activity");
}

#[test]
fn set_activity_duration_zero_remove_from_schedule_if_inserted() {
    let name = "Lyuba";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entity(name)
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_activity(Activity {
                entities: vec![name],
                duration: Time::new(0, 10),
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.set_activity_duration(id, Time::new(0, 0))
                .expect("Cannot set activity duration");
            assert!(
                data.activity(id).insertion_interval().is_none(),
                "Activity is inserted with empty duration"
            );
        }
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
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 00), Time::new(9, 0));
            while data.activity(id).insertion_interval().is_none() {
                // Wait until insertion interval is updated
            }

            assert_eq!(
                data.activity(id).insertion_interval(),
                Some(expected_insertion_interval)
            );

            // Decrease the duration, check that the insertion interval is still valid
            data.set_activity_duration(id, Time::new(0, 30)).unwrap();
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 00), Time::new(8, 30));
            while data.activity(id).insertion_interval().is_none() {
                // Wait until insertion interval is updated
            }
            assert_eq!(
                data.activity(id).insertion_interval(),
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
            let activity = data.activity(id);
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 00), Time::new(8, 30));
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval)
            );

            // Change the duration, check that the insertion interval is removed
            data.set_activity_duration(id, Time::new(1, 0)).unwrap();
            let activity = data.activity(id);
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
            let activity = data.activity(id);
            let expected_insertion_interval = TimeInterval::new(Time::new(8, 30), Time::new(9, 00));
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval)
            );

            // Change the duration, check that the insertion interval is removed
            data.set_activity_duration(id, Time::new(1, 0)).unwrap();
            let activity = data.activity(id);
            let expected_insertion_interval = None;
            assert_eq!(activity.insertion_interval(), expected_insertion_interval);

            // Wait for computation result
            while data
                .possible_insertion_times_of_activity_with_associated_cost(id)
                .is_none()
            {
                // For the purpose of this test, wait for asynchronous computation of possible beginnings.
            }

            // Ask data to find the closest spot for the activity
            data.insert_activities_removed_because_duration_increased_in_closest_spot();
            let insertion_beginning = data
                .activity(id)
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
            let activity = data.activity(id);
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
                .possible_insertion_times_of_activity_with_associated_cost(id)
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            let beginning = Time::new(10, 0);
            let expected_insertion_interval =
                TimeInterval::new(beginning, beginning + activity_duration);
            assert!(data.insert_activity(id, Some(beginning)).is_ok());

            let activity = data.activity(id);
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
                .possible_insertion_times_of_activity_with_associated_cost(id)
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
                .possible_insertion_times_of_activity_with_associated_cost(id)
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
                .possible_insertion_times_of_activity_with_associated_cost(id2)
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }
            // The only beginnings left are 10:00 and 12:00
            // (work hours are [10:00 - 13:00] with [11:00 - 12:00] taken by activity 1)
            assert_eq!(data.possible_insertion_times_of_activity_with_associated_cost(id2)
                        .unwrap().iter().map(|insertion_cost| insertion_cost.beginning)
                        .collect::<BTreeSet<_>>(),
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
                .possible_insertion_times_of_activity_with_associated_cost(id)
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }
            // The only beginnings is 10:00
            // Activity duration is 01:00 and intersection of work hours is [10:00 - 11:00]
            assert_eq!(data.possible_insertion_times_of_activity_with_associated_cost(id)
                       .unwrap().iter().map(|insertion_cost| insertion_cost.beginning)
                       .collect::<BTreeSet<_>>(),
                      [Time::new(10, 0)].iter().copied().collect::<BTreeSet<_>>(),
              "Insertion times with conflicts with inserted activities were not calculated right.");
        }
    );
}

#[test]
fn activities_with_empty_duration_not_taken_into_account_in_insertion_costs() {
    let name1 = "Paul";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1])
            .with_work_interval(TimeInterval::new(Time::new(10, 0), Time::new(13, 0)))
            .with_activities(vec![
                Activity {
                    name: "Activity1",
                    entities: vec![name1],
                    duration: Time::new(1, 0),
                    groups: Vec::new(),
                    ..Default::default()
                },
                Activity {
                    name: "Activity2",
                    entities: vec![name1],
                    duration: Time::new(1, 0),
                    groups: Vec::new(),
                    ..Default::default()
                }
            ]),
        {
            let id1 = data.activities_sorted()[0].id();
            let id2 = data.activities_sorted()[1].id();
            while data
                .possible_insertion_times_of_activity_with_associated_cost(id1)
                .is_none()
                || data
                    .possible_insertion_times_of_activity_with_associated_cost(id2)
                    .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            // Set empty duration -> Will invalidate insertion times of every incompatible activity
            data.set_activity_duration(id2, Time::new(0, 0))
                .expect("Could not set activity duration");
            while data
                .possible_insertion_times_of_activity_with_associated_cost(id1)
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            // Make sure there are no insertion costs for empty duration (never calculated)
            assert!(data
                .possible_insertion_times_of_activity_with_associated_cost(id2)
                .is_none());

            // Make sure it does not affect other activities
            assert!(!data
                .possible_insertion_times_of_activity_with_associated_cost(id1)
                .expect("We did not wait for possible insertion costs to be calculated")
                .is_empty());
        }
    );
}

/// This is a response to a bug in which the possible insertions of incompatible activities were
/// not calculated.
/// Insertion cost computation thought inserting the activity at any beginning would leave others
/// without any possible beginning and therefore every beginning was invalid.
#[test]
fn possible_insertion_costs_compute_possible_insertions_of_incompatible_activities_as_well() {
    let (name1, name2) = ("Paul", "Jeanne");
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1, name2])
            .with_work_interval(TimeInterval::new(Time::new(9, 0), Time::new(11, 0)))
            .with_activity(Activity {
                entities: vec![name1, name2],
                duration: Time::new(0, 10),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.add_activity("Other activity")
                .expect("Could not add activity");
            let other_id = data.activities_sorted()[1].id();
            data.set_activity_duration(other_id, Time::new(1, 0))
                .expect("Could not set activity duration");
            data.add_entity_to_activity(other_id, name1)
                .expect("Could not add participant to activity");

            while data
                .possible_insertion_times_of_activity_with_associated_cost(id)
                .is_none()
            {
                // Wait for possible insertion times to be asynchronously calculated
            }

            // Check that insertion times for activity with one entity only have been computed
            // (if this result is empty, they haven't)
            assert!(!data.possible_insertion_times_of_activity_with_associated_cost(id)
                    .unwrap()
                    .is_empty(),
              "Incompatible activities beginnings were not updated and therefore possible insertion times are empty");
        }
    );
}

#[test]
fn possible_insertion_costs_updated_when_activity_inserted() {
    let name1 = "Paul";
    test_err!(
        data,
        DataBuilder::new()
        .with_entities(vec![name1])
        .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
        .with_activities(vec![
         Activity {
            name: "Activity1",
            entities: vec![name1],
            duration: Time::new(1, 0),
            ..Default::default()
        },
        Activity {
            name: "Activity2",
            entities: vec![name1],
            duration: Time::new(1, 0),
            ..Default::default()
        }]
        ),
        {
            let id1 = data.activities_sorted()[0].id();
            let id2 = data.activities_sorted()[1].id();
            while data.possible_insertion_times_of_activity_with_associated_cost(id1).is_none()
                || data.possible_insertion_times_of_activity_with_associated_cost(id2).is_none()
            {
                // Wait for computation
            }

            data.insert_activity(id1, Some(Time::new(8, 30))).expect("Could not insert activity");

            while data.possible_insertion_times_of_activity_with_associated_cost(id2).is_none() {
                // Wait for computation
            }

            data.insert_activity(id2, Some(Time::new(8, 0)))
        },
        "Activity2 cannot be inserted with beginning 08:00 because it would overlap with 'Activity1'.",
        "Possible insertion costs were not updated when activity was inserted"
    );
}

#[test]
fn autoinsertion_launches_and_ignores_activities_with_zero_duration() {
    let name1 = "Paul";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1])
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_activities(vec![
                Activity {
                    name: "Activity1",
                    entities: vec![name1],
                    duration: Time::new(1, 0),
                    ..Default::default()
                },
                Activity {
                    name: "Activity2",
                    entities: vec![name1],
                    duration: Time::new(0, 0),
                    ..Default::default()
                }
            ]),
        {
            let id1 = data.activities_sorted()[0].id();
            while data
                .possible_insertion_times_of_activity_with_associated_cost(id1)
                .is_none()
            {
                // Wait for computation
            }

            let autoinsertion_handle = data.start_autoinsertion().expect(
                "Could not start autoinsertion: results should be computed for valid activities",
            );

            autoinsertion_handle
                .get_result()
                .expect("Autoinsertion failed");
        }
    );
}

#[test]
fn autoinsertion_launches_and_ignores_activities_with_no_participants() {
    let name1 = "Paul";
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![name1])
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_activities(vec![
                Activity {
                    name: "Activity1",
                    entities: vec![name1],
                    duration: Time::new(1, 0),
                    ..Default::default()
                },
                Activity {
                    name: "Activity2",
                    duration: Time::new(1, 0),
                    ..Default::default()
                }
            ]),
        {
            let id1 = data.activities_sorted()[0].id();
            while data
                .possible_insertion_times_of_activity_with_associated_cost(id1)
                .is_none()
            {
                // Wait for computation
            }

            let autoinsertion_handle = data.start_autoinsertion().expect(
                "Could not start autoinsertion: results should be computed for valid activities",
            );

            autoinsertion_handle
                .get_result()
                .expect("Autoinsertion failed");
        }
    );
}
