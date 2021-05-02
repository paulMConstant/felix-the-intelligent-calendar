//! Operations on activities which depend on / interact with groups.
//!
//! Includes :
//! - Addition of groups to the activity
//! - Deletion of groups from the activity

use test_utils::{Activity, DataBuilder, Group};

// *** Add groups ***
#[test]
fn simple_add_group() {
    let group_name = "Group";
    test_ok!(
        data,
        DataBuilder::new()
            .with_activity(Activity::default())
            .with_group(Group::default(group_name)),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, group_name)
                .expect("Could not add group to activity");
            let groups = data.activity(id).groups_sorted();
            assert_eq!(groups.len(), 1, "Group was not added to the activity");
            assert_eq!(
                groups[0], group_name,
                "Groups was not added to the activity with the right name"
            );
        }
    );
}

#[test]
fn add_group_check_sorting() {
    let (group1, group2, group3) = ("Group1", "Group2", "Group3");
    test_ok!(
        data,
        DataBuilder::new()
            .with_activity(Activity::default())
            .with_groups(vec![
                Group {
                    name: group1,
                    ..Default::default()
                },
                Group {
                    name: group3,
                    ..Default::default()
                },
                Group {
                    name: group2,
                    ..Default::default()
                }
            ]),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, group1)
                .expect("Could not add group to activity");
            data.add_group_to_activity(id, group3)
                .expect("Could not add group to activity");
            data.add_group_to_activity(id, group2)
                .expect("Could not add group to activity");

            let groups = data.activity(id).groups_sorted();
            assert_eq!(groups.len(), 3, "Groups were not added to the activity");
            assert_eq!(groups[0], group1, "Groups are not sorted");
            assert_eq!(groups[1], group2, "Groups are not sorted");
            assert_eq!(groups[2], group3, "Groups are not sorted");
        }
    );
}

#[test]
fn add_group_empty_name() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, " ")
        },
        "The given name is empty.",
        "Could add group with empty name to activity"
    );
}

#[test]
fn add_group_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();

            data.add_group_to_activity(id, "group ")
        },
        "The group 'Group' does not exist.",
        "Could add nonexistent group to activity"
    );
}

#[test]
fn add_group_activity_does_not_exist() {
    let group_name = "Group";
    std::panic::catch_unwind(|| {
        let mut data = DataBuilder::new()
            .with_group(Group::default(group_name))
            .into_data();
        data.add_group_to_activity(123, group_name).unwrap()
    })
    .expect_err("Could add group to nonexistent activity");
}

#[test]
fn add_group_already_in_activity() {
    let group_name = "Group";
    test_err!(
        data,
        DataBuilder::new()
            .with_group(Group::default(group_name))
            .with_activity(Activity {
                groups: vec![group_name],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, group_name)
        },
        "The group 'Group' is already in the activity 'Activity'.",
        "Could add group to the same activity twice"
    );
}

#[test]
fn add_group_check_entities_added() {
    let group_name = "Group";
    let (entity1, entity2) = ("Entity1", "Entity2");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity1, entity2])
            .with_group(Group {
                name: group_name,
                entities: vec![entity1, entity2]
            })
            .with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, group_name)
                .expect("Could not add group to activity");
            let entities = data.activity(id).entities_sorted();
            assert_eq!(entities.len(), 2, "Entities were not added to the activity");
            assert_eq!(entities[0], entity1, "The entities were not added right");
            assert_eq!(entities[1], entity2, "The entities were not added right");
        }
    );
}

#[test]
fn add_group_check_entities_not_added_twice() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    let group_name = "Group";
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity1, entity2])
            .with_activity(Activity {
                entities: vec![entity1],
                ..Default::default()
            })
            .with_group(Group {
                name: group_name,
                entities: vec![entity1, entity2]
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, group_name)
                .expect("Could not add group to activity");
            let entities = data.activity(id).entities_sorted();
            assert_eq!(
                entities.len(),
                2,
                "Entities were not added to the activity or added twice"
            );
            assert_eq!(entities[0], entity1, "The entities were not added right");
            assert_eq!(entities[1], entity2, "The entities were not added right");
        }
    );
}

#[test]
fn add_group_check_entities_have_enough_time() {
    let group_name = "Group";
    let entity_name = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity_name)
            .with_group(Group {
                name: group_name,
                entities: vec![entity_name]
            })
            .with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.add_group_to_activity(id, group_name)
        },
        "Entity will not have enough time if they are added to 'Activity'.",
        "Could add group in which a participant does not have enough free time"
    );
}

// *** Remove groups ***
#[test]
fn simple_remove_group_from_activity() {
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

            data.remove_group_from_activity(id, group1)
                .expect("Could not remove group");
            let groups = data.activity(id).groups_sorted();
            assert_eq!(groups.len(), 1, "Group was not removed from the activity");
            assert_eq!(
                groups[0], group2,
                "The wrong group was removed from the activity"
            );
        }
    );
}

#[test]
fn remove_group_from_activity_group_empty_name() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_group_from_activity(id, " ")
        },
        "The given name is empty.",
        "Could remove group from activity with empty name"
    );
}

#[test]
fn remove_group_from_activity_group_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_group_from_activity(id, "Group")
        },
        "The group 'Group' does not exist.",
        "Could remove group from activity with empty name"
    );
}

#[test]
fn remove_group_from_activity_activity_does_not_exist() {
    let group_name = "Group";
    std::panic::catch_unwind(|| {
        let mut data = DataBuilder::new()
            .with_group(Group::default(group_name))
            .with_activity(Activity::default())
            .into_data();
        data.remove_group_from_activity(193, group_name).unwrap();
    })
    .expect_err("Could remove group from activity with empty name");
}

#[test]
fn remove_group_from_activity_group_not_in_activity() {
    let group_name = "Group";
    test_err!(
        data,
        DataBuilder::new()
            .with_group(Group::default(group_name))
            .with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_group_from_activity(id, group_name)
        },
        "The group 'Group' is not in the activity 'Activity'.",
        "Could remove group from activity with empty name"
    );
}

#[test]
fn remove_group_from_activity_check_entities_removed() {
    let group_name = "Group";
    let (entity_group, entity_independent) = ("Entity Group", "Entity Independent");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity_group, entity_independent])
            .with_group(Group {
                name: group_name,
                entities: vec![entity_group]
            })
            .with_activity(Activity {
                entities: vec![entity_independent],
                groups: vec![group_name],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_group_from_activity(id, group_name)
                .expect("Could not remove group from activity");

            let entities = data.activity(id).entities_sorted();
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
    );
}

#[test]
fn remove_group_from_activity_check_entities_in_other_groups_stay() {
    let (group1, group2) = ("Group1", "Group2");
    let (entity_one_group, entity_two_groups) = ("One Group", "Two Groups");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity_two_groups, entity_one_group])
            .with_groups(vec![
                Group {
                    name: group1,
                    entities: vec![entity_one_group, entity_two_groups]
                },
                Group {
                    name: group2,
                    entities: vec![entity_two_groups]
                }
            ])
            .with_activity(Activity {
                groups: vec![group1, group2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_group_from_activity(id, group1)
                .expect("Could not remove group from activity");

            let entities = data.activity(id).entities_sorted();
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
    );
}

#[test]
fn add_group_to_activity_check_insertion_costs_updated() {
    let entity_name = "Charles";
    let group_name = "Group";
    test_ok!(data,
             DataBuilder::new()
             .with_entity(entity_name)
             .with_group(Group { name: group_name, entities: vec![entity_name] })
             .with_work_interval_of_duration(1)
             .with_activity(Activity::default()),
     {
        let id = data.activities_sorted()[0].id();
        assert_eq!(data.activity(id).insertion_costs(), Some(Vec::new()));
        data.add_group_to_activity(id, group_name).expect("Could not add group to activity");

        // Added one participant => insertion costs updated
        data.wait_for_possible_insertion_costs_computation(id);
        assert!(data.activity(id).insertion_costs().expect("Insertion costs were not computed").len() > 1);
     });
}

#[test]
fn remove_group_from_activity_check_insertion_costs_updated() {
    let entity_name = "Charles";
    let group_name = "Group";
    test_ok!(data,
             DataBuilder::new()
             .with_entity(entity_name)
             .with_group(Group { name: group_name, entities: vec![entity_name] })
             .with_work_interval_of_duration(1)
             .with_activity(Activity {
                groups: vec![group_name],
                ..Default::default()
             }),
     {
        let id = data.activities_sorted()[0].id();
        assert!(data.activity(id).insertion_costs().expect("Insertion costs were not computed").len() > 1);
        data.remove_group_from_activity(id, group_name).expect("Could not remove group from activity");

        // Removed one participant => insertion costs updated
        data.wait_for_possible_insertion_costs_computation(id);
        assert_eq!(data.activity(id).insertion_costs(), Some(Vec::new()));
     });
}


