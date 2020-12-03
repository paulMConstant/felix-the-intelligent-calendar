//! Operations on activities which depend on entities.
//! Checks interactions with entities.
//!
//! Includes:
//! - Addition of entities to the activity
//! - Deletion of entities from the activity
//! - Changing the duration of the activity (makes sure all entities have enough time)

use plan_backend::data::{Data, Time, TimeInterval};

// *** Add entities ***
#[test]
fn simple_add_entity() {
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

    data.add_entity_to_activity(id, name.clone())
        .expect("Could not add entity");
    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();
    assert_eq!(entities.len(), 1, "Participant was not added");
    assert_eq!(entities[0], name, "Participant was added with wrong name");
}

#[test]
fn add_entities_check_sorting() {
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

    data.add_entity_to_activity(id, name1.clone())
        .expect("Could not add entity");
    data.add_entity_to_activity(id, name3.clone())
        .expect("Could not add entity");
    data.add_entity_to_activity(id, name2.clone())
        .expect("Could not add entity");

    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();
    assert_eq!(entities.len(), 3, "Participants were not added");
    assert_eq!(entities[0], name1, "Participants are not sorted");
    assert_eq!(entities[1], name2, "Participants are not sorted");
    assert_eq!(entities[2], name3, "Participants are not sorted");
}

#[test]
fn add_entity_not_enough_time() {
    let mut data = Data::new();

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();

    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_activity(id, name),
            Err("Entity1 does not have enough time left for this activity.".to_owned()),
            "Could add entity with not enough time"
        );
    });
}

#[test]
fn add_entity_already_participating() {
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

    data.add_entity_to_activity(id, name.clone())
        .expect("Could not add entity");
    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_activity(id, name),
            Err("Entity1 is already taking part in the activity 'Name'.".to_owned()),
            "Could add the same entity twice"
        );
    });
}

#[test]
fn add_entity_wrong_id() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let name = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();
    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_activity(0, name),
            Err("Cannot get activity with id 0.".to_owned()),
            "Could add entity to activity with wrong id"
        );
    });
}

#[test]
fn add_entity_does_not_exist() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_activity(id, "Does not exist"),
            Err("The entity 'Does Not Exist' does not exist.".to_owned()),
            "Could add nonexistent entity to activity"
        );
    });
}

// *** Remove entities ***
#[test]
fn simple_remove_entity() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();

    // Add two entities to check that the right one is removed
    let name1 = data
        .add_entity("Name1")
        .expect("Could not add entity")
        .name();
    let name2 = data
        .add_entity("Name2")
        .expect("Could not add entity")
        .name();

    data.add_entity_to_activity(id, name1.clone())
        .expect("Could not add entity");
    data.add_entity_to_activity(id, name2.clone())
        .expect("Could not add entity");

    data.remove_entity_from_activity(id, name1)
        .expect("Could not remove entity");
    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();

    assert_eq!(entities.len(), 1, "Participant was not removed");
    assert_eq!(entities[0], name2, "The wrong entity was removed");
}

#[test]
fn remove_entity_not_participating() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    // Participant does not exist
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_activity(id, "Does not exist"),
            Err("The entity 'Does Not Exist' does not exist.".to_owned()),
            "Could remove entity which does not exist"
        );
    });
    // Participant exists but is not entitying
    let name = data
        .add_entity("Not participating")
        .expect("Could not add entity")
        .name();
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_activity(id, name),
            Err("Not Participating is not taking part in the activity 'Name'.".to_owned()),
            "Could remove entity which is not taking part in the activity"
        );
    });
}

#[test]
fn remove_entity_wrong_activity_id() {
    let mut data = Data::new();

    let name = data
        .add_entity("Entity")
        .expect("Could not add entity")
        .name();
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_activity(0, name),
            Err("Cannot get activity with id 0.".to_owned()),
            "Could remove entity from nonexistent activity"
        );
    });
}

// *** Set duration ***

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
    data.add_entity_to_activity(id, name)
        .expect("Could not add entity to activity");

    assert_not_modified!(data, {
        assert_eq!(
            data.set_activity_duration(id, duration),
            Err("Entity does not have enough time for the new duration.".to_owned()),
            "Could set duration where an entity has not enough free time"
        );
    });
}
