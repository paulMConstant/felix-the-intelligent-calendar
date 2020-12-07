//! Operations on groups which depend on entities.
//!
//! Includes:
//! - Addition of entities
//! - Deletion of entities
//! - Renaming groups

use plan_backend::data::Data;

// *** Add entity to group ***
#[test]
fn simple_add_entity_to_group() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    let entity_name = data.add_entity("Entity").expect("Could not add entity");

    data.add_entity_to_group(group_name.clone(), entity_name.clone())
        .expect("Could not add entity to group");
    let entities = data
        .group(group_name)
        .expect("Could not get group by name")
        .entities_sorted();
    assert_eq!(entities.len(), 1, "Entity was not added to group");
    assert_eq!(
        entities[0], &entity_name,
        "Entity was not added with right name"
    );
}

#[test]
fn add_entity_to_group_already_in_group() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    let entity_name = data.add_entity("Entity").expect("Could not add entity");
    data.add_entity_to_group(group_name.clone(), entity_name.clone())
        .expect("Could not add entity to group");

    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_group(group_name, entity_name),
            Err("Entity is already a member of the group 'Group'.".to_owned()),
            "Could add the same entity to the same group twice"
        );
    });
}

#[test]
fn add_nonexistent_entity_to_group() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_group(group_name, "entity"),
            Err("The entity 'Entity' does not exist.".to_owned()),
            "Could add nonexistent entity to group"
        );
    });
}

#[test]
fn add_entity_to_nonexistent_group() {
    let mut data = Data::new();

    let entity_name = data.add_entity("Entity").expect("Could not add entity");

    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_group("group", entity_name),
            Err("The group 'Group' does not exist.".to_owned()),
            "Could add entity to nonexistent group"
        );
    });
}

#[test]
fn add_entity_to_group_empty_names() {
    let mut data = Data::new();

    let entity_name = data.add_entity("Entity").expect("Could not add entity");
    let group_name = data.add_group("Group").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_group(group_name, "\t "),
            Err("The given name is empty.".to_owned()),
            "Could add entity with empty name to group"
        );

        assert_eq!(
            data.add_entity_to_group(" ", entity_name),
            Err("The given name is empty.".to_owned()),
            "Could add entity to group with empty name"
        );
    });
}

#[test]
fn add_entity_to_group_check_sorting() {
    let mut data = Data::new();

    let entity1 = data.add_entity("Entity1").expect("Could not add entity");
    let entity2 = data.add_entity("Entity2").expect("Could not add entity");
    let group_name = data.add_group("Group").expect("Could not add group");

    data.add_entity_to_group(group_name.clone(), entity2.clone())
        .expect("Could not add entity to group");
    data.add_entity_to_group(group_name.clone(), entity1.clone())
        .expect("Could not add entity to group");

    let entities = data
        .group(group_name)
        .expect("Could not get group")
        .entities_sorted();
    assert_eq!(entities[0], &entity1, "Group members are not sorted");
    assert_eq!(entities[1], &entity2, "Group members are not sorted");
}

#[test]
fn add_entity_to_group_not_enough_time_left() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    let entity_name = data.add_entity("Entity").expect("Could not add entity");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    data.add_group_to_activity(id, group_name.clone())
        .expect("Could not add group to activity");
    // The entity which is added to the group will take part in every activity of the group
    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity_to_group(&group_name, entity_name),
            Err(
                "Entity does not have enough time for the activities of the group 'Group'."
                    .to_owned()
            ),
            "Could add entity with not enough time to group"
        )
    });
}

// *** Remove entity from group ***
#[test]
fn simple_remove_entity() {
    let mut data = Data::new();

    // Add two entities to check that the right one is removed
    let entity_name1 = data.add_entity("Entity1").expect("Could not add entity");
    let entity_name2 = data.add_entity("Entity2").expect("Could not add entity");
    let group_name = data.add_group("Group").expect("Could not add group");
    data.add_entity_to_group(group_name.clone(), entity_name1.clone())
        .expect("Could not add entity to group");
    data.add_entity_to_group(group_name.clone(), entity_name2.clone())
        .expect("Could not add entity to group");

    data.remove_entity_from_group(group_name.clone(), entity_name1.clone())
        .expect("Could not remove entity from group");
    let entities = data
        .group(group_name)
        .expect("Could not get group by name")
        .entities_sorted();
    assert_eq!(entities.len(), 1, "Entity was not removed from group");
    assert_eq!(entities[0], &entity_name2, "The wrong entity was removed");
}

#[test]
fn remove_entity_not_in_group() {
    let mut data = Data::new();

    let entity_name = data.add_entity("Entity").expect("Could not add entity");
    let group_name = data.add_group("Group").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_group(group_name, entity_name),
            Err("Entity is not a member of the group 'Group'.".to_owned()),
            "Could remove entity which is not a member of a group"
        );
    });
}

#[test]
fn remove_nonexistent_entity_from_group() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_group(group_name, "Does not exist"),
            Err("The entity 'Does Not Exist' does not exist.".to_owned()),
            "Could remove nonexistent entity from group"
        );
    });
}

#[test]
fn remove_entity_from_nonexistent_group() {
    let mut data = Data::new();

    let entity_name = data.add_entity("Entity").expect("Could not add entiy");
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_group("Does not exist", entity_name),
            Err("The group 'Does Not Exist' does not exist.".to_owned()),
            "Could remove entity from nonexistent group"
        );
    });
}

#[test]
fn remove_entity_empty_names() {
    let mut data = Data::new();

    let entity_name = data.add_entity("Entity").expect("Could not add entity");
    let group_name = data.add_group("Group name").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity_from_group(" ", entity_name),
            Err("The given name is empty.".to_owned()),
            "Could remove entity from group with empty name"
        );

        assert_eq!(
            data.remove_entity_from_group(group_name, " "),
            Err("The given name is empty.".to_owned()),
            "Could remove entity with empty name from group"
        );
    });
}

#[test]
fn rename_group_name_taken_by_entity() {
    let mut data = Data::new();

    let group_name = data.add_group("Group name").expect("Could not add group");
    let entity_name = data.add_entity("name").expect("Could not add entity");

    assert_not_modified!(data, {
        assert_eq!(
            data.set_group_name(group_name, entity_name),
            Err("The name 'Name' is already taken by an entity.".to_owned()),
            "Could add group with name taken by an entity"
        );
    });
}
