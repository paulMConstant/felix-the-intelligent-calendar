//! Basic operations on groups.
//! Does not check interaction with entities or activities.
//!
//! Includes:
//! - Addition
//! - Deletion
//! - Getter
//! - Edition (name)

use plan_test_utils::data_builder::{DataBuilder, Group};

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
    test_ok!(data, DataBuilder::new(), {
        let name = data.add_group("Group").expect("Could not add group");
        let groups = data.groups_sorted();
        assert_eq!(groups.len(), 1, "Group was not added");
        assert_eq!(groups[0].name(), name, "Group was added with wrong name");
    });
}

#[test]
fn add_group_check_sorting() {
    let (name1, name2) = ("Group1", "Group2");
    test_ok!(
        data,
        DataBuilder::new().with_groups(vec![Group::default(name1), Group::default(name2)]),
        {
            let groups = data.groups_sorted();

            assert_eq!(groups[0].name(), name1, "Groups are not sorted");
            assert_eq!(groups[1].name(), name2, "Groups are not sorted");
        }
    );
}

#[test]
fn add_group_check_formatting() {
    test_ok!(data, DataBuilder::new(), {
        let name = data.add_group("group  1").expect("Could not add group");
        assert_eq!(name, "Group 1", "Group name was not formatted right");
    });
}

#[test]
fn add_group_entity_has_same_name() {
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(entity),
        data.add_group(entity),
        "The name 'Entity' is already taken by an entity.",
        "Could add group with the same name as an entity"
    );
}

#[test]
fn add_group_group_has_same_name() {
    let group = "Group";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(group)),
        data.add_group(group),
        "The group 'Group' already exists.",
        "Could add group with the same name as other group"
    );
}

// *** Remove group ***
#[test]
fn simple_remove_group() {
    // Add two groups to check that the right one was removed
    let (name1, name2) = ("Group1", "Group2");
    test_ok!(
        data,
        DataBuilder::new().with_groups(vec![Group::default(name1), Group::default(name2)]),
        {
            data.remove_group(name1).expect("Could not remove group");

            let groups = data.groups_sorted();
            assert_eq!(groups.len(), 1, "Group was not removed");
            assert_eq!(groups[0].name(), name2, "The wrong group was removed");
        }
    );
}

#[test]
fn remove_group_name_empty() {
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_group(" "),
        "The given name is empty.",
        "Could remove group with empty name"
    );
}

#[test]
fn remove_group_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_group("Group"),
        "The group 'Group' does not exist.",
        "Could remove nonexistent group"
    );
}

// *** Get individual group ***
#[test]
fn simple_get_group() {
    let group_name = "Group";
    test_ok!(
        data,
        DataBuilder::new().with_group(Group::default(group_name)),
        {
            let group = data.group(group_name).expect("Could not get group");
            assert_eq!(group.name(), group_name, "Group has wrong name");
        }
    );
}

#[test]
fn get_group_empty_name() {
    test_err!(
        data,
        DataBuilder::new(),
        data.group(" \t"),
        "The given name is empty.",
        "Could get group with empty name"
    );
}

#[test]
fn get_group_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new(),
        data.group("Group"),
        "The group 'Group' does not exist.",
        "Could get group which does not exist"
    );
}

// *** Rename group ***
#[test]
fn simple_rename_group() {
    let group_name = "Group Name";
    let new_name = "New Name";
    test_ok!(
        data,
        DataBuilder::new().with_group(Group::default(group_name)),
        {
            data.set_group_name(group_name, new_name)
                .expect("Could not rename group");

            assert_eq!(
                data.groups_sorted()[0].name(),
                new_name,
                "Group was not renamed"
            );
        }
    );
}

#[test]
fn rename_group_check_formatting() {
    let group_name = "Group Name";
    test_ok!(
        data,
        DataBuilder::new().with_group(Group::default(group_name)),
        {
            let new_name = data
                .set_group_name(group_name, "new   NAMe")
                .expect("Could not rename group");
            assert_eq!(new_name, "New Name", "Name was not formatted right");
        }
    );
}

#[test]
fn rename_group_empty_new_name() {
    let group_name = "Group Name";
    test_err!(
        data,
        DataBuilder::new().with_group(Group::default(group_name)),
        data.set_group_name(group_name, ""),
        "The given name is empty.",
        "Could set name of group to empty"
    );
}

#[test]
fn rename_group_empty_old_name() {
    test_err!(
        data,
        DataBuilder::new(),
        data.set_group_name("", "other name"),
        "The given name is empty.",
        "Could set name of group with empty name"
    );
}

#[test]
fn rename_group_name_taken_by_group() {
    let group_name = "Group Name";
    let other_name = "Other Name";
    test_err!(
        data,
        DataBuilder::new()
            .with_groups(vec![Group::default(group_name), Group::default(other_name)]),
        data.set_group_name(group_name, other_name),
        "The name 'Other Name' is already taken by another group.",
        "Could add group with name taken by another group"
    );
}

#[test]
fn rename_nonexistent_group() {
    test_err!(
        data,
        DataBuilder::new(),
        data.set_group_name("Does not exist", "Valid name"),
        "The group 'Does Not Exist' does not exist.",
        "Could set name of nonexistent group"
    );
}
