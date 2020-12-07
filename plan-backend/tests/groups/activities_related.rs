//! Operations on groups which depend on/interact with activities.
//!
//! Includes:
//! - Renaming a group
//! - Removing a group
//! - Adding entities to group
//! - Removing entities from a group

use plan_backend::data::Data;

#[test]
fn rename_group_check_renamed_in_activities() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Add two groups to check that the right one was renamed
    let group1 = data.add_group("Group1").expect("Could not add group");
    let group2 = data.add_group("Group2").expect("Could not add group");

    data.add_group_to_activity(id, group1.clone())
        .expect("Could not add group");
    data.add_group_to_activity(id, group2.clone())
        .expect("Could not add group");
    let group3 = data
        .set_group_name(group1, "Group3")
        .expect("Could not set group name");

    let groups = data
        .activity(id)
        .expect("Could not get activity by id")
        .groups_sorted();
    assert_eq!(groups.len(), 2, "Groups were not added to the activity");
    assert_eq!(groups[0], group2, "Group was not renamed right in activity");
    assert_eq!(groups[1], group3, "Group was not renamed right in activity");
}

#[test]
fn remove_group_check_removed_in_activities() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Add two groups to check that the right one was removed
    let group1 = data.add_group("Group1").expect("Could not add group");
    let group2 = data.add_group("Group2").expect("Could not add group");

    data.add_group_to_activity(id, group1.clone())
        .expect("Could not add group");
    data.add_group_to_activity(id, group2.clone())
        .expect("Could not add group");

    data.remove_group(group1).expect("Could not remove group");

    let groups = data
        .activity(id)
        .expect("Could not get activity by id")
        .groups_sorted();
    assert_eq!(groups.len(), 1, "Group was not removed from the activity");
    assert_eq!(
        groups[0], group2,
        "The wrong group was removed from the activity"
    );
}

#[test]
fn add_entity_to_group_check_added_to_activities() {
    //let mut data = Data::new();

    //let id = data.add_activity("Activity").expect("Could not add activity").id();
    //let group = data.add_group("Group").expect("Could not add group");
    //let entity = data.add_entity("Entity").expect("Could not add entity");
}

#[test]
fn add_entity_check_not_added_twice_in_activities() {}

#[test]
fn remove_entity_check_removed_in_activities() {}

#[test]
fn remove_entity_check_stays_in_activity_if_in_other_groups() {}
