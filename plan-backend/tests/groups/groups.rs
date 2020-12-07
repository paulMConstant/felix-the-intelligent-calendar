//! Basic operations on groups.
//! Does not check interaction with entities or activities.
//!
//! Includes:
//! - Addition
//! - Deletion
//! - Getter
//! - Edition (name)

// TODO use data_builder for all files in groups directory
use plan_backend::data::Data;

// Test organization
// - Add group
// - Remove group
// - Get individual group
// - Add entity to group
// - Remove entity from group
// - Rename group

// *** Add group ***
#[test]
fn simple_add_group() {
    let mut data = Data::new();

    let name = data.add_group("Group").expect("Could not add group");
    let groups = data.groups_sorted();
    assert_eq!(groups.len(), 1, "Group was not added");
    assert_eq!(groups[0].name(), name, "Group was added with wrong name");
}

#[test]
fn add_group_check_sorting() {
    let mut data = Data::new();

    let name2 = data.add_group("Group2").expect("Could not add group");
    let name1 = data.add_group("Group1").expect("Could not add group");
    let groups = data.groups_sorted();

    assert_eq!(groups[0].name(), name1, "Groups are not sorted");
    assert_eq!(groups[1].name(), name2, "Groups are not sorted");
}

#[test]
fn add_group_check_formatting() {
    let mut data = Data::new();

    let name = data.add_group("group  1").expect("Could not add group");
    assert_eq!(name, "Group 1", "Group name was not formatted right");
}

#[test]
fn add_group_entity_has_same_name() {
    let mut data = Data::new();
    let entity_name = data.add_entity("Entity").expect("Could not add entity");
    assert_not_modified!(data, {
        assert_eq!(
            data.add_group(entity_name),
            Err("The name 'Entity' is already taken by an entity.".to_owned()),
            "Could add group with the same name as an entity"
        );
    });
}

#[test]
fn add_group_group_has_same_name() {
    let mut data = Data::new();

    let name = data.add_group("Group").expect("Could not add group");
    assert_not_modified!(data, {
        assert_eq!(
            data.add_group(name.clone()),
            Err("The group 'Group' already exists.".to_owned()),
            "Could add group with the same name as other group"
        );
    });
}

// *** Remove group ***
#[test]
fn simple_remove_group() {
    let mut data = Data::new();

    // Add two groups to check that the right one was removed
    let name1 = data.add_group("Group1").expect("Could not add group");
    let name2 = data.add_group("Group2").expect("Could not add group");
    data.remove_group(name1).expect("Could not remove group");

    let groups = data.groups_sorted();
    assert_eq!(groups.len(), 1, "Group was not removed");
    assert_eq!(groups[0].name(), name2, "The wrong group was removed");
}

#[test]
fn remove_group_name_empty() {
    let mut data = Data::new();

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_group(" "),
            Err("The given name is empty.".to_owned()),
            "Could remove group with empty name"
        );
    });
}

#[test]
fn remove_group_does_not_exist() {
    let mut data = Data::new();

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_group("Group"),
            Err("The group 'Group' does not exist.".to_owned()),
            "Could remove nonexistent group"
        );
    });
}

// *** Get individual group ***
#[test]
fn simple_get_group() {
    let mut data = Data::new();

    let name = data.add_group("Group").expect("Could not add group");
    let group = data.group(name.clone()).expect("Could not get group");
    assert_eq!(group.name(), name, "Group has wrong name");
}

#[test]
fn get_group_empty_name() {
    let data = Data::new();

    assert_eq!(
        data.group(" \t"),
        Err("The given name is empty.".to_owned()),
        "Could get group with empty name"
    );
}

#[test]
fn get_group_does_not_exist() {
    let data = Data::new();

    assert_eq!(
        data.group("Group"),
        Err("The group 'Group' does not exist.".to_owned()),
        "Could get group which does not exist"
    );
}

// *** Rename group ***
#[test]
fn simple_rename_group() {
    let mut data = Data::new();

    let group_name = data.add_group("Group name").expect("Could not add group");
    let new_name = data
        .set_group_name(group_name, "New Name")
        .expect("Could not rename group");

    assert_eq!(
        data.groups_sorted()[0].name(),
        new_name,
        "Group was not renamed"
    );
}

#[test]
fn rename_group_check_formatting() {
    let mut data = Data::new();

    let group_name = data.add_group("Group name").expect("Could not add group");
    let new_name = data
        .set_group_name(group_name, "new   NAMe")
        .expect("Could not rename group");

    assert_eq!(new_name, "New Name", "Name was not formatted right");
}

#[test]
fn rename_group_empty_names() {
    let mut data = Data::new();

    let group_name = data.add_group("Group name").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.set_group_name(group_name, ""),
            Err("The given name is empty.".to_owned()),
            "Could set name of group to empty"
        );

        assert_eq!(
            data.set_group_name("", "other name"),
            Err("The given name is empty.".to_owned()),
            "Could set name of group with empty name"
        );
    });
}

#[test]
fn rename_group_name_taken_by_group() {
    let mut data = Data::new();

    let group_name = data.add_group("Group name").expect("Could not add group");
    let other_name = data.add_group("Group name2").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.set_group_name(group_name, other_name),
            Err("The name 'Group Name2' is already taken by another group.".to_owned()),
            "Could add group with name taken by another group"
        );
    });
}

#[test]
fn rename_nonexistent_group() {
    let mut data = Data::new();

    assert_not_modified!(data, {
        assert_eq!(
            data.set_group_name("Does not exist", "Valid name"),
            Err("The group 'Does Not Exist' does not exist.".to_owned()),
            "Could set name of nonexistent group"
        );
    });
}
