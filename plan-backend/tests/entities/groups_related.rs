//! Operations on entities which depend on groups.
//! Includes :
//! - Addition
//! - Renaming
//! - Deletion

use plan_test_utils::data_builder::{DataBuilder, Group};

#[test]
fn add_entity_group_has_same_name() {
    let name = "Name";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(name)),
        data.add_entity(name),
        "The name 'Name' is already taken by a group.",
        "Could add entity with a name taken by a group"
    );
}

#[test]
fn rename_entity_group_already_exists() {
    let (entity, group) = ("Entity", "Group");
    test_err!(
        data,
        DataBuilder::new()
            .with_group(Group::default(group))
            .with_entity(entity),
        data.set_entity_name(entity, group),
        "The name 'Group' is already taken by a group.",
        "Could rename with name taken by a group"
    );
}

#[test]
fn rename_entity_check_renamed_in_group() {
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
            let entity3 = data
                .set_entity_name(entity1, "Name3")
                .expect("Could not rename entity");
            let group_members = data
                .group(group)
                .expect("Could not find group")
                .entities_sorted();
            assert_eq!(
                group_members[0], entity2,
                "Entity was not renamed in group or names are not sorted"
            );
            assert_eq!(
                group_members[1], &entity3,
                "Entity was not renamed in group or names are not sorted"
            );
        }
    );
}

#[test]
fn remove_entity_check_removed_in_group() {
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
            data.remove_entity_from_group(group.clone(), entity1)
                .expect("Could not remove entity from group");

            let entities = data
                .group(group)
                .expect("Could not find group")
                .entities_sorted();
            assert_eq!(entities.len(), 1, "Entity was not removed from the group");
            assert_eq!(
                entities[0], entity2,
                "Entity was not removed from the group"
            );
        }
    );
}
