//! Operations on groups which depend on/interact with activities.
//!
//! Includes:
//! - Renaming a group
//! - Removing a group
//! - Adding entities to group
//! - Removing entities from a group

use test_utils::{Activity, DataBuilder, Group};

#[test]
fn rename_group_check_renamed_in_activities() {
    let (group1, group2) = ("Group1", "Group2");
    test_ok!(
        data,
        DataBuilder::new()
            .with_groups(vec![Group::default(group1), Group::default(group2)])
            .with_activity(Activity {
                groups: vec![group1, group2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
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
    );
}

#[test]
fn remove_group_check_removed_in_activities() {
    let (group1, group2) = ("Group1", "Group2");
    test_ok!(
        data,
        DataBuilder::new()
            .with_groups(vec![Group::default(group1), Group::default(group2)])
            .with_activity(Activity {
                groups: vec![group1, group2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
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
    );
}

#[test]
fn add_entity_to_group_check_added_to_activities() {
    let (group, entity) = ("Group", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_group(Group::default(group))
            .with_activity(Activity {
                groups: vec![group],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.add_entity_to_group(group, entity)
                .expect("Could not add entity to group");
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
            let expected = data.groups_sorted()[0].entities_sorted();
            assert_eq!(
                entities, expected,
                "Entity was not added to activity when it was added to a group"
            );
        }
    );
}

#[test]
fn add_entity_to_group_check_not_added_twice_in_activities() {
    let (group, entity) = ("Group", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_group(Group::default(group))
            .with_activity(Activity {
                groups: vec![group],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.add_entity_to_group(group, entity)
                .expect("Could not add entity to group");
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
            let expected = data.groups_sorted()[0].entities_sorted();
            assert_eq!(
                entities, expected,
                "Entity was added to activity again when its group was added"
            );
        }
    );
}

#[test]
fn add_entity_to_group_check_only_added_to_activity_with_group() {
    let (group, entity) = ("Group", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_group(Group::default(group))
            .with_activities(vec![Activity::default(), Activity::default()]),
        {
            let activities = data.activities_sorted();
            let id1 = activities[0].id();
            let id2 = activities[1].id();
            data.add_group_to_activity(id1, group)
                .expect("Could not add group to activity");
            data.add_entity_to_group(group, entity)
                .expect("Could not add entity to group");

            let entities1 = data
                .activity(id1)
                .expect("Could not get activity by id")
                .entities_sorted();
            let expected = data.groups_sorted()[0].entities_sorted();
            assert_eq!(
                entities1, expected,
                "Entity was not added to activity when its group was added"
            );

            let entities2 = data
                .activity(id2)
                .expect("Could not get activity by id")
                .entities_sorted();
            assert!(entities2.is_empty(), "Entity was added to activity when its group was added even though the activity does not contain the group");
        }
    );
}

#[test]
fn remove_entity_from_group_check_removed_in_activities() {
    let (group, entity) = ("Group", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_group(Group {
                name: group,
                entities: vec![entity]
            })
            .with_activity(Activity {
                groups: vec![group],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_group(group, entity)
                .expect("Could not remove entity from group");
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
            assert!(
                entities.is_empty(),
                "Entity was not removed from activity when removed from group"
            );
        }
    );
}

#[test]
fn remove_entity_from_group_check_stays_in_activity_if_in_other_groups() {
    let (group1, group2, entity) = ("Group1", "Group2", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_groups(vec![
                Group {
                    name: group1,
                    entities: vec![entity]
                },
                Group {
                    name: group2,
                    entities: vec![entity]
                }
            ])
            .with_activity(Activity {
                groups: vec![group1, group2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_group(group1, entity)
                .expect("Could not remove entity from group");
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
            let expected = data
                .group(group2)
                .expect("Could not get group by name")
                .entities_sorted();

            assert_eq!(entities, expected, "Entity was removed from activity even though it is participating through another group");
        }
    );
}

#[test]
fn remove_entity_from_group_check_stays_in_activity_if_not_in_any_group() {
    let (group, entity) = ("Group", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_group(Group {
                name: group,
                entities: vec![entity]
            })
            .with_activity(Activity {
                entities: vec![entity],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_group(group, entity)
                .expect("Could not remove entity from group");
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
            let expected = vec![entity];

            assert_eq!(entities, expected, "Entity was removed from activity even though it is not participating throug the group in which it was removed");
        }
    );
}

#[test]
fn remove_entity_from_group_check_not_removed_in_activity_where_excluded_from_group() {
    // If the user has added the group to the activity then removed the entity, the entity is
    // excluded from the group
    let (group, entity) = ("Group", "Entity");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_group(Group {
                name: group,
                entities: vec![entity]
            })
            .with_activity(Activity {
                groups: vec![group],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_activity(id, entity)
                .expect("Could not remove entity from activity");
            data.remove_group_from_activity(id, group).expect("May have thrown because an entity of the group is not in the activity anymore. \
                                                              This behaviour is standard and should not throw");
        }
    );
}
