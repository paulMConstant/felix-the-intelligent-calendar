#[macro_use]
extern crate plan_test_utils;

use plan_test_utils::data_builder::{DataBuilder, Activity, Group};
use plan_backend::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION};

#[test]
fn add_entity() {
    let name = "Name";
    test_ok!(data, DataBuilder::new().with_entity(name), {
        let entities = data.entities_sorted();
    assert_eq!(entities.len(), 1, "Entity was not added");
    assert_eq!(entities[0].name(), name, "Entity was not added with the right name");
               });
}

#[test]
fn add_entities() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    test_ok!(data, DataBuilder::new().with_entities(vec![entity1, entity2]), {
        let entities = data.entities_sorted();
    assert_eq!(entities.len(), 2, "Entity was not added");
    assert_eq!(entities[0].name(), entity1, "Entity was not added with the right name or entities are not sorted");
    assert_eq!(entities[1].name(), entity2, "Entity was not added with the right name or entities are not sorted");
    });
}

#[test]
fn add_custom_interval_for() {
    let entity = "Entity";
    let interval = TimeInterval::new(Time::new(1, 0), Time::new(5, 0));
    test_ok!(data, DataBuilder::new().with_entity(entity).with_custom_work_interval_for(entity, interval), {
        let entity = data.entities_sorted()[0];
        let custom_work_hours = entity.custom_work_hours();
        assert_eq!(custom_work_hours.len(), 1, "Custom work hour was not added");
        assert_eq!(custom_work_hours[0], interval, "The wrong interval was added");
    });
}

#[test]
fn add_custom_intervals_for() {
    let entity = "Entity";
    let interval1 = TimeInterval::new(Time::new(1, 0), Time::new(5, 0));
    let interval2 = TimeInterval::new(Time::new(7, 0), Time::new(10, 0));
    test_ok!(data, DataBuilder::new().with_entity(entity).with_custom_work_intervals_for(entity, vec![interval1, interval2]), {
        let entity = data.entities_sorted()[0];
        let custom_work_hours = entity.custom_work_hours();
        assert_eq!(custom_work_hours.len(), 2, "Custom work hours were not added");
        assert_eq!(custom_work_hours[0], interval1, "The wrong interval was added");
        assert_eq!(custom_work_hours[1], interval2, "The wrong interval was added");
    });
}

#[test]
fn add_group() {
    let entity1 = "Entity1";
    let entity2 = "Entity2";
    let group_name = "Group";
    test_ok!(data, DataBuilder::new().with_entities(vec![entity1, entity2]).with_group( Group { name: group_name, entities: vec![entity1, entity2] }), {
        let groups = data.groups_sorted();
        assert_eq!(groups.len(), 1, "Group was not added");
        assert_eq!(groups[0].name(), group_name, "Group name is not right");
        assert_eq!(groups[0].entities_sorted(), vec![entity1, entity2], "Entities were not added to the group");
    });
}

#[test]
fn add_default_group() {
    let group_name = "Group";
    test_ok!(data, DataBuilder::new().with_group(Group::default(group_name)), {
        let group = data.groups_sorted()[0];
        assert_eq!(group.name(), group_name, "Default group name is wrong");
        assert!(group.entities_sorted().is_empty(), "Default group members is wrong");
    });
}

#[test]
fn add_groups() {
    let (group1, group2) = ("Group1", "Group2");
    test_ok!(data, DataBuilder::new().with_groups(vec![Group::default(group1), Group::default(group2)]), {
        let groups = data.groups_sorted();
        assert_eq!(groups.len(), 2, "Groups were not added");
        assert_eq!(groups[0].name(), group1, "Group was not added correctly");
        assert_eq!(groups[1].name(), group2, "Group was not added correctly");
        assert!(groups[0].entities_sorted().is_empty(), "Default group members is wrong");
        assert!(groups[1].entities_sorted().is_empty(), "Default group members is wrong");
    });
}

#[test]
fn add_work_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_ok!(data, DataBuilder::new().with_work_interval(interval), {
        let intervals = data.work_hours();
        assert_eq!(intervals.len(), 1, "Interval was not added");
        assert_eq!(intervals[0], interval, "Interval was not added right");
    });
}

#[test]
fn add_work_interval_with_duration() {
    test_ok!(data, DataBuilder::new().with_work_interval_of_duration(4), {
        let expected = TimeInterval::new(Time::new(0, 0), Time::new(4, 0));
        assert_eq!(data.work_hours()[0], expected);
    });
}

#[test]
fn add_work_intervals() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(14, 0));
    test_ok!(data, DataBuilder::new().with_work_intervals(vec![interval1, interval2]), {
        let intervals = data.work_hours();
        assert_eq!(intervals.len(), 2, "Intervals were not added");
        assert_eq!(intervals[0], interval1, "Interval was not added right");
        assert_eq!(intervals[1], interval2, "Interval was not added right");
    });
}

#[test]
fn add_activity() {
    let activity_name = "Activity Name";
    let duration = Time::new(5, 30);
    let (entity1, entity2) = ("Entity1", "Entity2");
    let (group1, group2) = ("Group1", "Group2");
    test_ok!(data, DataBuilder::new().with_work_interval_of_duration(6)
             .with_entities(vec![entity1, entity2])
             .with_groups(vec![Group::default(group1), Group::default(group2)])
             .with_activity( Activity { name: activity_name, duration, entities: vec![entity2, entity1], groups: vec![group2, group1]}), {
                 let activities = data.activities_sorted();
                 assert_eq!(activities.len(), 1, "Activity was not added");
                 let activity = activities[0];
                 assert_eq!(activity.name(), activity_name, "Activity name is wrong");
                 assert_eq!(activity.duration(), duration, "Activity name is wrong");
                 assert_eq!(activity.entities_sorted(), vec![entity1, entity2], "Activity members is wrong");
                 assert_eq!(activity.groups_sorted(), vec![group1, group2], "Activity groups is wrong");
             });
}

#[test]
fn add_default_activity() {
    test_ok!(data, DataBuilder::new().with_activity(Activity::default()), {
        let activity = data.activities_sorted()[0];
        assert_eq!(activity.name(), "Activity", "Default activity name is wrong");
        assert_eq!(activity.duration(), MIN_TIME_DISCRETIZATION, "Default activity duration is wrong");
        assert!(activity.entities_sorted().is_empty(), "Default activity members is wrong");
        assert!(activity.groups_sorted().is_empty(), "Default activity groups is wrong");
    });
}

#[test]
fn add_activities() {
    test_ok!(data, DataBuilder::new().with_activities(vec![Activity::default(), Activity::default()]), {
        let activities = data.activities_sorted();
        assert_eq!(activities.len(), 2, "Activities were not added");
        let default_activity = Activity::default();
        let expected_name = default_activity.name;
        let expected_duration = default_activity.duration;
        let expected_entities = default_activity.entities;
        let expected_groups = default_activity.groups;

        for i in 0..1 {
            assert_eq!(activities[i].name(), expected_name, "Activities were not added right");
            assert_eq!(activities[i].duration(), expected_duration, "Activities were not added right");
            assert_eq!(activities[i].entities_sorted(), expected_entities, "Activities were not added right");
            assert_eq!(activities[i].groups_sorted(), expected_groups, "Activities were not added right");
        }
    });
}
