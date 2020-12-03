//! Basic operations on entities.
//! Does not check interaction with work hours, groups or entities.
//!
//! Includes:
//! - Addition of activities
//! - Deletion of activities
//! - Edition (name, duration of activities)
//! - Getter for activity

use plan_backend::data::{Data, Time};

// *** Add ***
#[test]
fn simple_add_activity() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    let activities = data.activities_sorted();
    assert_eq!(activities.len(), 1, "Activity was not added");
    assert_eq!(activities[0].id(), id, "Activity was not added right");
}

#[test]
fn add_activity_check_formatting() {
    let mut data = Data::new();

    let name = data
        .add_activity("MY   ACtIvity \t")
        .expect("Could not add activity")
        .name();
    assert_eq!(name, "My Activity", "Activity name was not formatted right");
}

#[test]
fn add_activity_check_sorting() {
    let mut data = Data::new();

    let id2 = data
        .add_activity("Name2")
        .expect("Could not add activity")
        .id();
    let id1 = data
        .add_activity("Name1")
        .expect("Could not add activity")
        .id();
    let id3 = data
        .add_activity("Name3")
        .expect("Could not add activity")
        .id();

    let activities = data.activities_sorted();
    assert_eq!(activities.len(), 3, "Activities were not added");
    assert_eq!(activities[0].id(), id1, "Activities are not sorted");
    assert_eq!(activities[1].id(), id2, "Activities are not sorted");
    assert_eq!(activities[2].id(), id3, "Activities are not sorted");
}

#[test]
fn add_activity_empty_name() {
    let mut data = Data::new();

    assert_not_modified!(data, {
        assert_eq!(
            data.add_activity("  \t"),
            Err("The given name is empty.".to_owned()),
            "Could add activity with empty name"
        );
    });
}

// *** Remove ***
#[test]
fn simple_remove_activity() {
    let mut data = Data::new();

    // Add two activities and check that the right one was removed
    let id1 = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let id2 = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    data.remove_activity(id1)
        .expect("Could not remove activity");

    let activities = data.activities_sorted();
    assert_eq!(activities.len(), 1, "Activity was not removed");
    assert_eq!(activities[0].id(), id2, "The wrong activity was removed");
}

#[test]
fn remove_invalid_activity() {
    let mut data = Data::new();

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_activity(0),
            Err("The activity with id 0 does not exist.".to_owned()),
            "Could remove an activity with wrong id"
        );
    });
}

// *** Get individual activity ***
#[test]
fn simple_get_activity() {
    let mut data = Data::new();

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let activity = data.activity(id).expect("Could not fetch activity by id");
    assert_eq!(activity.id(), id, "Fetched activity with wrong id");
}

#[test]
fn get_activity_with_wrong_id() {
    let data = Data::new();

    assert_eq!(
        data.activity(0),
        Err("Cannot get activity with id 0.".to_owned()),
        "Could get activity with wrong id"
    );
}

// *** Set name ***
#[test]
fn simple_set_activity_name() {
    let mut data = Data::new();
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();

    let new_name = "New Name";
    data.set_activity_name(id, new_name)
        .expect("Could not set activity name");

    let name = data
        .activity(id)
        .expect("Could not get activity by id")
        .name();
    assert_eq!(name, new_name, "Activity was not renamed");
}

#[test]
fn set_activity_name_check_formatting() {
    let mut data = Data::new();
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();

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

#[test]
fn set_activity_name_invalid_id() {
    let mut data = Data::new();

    let new_name = "New Name";
    assert_not_modified!(data, {
        assert_eq!(
            data.set_activity_name(0, new_name),
            Err("Cannot get activity with id 0.".to_owned()),
            "Could set name of activity with invalid id"
        );
    });
}

#[test]
fn set_activity_name_empty_name() {
    let mut data = Data::new();
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();

    assert_not_modified!(data, {
        assert_eq!(
            data.set_activity_name(id, " \t"),
            Err("The given name is empty.".to_owned()),
            "Could set empty activity name"
        );
    });
}

// *** Set duration ***
#[test]
fn simple_set_activity_duration() {
    let mut data = Data::new();

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
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

#[test]
fn set_activity_duration_invalid_id() {
    let mut data = Data::new();
    let duration = Time::new(1, 0);

    assert_not_modified!(data, {
        assert_eq!(
            data.set_activity_duration(0, duration),
            Err("Cannot get activity with id 0.".to_owned()),
            "Could set the duration of nonexistent activity"
        );
    });
}

#[test]
fn set_activity_duration_too_short() {
    let mut data = Data::new();
    let duration = Time::new(0, 0);

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    assert_not_modified!(data, {
        assert_eq!(
            data.set_activity_duration(id, duration),
            Err("The given duration is too short.".to_owned()),
            "Could set the duration to 0"
        );
    });
}
