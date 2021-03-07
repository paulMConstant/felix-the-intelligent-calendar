//! Operations on groups which depend on entities.
//!
//! Includes:
//! - Addition of entities
//! - Deletion of entities
//! - Renaming groups

use felix_backend::data::Time;
use test_utils::{Activity, DataBuilder, Group};

// *** Add entity to group ***
#[test]
fn simple_add_entity_to_group() {
    let group_name = "Group";
    let entity_name = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_group(Group::default(group_name))
            .with_entity(entity_name),
        {
            data.add_entity_to_group(group_name.clone(), entity_name.clone())
                .expect("Could not add entity to group");
            let entities = data
                .group(group_name)
                .expect("Could not get group by name")
                .entities_sorted();
            assert_eq!(entities.len(), 1, "Entity was not added to group");
            assert_eq!(
                entities[0], entity_name,
                "Entity was not added with right name"
            );
        }
    );
}

#[test]
fn add_entity_to_group_already_in_group() {
    let group_name = "Group";
    let entity_name = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity_name)
            .with_group(Group {
                name: group_name,
                entities: vec![entity_name]
            }),
        data.add_entity_to_group(group_name, entity_name),
        "Entity is already in the group 'Group'.",
        "Could add the same entity to the same group twice"
    );
}

#[test]
fn add_nonexistent_entity_to_group() {
    let group_name = "Group";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(group_name)),
        data.add_entity_to_group(group_name, "entity"),
        "Entity does not exist.",
        "Could add nonexistent entity to group"
    );
}

#[test]
fn add_entity_to_nonexistent_group() {
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.add_entity_to_group("group", entity),
        "The group 'Group' does not exist.",
        "Could add entity to nonexistent group"
    );
}

#[test]
fn add_entity_to_group_empty_group_name() {
    let group = "Group";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(group)),
        data.add_entity_to_group(group, "\t "),
        "The given name is empty.",
        "Could add entity with empty name to group"
    );
}

#[test]
fn add_entity_to_group_empty_entity_name() {
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.add_entity_to_group(" ", entity),
        "The given name is empty.",
        "Could add entity to group with empty name"
    );
}

#[test]
fn add_entity_to_group_check_sorting() {
    let (entity1, entity2, group) = ("Entity1", "Entity2", "Group");
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![entity1, entity2])
            .with_group(Group {
                name: group,
                entities: vec![entity2, entity1]
            }),
        {
            let entities = data
                .group(group)
                .expect("Could not get group")
                .entities_sorted();
            assert_eq!(entities[0], entity1, "Group members are not sorted");
            assert_eq!(entities[1], entity2, "Group members are not sorted");
        }
    );
}

#[test]
fn add_entity_to_group_not_enough_time_left() {
    let (entity, group) = ("Entity", "Group");
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_group(Group::default(group))
            .with_activity(Activity {
                groups: vec![group],
                duration: Time::new(1, 0),
                ..Default::default()
            }),
        data.add_entity_to_group(group, entity),
        "Entity will not have enough time if they take part in the activities of the group 'Group'.",
        "Could add entity with not enough time to group"
    );
}

// *** Remove entity from group ***
#[test]
fn simple_remove_entity() {
    let (entity1, entity2, group) = ("Entity1", "Entity2", "Group");
    test_ok!(
        data,
        DataBuilder::new()
            .with_entities(vec![entity1, entity2])
            .with_group(Group {
                name: group,
                entities: vec![entity1, entity2]
            }),
        {
            data.remove_entity_from_group(group, entity1)
                .expect("Could not remove entity from group");
            let entities = data
                .group(group)
                .expect("Could not get group by name")
                .entities_sorted();
            assert_eq!(entities.len(), 1, "Entity was not removed from group");
            assert_eq!(entities[0], entity2, "The wrong entity was removed");
        }
    );
}

#[test]
fn remove_entity_not_in_group() {
    let (entity, group) = ("Entity", "Group");
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity)
            .with_group(Group::default(group)),
        data.remove_entity_from_group(group, entity),
        "Entity is not in the group 'Group'.",
        "Could remove entity which is not a member of a group"
    );
}

#[test]
fn remove_nonexistent_entity_from_group() {
    let group = "Group";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(group)),
        data.remove_entity_from_group(group, "Does not exist"),
        "Does Not Exist does not exist.",
        "Could remove nonexistent entity from group"
    );
}

#[test]
fn remove_entity_from_nonexistent_group() {
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.remove_entity_from_group("Does not exist", entity),
        "The group 'Does Not Exist' does not exist.",
        "Could remove entity from nonexistent group"
    );
}

#[test]
fn remove_entity_empty_entity_name() {
    let group = "Group";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(group)),
        data.remove_entity_from_group(group, " "),
        "The given name is empty.",
        "Could remove entity with empty name from group"
    );
}

#[test]
fn remove_entity_empty_group_name() {
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.remove_entity_from_group(" ", entity),
        "The given name is empty.",
        "Could remove entity from group with empty name"
    );
}

#[test]
fn rename_group_name_taken_by_entity() {
    let (group_name, entity_name) = ("Group", "Entity");
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity_name)
            .with_group(Group::default(group_name)),
        data.set_group_name(group_name, entity_name),
        "The name 'Entity' is already taken by an entity.",
        "Could add group with name taken by an entity"
    );
}
