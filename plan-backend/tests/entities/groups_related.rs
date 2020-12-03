//! Operations on entities which depend on groups.
//! Includes :
//! - Addition
//! - Renaming
//! - Deletion

use plan_backend::data::Data;

#[test]
fn add_entity_group_has_same_name() {
    let mut data = Data::new();

    let name = data.add_group("name").expect("Could not add group");
    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity(name),
            Err("The name 'Name' is already taken by a group.".to_owned()),
            "Could add entity with a name taken by a group"
        );
    });
}

#[test]
fn rename_entity_group_already_exists() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let other_name = data.add_group("Other Name").expect("Coud not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.set_entity_name(name, other_name),
            Err("The name 'Other Name' is already taken by a group.".to_owned()),
            "Could rename with name taken by a group"
        );
    });
}

#[test]
fn rename_entity_check_renamed_in_group() {
    let mut data = Data::new();

    // Add two names to check that the right one was removed
    let name1 = data
        .add_entity("Name1")
        .expect("Could not add entity")
        .name();
    let name2 = data
        .add_entity("Name2")
        .expect("Could not add entity")
        .name();

    let group = data.add_group("Group").expect("Could not add group");
    data.add_entity_to_group(group.clone(), name1.clone())
        .expect("Could not add entity to group");
    data.add_entity_to_group(group.clone(), name2.clone())
        .expect("Could not add entity to group");

    let name3 = data
        .set_entity_name(name1, "Name3")
        .expect("Could not rename entity");
    let group_members = data
        .group(group)
        .expect("Could not find group")
        .entities_sorted();
    assert_eq!(
        group_members[0], &name2,
        "Entity was not renamed in group or names are not sorted"
    );
    assert_eq!(
        group_members[1], &name3,
        "Entity was not renamed in group or names are not sorted"
    );
}

#[test]
fn remove_entity_check_removed_in_group() {
    let mut data = Data::new();

    let entity1 = data.add_entity("Entity1").expect("Could not add entity").name();
    let entity2 = data.add_entity("Entity2").expect("Could not add entity").name();
    let group = data.add_group("Group").expect("Could not add group");
    
    data.add_entity_to_group(group.clone(), entity1.clone()).expect("Could not add entity to group");
    data.add_entity_to_group(group.clone(), entity2.clone()).expect("Could not add entity to group");
    data.remove_entity_from_group(group.clone(), entity1).expect("Could not remove entity from group");

    let entities = data.group(group).entities_sorted();
    assert_eq!(entities.len(), 1, "Entity was not removed from the group");
    assert_eq!(entities[0], entity2, "Entity was not removed from the group");
}
