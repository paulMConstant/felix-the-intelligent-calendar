//! Operations on entities which depend on activities.
//! Includes
//! - Renaming
//! - Removing

use test_utils::{Activity, DataBuilder};

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
fn remove_entity_check_remove_in_activity() {
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
