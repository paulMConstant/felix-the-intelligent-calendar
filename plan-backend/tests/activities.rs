use plan_backend::data::{Data, Time, TimeInterval};

// Test organization
// - Add
// - Remove
// - Get individual
// - Add participants
// - Add groups
// - Set name
// - Set duration

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
    assert_eq!(name, "My Activity", "Activity name was nor formatted right");
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

    assert_eq!(
        data.add_activity("  \t"),
        Err("The formatted name is empty.".to_owned()),
        "Could add activity with empty name"
    );
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

    assert_eq!(
        data.remove_activity(0),
        Err("The activity with id 0 does not exist !".to_owned()),
        "Could remove an activity with wrong id"
    );
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

// *** Add participants ***
#[test]
fn simple_add_participant() {
    let mut data = Data::new();

    // Without work hours, cannot add activity : not enough time
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();

    data.add_participant_to_activity(id, name.clone())
        .expect("Could not add participant");
    let participants = data
        .activity(id)
        .expect("Could not get activity by id")
        .participants_sorted();
    assert_eq!(participants.len(), 1, "Participant was not added");
    assert_eq!(
        participants[0], name,
        "Participant was added with wrong name"
    );
}

#[test]
fn add_participants_check_sorting() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name1 = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();
    let name2 = data
        .add_entity("Entity2")
        .expect("Could not add entity")
        .name();
    let name3 = data
        .add_entity("Entity3")
        .expect("Could not add entity")
        .name();

    data.add_participant_to_activity(id, name1.clone())
        .expect("Could not add participant");
    data.add_participant_to_activity(id, name3.clone())
        .expect("Could not add participant");
    data.add_participant_to_activity(id, name2.clone())
        .expect("Could not add participant");

    let participants = data
        .activity(id)
        .expect("Could not get activity by id")
        .participants_sorted();
    assert_eq!(participants.len(), 3, "Participants were not added");
    assert_eq!(participants[0], name1, "Participants are not sorted");
    assert_eq!(participants[1], name2, "Participants are not sorted");
    assert_eq!(participants[2], name3, "Participants are not sorted");
}

#[test]
fn add_participant_not_enough_time() {
    let mut data = Data::new();

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();

    assert_eq!(
        data.add_participant_to_activity(id, name),
        Err("Entity1 does not have enough time left for this activity.".to_owned()),
        "Could add participant with not enough time"
    );
}

#[test]
fn add_participant_already_participating() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();

    data.add_participant_to_activity(id, name.clone())
        .expect("Could not add participant");
    assert_eq!(
        data.add_participant_to_activity(id, name),
        Err("Entity1 is already taking part in the activity 'Name' !".to_owned()),
        "Could add the same participant twice"
    );
}

#[test]
fn add_participant_wrong_id() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let name = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();
    assert_eq!(
        data.add_participant_to_activity(0, name),
        Err("Cannot get activity with id 0.".to_owned()),
        "Could add entity to activity with wrong id"
    );
}

#[test]
fn add_participant_does_not_exist() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    assert_eq!(
        data.add_participant_to_activity(id, "Does not exist"),
        Err("Does Not Exist does not exist !".to_owned()),
        "Could add nonexistent participant to activity"
    );
}

// *** Remove participant ***
#[test]
fn simple_remove_participant() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();

    // Add two participants to check that the right one is removed
    let name1 = data
        .add_entity("Name1")
        .expect("Could not add entity")
        .name();
    let name2 = data
        .add_entity("Name2")
        .expect("Could not add entity")
        .name();

    data.add_participant_to_activity(id, name1.clone())
        .expect("Could not add participant");
    data.add_participant_to_activity(id, name2.clone())
        .expect("Could not add participant");

    data.remove_participant_from_activity(id, name1)
        .expect("Could not remove participant");
    let participants = data
        .activity(id)
        .expect("Could not get activity by id")
        .participants_sorted();

    assert_eq!(participants.len(), 1, "Participant was not removed");
    assert_eq!(participants[0], name2, "The wrong participant was removed");
}

#[test]
fn remove_participant_not_participating() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    // Participant does not exist
    assert_eq!(
        data.remove_participant_from_activity(id, "Does not exist"),
        Err("Does Not Exist does not exist !".to_owned()),
        "Could remove participant which does not exist"
    );
    // Participant exists but is not participanting
    let name = data
        .add_entity("Not participating")
        .expect("Could not add entity")
        .name();
    assert_eq!(
        data.remove_participant_from_activity(id, name),
        Err("Not Participating is not taking part in the activity 'Name' !".to_owned()),
        "Could remove participant which is not taking part in the activity"
    );
}

#[test]
fn remove_participant_wrong_activity_id() {
    let mut data = Data::new();

    let name = data
        .add_entity("Entity")
        .expect("Could not add entity")
        .name();
    assert_eq!(
        data.remove_participant_from_activity(0, name),
        Err("Cannot get activity with id 0.".to_owned()),
        "Could remove participant from nonexistent activity"
    );
}

// *** TODO Add groups ***

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
    assert_eq!(
        data.set_activity_name(0, new_name),
        Err("Cannot get activity with id 0.".to_owned()),
        "Could set name of activity with invalid id"
    );
}

#[test]
fn set_activity_name_empty_name() {
    let mut data = Data::new();
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();

    assert_eq!(
        data.set_activity_name(id, " \t"),
        Err("The formatted name is empty.".to_owned()),
        "Could set empty activity name"
    );
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

    assert_eq!(
        data.set_activity_duration(0, duration),
        Err("Cannot get activity with id 0.".to_owned()),
        "Could set the duration of nonexistent activity"
    );
}

#[test]
fn set_activity_duration_too_short() {
    let mut data = Data::new();
    let duration = Time::new(0, 0);

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    assert_eq!(
        data.set_activity_duration(id, duration),
        Err("The given duration is too short !".to_owned()),
        "Could set the duration to 0"
    );
}

#[test]
fn set_activity_duration_not_enough_free_time() {
    let mut data = Data::new();
    let duration = Time::new(5, 0);

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Entity")
        .expect("Could not add entity")
        .name();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert!(
        interval.duration() < duration,
        "Test is pointless: entity has enough time"
    );

    data.add_work_interval(interval)
        .expect("Could not add work interval");
    data.add_participant_to_activity(id, name)
        .expect("Could not add participant to activity");

    assert_eq!(
        data.set_activity_duration(id, duration),
        Err("Entity does not have enough time for the new duration.".to_owned()),
        "Could set duration where an entity has not enough free time"
    );
}
