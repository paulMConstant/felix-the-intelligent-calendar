//! Basic operations on entities.
//! Does not check interaction with work hours, groups or entities.
//!
//! Includes:
//! - Addition of activities
//! - Deletion of activities
//! - Edition (name, duration of activities)
//! - Getter for activity

use felix_backend::data::Time;
use test_utils::data_builder::{Activity, DataBuilder};

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
