//! Operations on entities which depend on activities.
//! Includes
//! - Renaming
//! - Removing

#[test]
fn rename_entity_check_renamed_in_activity() {
    let mut data = Data::new();

    // Add work hours and an activity
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add valid work interval");
    let id = data
        .add_activity("Activity")
        .expect("Could not add valid activity")
        .id();

    // Add two entities to check that the right one is renamed
    let name1 = data
        .add_entity("Entity name1")
        .expect("Could not add valid entity")
        .name();
    let name2 = data
        .add_entity("Entity name2")
        .expect("Could not add valid entity")
        .name();
    data.add_entity_to_activity(id, name1.clone())
        .expect("Could not add entity to activity");
    data.add_entity_to_activity(id, name2.clone())
        .expect("Could not add entity to activity");

    let entities = data
        .activity(id)
        .expect("Could not get activity")
        .entities_sorted();
    assert_eq!(entities.len(), 2, "Entities were not added to the activity");

    let name3 = data
        .set_entity_name(name1, "Entity name3")
        .expect("Could not rename entity");
    let entities = data
        .activity(id)
        .expect("Could not get activity")
        .entities_sorted();

    assert_eq!(
        entities[0], name2,
        "The wrong entity was renamed or they are not sorted"
    );
    assert_eq!(
        entities[1], name3,
        "The entity was not renamed or they are not sorted"
    );
}

#[test]
fn remove_entity_check_remove_in_activity() {
    let mut data = Data::new();

    // Add work hours and an activity
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add valid work interval");
    let id = data
        .add_activity("Activity")
        .expect("Could not add valid activity")
        .id();

    // Add two entities to check that the right one is removed
    let name1 = data
        .add_entity("Entity name1")
        .expect("Could not add valid entity")
        .name();
    let name2 = data
        .add_entity("Entity name2")
        .expect("Could not add valid entity")
        .name();
    data.add_entity_to_activity(id, name1.clone())
        .expect("Could not add entity to activity");
    data.add_entity_to_activity(id, name2.clone())
        .expect("Could not add entity to activity");

    let entities = data
        .activity(id)
        .expect("Could not get activity")
        .entities_sorted();
    assert_eq!(entities.len(), 2, "Entities were not added to the activity");

    data.remove_entity(name1).expect("Could not remove entity");
    let entities = data
        .activity(id)
        .expect("Could not get activity")
        .entities_sorted();
    assert_eq!(
        entities.len(),
        1,
        "Entity was not removed from the activity"
        );
    assert_eq!(entities[0], name2, "The wrong entity was removed");
}
