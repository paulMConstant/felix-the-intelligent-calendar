//! Operations on entities which depend on activities.
//! Includes
//! - Renaming
//! - Removing
//! - Custom work hours on inserted activities

use felix_test_utils::{Activity, DataBuilder};
use felix_backend::data::TimeInterval;
use felix_backend::Time;

#[test]
fn rename_entity_check_renamed_in_activity() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity1, entity2])
            .with_activity(Activity {
                entities: vec![entity1, entity2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            let entity3 = data
                .set_entity_name(entity1, "Entity3")
                .expect("Could not rename entity");
            let entities = data.activity(id).entities_sorted();

            assert_eq!(
                entities[0], entity2,
                "The wrong entity was renamed or they are not sorted"
            );
            assert_eq!(
                entities[1], entity3,
                "The entity was not renamed or they are not sorted"
            );
        }
    );
}

#[test]
fn remove_entity_check_removed_in_activity() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity1, entity2])
            .with_activity(Activity {
                entities: vec![entity1, entity2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity(entity1)
                .expect("Could not remove entity");
            let entities = data.activity(id).entities_sorted();
            assert_eq!(
                entities.len(),
                1,
                "Entity was not removed from the activity"
            );
            assert_eq!(entities[0], entity2, "The wrong entity was removed");
        }
    );
}

#[test]
fn remove_entity_check_activity_insertion_costs_updated() {
    let entity = "Jean";
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity)
            .with_activity(Activity {
                entities: vec![entity],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            assert!(data.activity(id).insertion_costs().expect("Insertion costs were not computed").len() > 1);
            data.remove_entity(entity).expect("Could not remove entity");

            data.wait_for_possible_insertion_costs_computation(id);
            assert_eq!(data.activity(id).insertion_costs(), Some(Vec::new()));
        }
    );
}

/// Tests that custom work hours cannot be added while activities are inserted
#[test]
fn add_custom_work_hours_with_inserted_activities() {
    let entity = "Jean";
    let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_entity(entity)
            .with_activity(Activity {
                entities: vec![entity],
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            data.add_custom_work_interval_for(entity, custom_work_interval)
        },
        "Work hours cannot be modified while an activity is inserted.",
        "Could add custom work hours with one inserted activity"
    );
}

/// Tests that custom work hours cannot be removed while activities are inserted
#[test]
fn remove_custom_work_hours_with_inserted_activities() {
    let entity = "Jean";
    let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_entity(entity)
            .with_custom_work_interval_for(entity, custom_work_interval)
            .with_activity(Activity {
                entities: vec![entity],
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            data.remove_custom_work_interval_for(entity, custom_work_interval)
        },
        "Work hours cannot be modified while an activity is inserted.",
        "Could remove custom work hours with one inserted activity"
    );
}

/// Tests that custom work hours cannot be updated while activities are inserted
#[test]
fn update_custom_work_hours_with_inserted_activities() {
    let entity = "Jean";
    let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(TimeInterval::new(Time::new(8, 0), Time::new(12, 0)))
            .with_entity(entity)
            .with_custom_work_interval_for(entity, custom_work_interval)
            .with_activity(Activity {
                entities: vec![entity],
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            let new_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
            data.update_custom_work_interval_for(entity, custom_work_interval, new_work_interval)
        },
        "Work hours cannot be modified while an activity is inserted.",
        "Could update custom work hours with one inserted activity"
    );
}
