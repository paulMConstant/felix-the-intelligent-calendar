//! Operations on activities which depend on / interact with groups.
//!
//! Includes :
//! - Addition of groups to the activity
//! - Deletion of groups from the activity

use plan_backend::data::{Data, Time, TimeInterval};

// *** Add groups ***
#[test]
fn simple_add_group() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    let group = data.add_group("Group").expect("Could not add group");

    data.add_group_to_activity(id, group.clone())
        .expect("Could not add group to activity");
    let groups = data
        .activity(id)
        .expect("Could not get activity by id")
        .groups_sorted();
    assert_eq!(groups.len(), 1, "Group was not added to the activity");
    assert_eq!(
        groups[0], group,
        "Groups was not added to the activity with the right name"
    );
}

#[test]
fn add_group_check_sorting() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    let group1 = data.add_group("Group1").expect("Could not add group");
    let group2 = data.add_group("Group2").expect("Could not add group");
    let group3 = data.add_group("Group3").expect("Could not add group");

    data.add_group_to_activity(id, group1.clone())
        .expect("Could not add group to activity");
    data.add_group_to_activity(id, group3.clone())
        .expect("Could not add group to activity");
    data.add_group_to_activity(id, group2.clone())
        .expect("Could not add group to activity");

    let groups = data
        .activity(id)
        .expect("Could not get activity by id")
        .groups_sorted();
    assert_eq!(groups.len(), 3, "Groups were not added to the activity");
    assert_eq!(groups[0], group1, "Groups are not sorted");
    assert_eq!(groups[1], group2, "Groups are not sorted");
    assert_eq!(groups[2], group3, "Groups are not sorted");
}

#[test]
fn add_group_check_formatting() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    let group1 = data.add_group("Group1").expect("Could not add group");

    data.add_group_to_activity(id, "group1 ")
        .expect("Could not add group to activity");

    let groups = data
        .activity(id)
        .expect("Could not get activity by id")
        .groups_sorted();
    assert_eq!(
        groups[0], group1,
        "Formatting was not done when adding group"
    );
}

#[test]
fn add_group_empty_name() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    assert_not_modified!(data, {
        assert_eq!(
            data.add_group_to_activity(id, " "),
            Err("The given name is empty.".to_owned()),
            "Could add group with empty name to activity"
        );
    });
}

#[test]
fn add_group_does_not_exist() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    assert_not_modified!(data, {
        assert_eq!(
            data.add_group_to_activity(id, "group "),
            Err("The group 'Group' does not exist.".to_owned()),
            "Could add nonexistent group to activity"
        );
    });
}

#[test]
fn add_group_activity_does_not_exist() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");

    assert_not_modified!(data, {
        assert_eq!(
            data.add_group_to_activity(0, group_name),
            Err("Cannot get activity with id 0.".to_owned()),
            "Could add group to nonexistent activity"
        );
    });
}

#[test]
fn add_group_already_in_activity() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    data.add_group_to_activity(id, group_name.clone())
        .expect("Could not add group to activity");
    assert_not_modified!(data, {
        assert_eq!(
            data.add_group_to_activity(id, group_name),
            Err("The group 'Group' is already in the activity 'Activity'.".to_owned()),
            "Could add group to the same activity twice"
        );
    });
}

#[test]
fn add_group_check_entities_added() {
    let mut data = Data::new();

    let group = data.add_group("Group").expect("Could not add group");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Add work hours so that entities have enough time
    let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(morning_shift)
        .expect("Could not add work interval");

    // Check that two entities are added
    let entity1 = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();
    let entity2 = data
        .add_entity("Entity2")
        .expect("Could not add entity")
        .name();

    data.add_entity_to_group(group.clone(), entity1.clone())
        .expect("Could not add entity to group");
    data.add_entity_to_group(group.clone(), entity2.clone())
        .expect("Could not add entity to group");

    data.add_group_to_activity(id, group)
        .expect("Could not add group to activity");
    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();
    assert_eq!(entities.len(), 2, "Entities were not added to the activity");
    assert_eq!(entities[0], entity1, "The entities were not added right");
    assert_eq!(entities[1], entity2, "The entities were not added right");
}

#[test]
fn add_group_check_entities_not_added_twice() {
    let mut data = Data::new();

    let group = data.add_group("Group").expect("Could not add group");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Add work hours so that entities have enough time
    let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(morning_shift)
        .expect("Could not add work interval");

    // We will check that two entities are added
    let entity1 = data
        .add_entity("Entity1")
        .expect("Could not add entity")
        .name();
    let entity2 = data
        .add_entity("Entity2")
        .expect("Could not add entity")
        .name();

    data.add_entity_to_group(group.clone(), entity1.clone())
        .expect("Could not add entity to group");
    data.add_entity_to_group(group.clone(), entity2.clone())
        .expect("Could not add entity to group");

    // Add entity to activity first, then add group.
    data.add_entity_to_activity(id, entity1.clone())
        .expect("Could not add entity to activity");
    data.add_group_to_activity(id, group)
        .expect("Could not add group to activity");
    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();
    assert_eq!(
        entities.len(),
        2,
        "Entities were not added to the activity or added twice"
    );
    assert_eq!(entities[0], entity1, "The entities were not added right");
    assert_eq!(entities[1], entity2, "The entities were not added right");
}

#[test]
fn add_group_check_entities_have_enough_time() {
    let mut data = Data::new();

    let group = data.add_group("Group").expect("Could not add group");
    let entity = data
        .add_entity("Entity")
        .expect("Could not add entity")
        .name();
    data.add_entity_to_group(group.clone(), entity.clone())
        .expect("Could not add entity to group");

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    assert_not_modified!(data, {
        assert_eq!(
            data.add_group_to_activity(id, group),
            Err("'Entity' does not have enough time left for this activity.".to_owned()),
            "Could add group in which a participant does not have enough free time"
        );
    });
}

// *** Remove groups ***
#[test]
fn simple_remove_group_from_activity() {
    let mut data = Data::new();

    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Add two groups and check that the right one is removed
    let group1 = data.add_group("Group1").expect("Could not add group");
    let group2 = data.add_group("Group2").expect("Could not add group");
    data.add_group_to_activity(id, group1.clone())
        .expect("Could not add group to activity");
    data.add_group_to_activity(id, group2.clone())
        .expect("Could not add group to activity");

    data.remove_group_from_activity(id, group1)
        .expect("Could not remove group");
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
fn remove_group_from_activity_group_empty_name() {
    let mut data = Data::new();
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_group_from_activity(id, " "),
            Err("The given name is empty.".to_owned()),
            "Could remove group from activity with empty name"
        );
    });
}

#[test]
fn remove_group_from_activity_group_does_not_exist() {
    let mut data = Data::new();
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_group_from_activity(id, " group"),
            Err("The group 'Group' does not exist.".to_owned()),
            "Could remove group from activity with empty name"
        );
    });
}

#[test]
fn remove_group_from_activity_activity_does_not_exist() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_group_from_activity(0, group_name),
            Err("Cannot get activity with id 0.".to_owned()),
            "Could remove group from activity with empty name"
        );
    });
}

#[test]
fn remove_group_from_activity_group_not_in_activity() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_group_from_activity(id, group_name),
            Err("The group 'Group' is not in the activity 'Activity'.".to_owned()),
            "Could remove group from activity with empty name"
        );
    });
}

#[test]
fn remove_group_from_activity_check_entities_removed() {
    let mut data = Data::new();

    let group_name = data.add_group("Group").expect("Could not add group");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Make sure every entity has enough time
    data.add_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
        .expect("Could not add interval");

    // Add two entities : one participates through the group, the other independently
    let entity_group = data
        .add_entity("Group entity")
        .expect("Could not add entity")
        .name();
    let entity_independent = data
        .add_entity("Independent entity")
        .expect("Could not add entity")
        .name();

    data.add_entity_to_group(&group_name, entity_group)
        .expect("Could not add entity to group");
    data.add_entity_to_activity(id, entity_independent.clone())
        .expect("Could not add entity to activity");

    data.add_group_to_activity(id, group_name.clone())
        .expect("Could not add group to activity");
    data.remove_group_from_activity(id, &group_name)
        .expect("Could not remove group from activity");

    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();
    assert_eq!(
        entities.len(),
        1,
        "The entity was not removed from the activity when its group was removed"
    );
    assert_eq!(
        entities[0], entity_independent,
        "The wrong entity was removed"
    );
}

#[test]
fn remove_group_from_activity_check_entities_in_other_groups_stay() {
    let mut data = Data::new();

    let group1 = data.add_group("Group1").expect("Could not add group");
    let group2 = data.add_group("Group2").expect("Could not add group");
    let id = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();

    // Make sure every entity has enough time
    data.add_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
        .expect("Could not add interval");

    // Add two entities : one participates through the group, the other through two groups
    let entity_group = data
        .add_entity("One Group entity")
        .expect("Could not add entity")
        .name();
    let entity_two_groups = data
        .add_entity("Two groups entity")
        .expect("Could not add entity")
        .name();

    data.add_entity_to_group(&group1, entity_group)
        .expect("Could not add entity to group");

    data.add_entity_to_group(&group1, entity_two_groups.clone())
        .expect("Could not add entity to group");
    data.add_entity_to_group(&group2, entity_two_groups.clone())
        .expect("Could not add entity to activity");

    data.add_group_to_activity(id, group1.clone())
        .expect("Could not add group to activity");
    data.add_group_to_activity(id, group2.clone())
        .expect("Could not add group to activity");
    data.remove_group_from_activity(id, &group1)
        .expect("Could not remove group from activity");

    let entities = data
        .activity(id)
        .expect("Could not get activity by id")
        .entities_sorted();
    assert_eq!(
        entities.len(),
        1,
        "The entity was not removed from the activity when its group was removed"
    );
    assert_eq!(
        entities[0], entity_two_groups,
        "The wrong entity was removed"
    );
}
