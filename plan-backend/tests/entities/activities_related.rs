//! Operations on entities which depend on activities.
//! Includes
//! - Renaming
//! - Removing

use plan_test_utils::data_builder::{Activity, DataBuilder};

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
            let entities = data
                .activity(id)
                .expect("Could not get activity")
                .entities_sorted();

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
            let entities = data
                .activity(id)
                .expect("Could not get activity")
                .entities_sorted();
            assert_eq!(
                entities.len(),
                1,
                "Entity was not removed from the activity"
            );
            assert_eq!(entities[0], entity2, "The wrong entity was removed");
        }
    );
}
