use super::super::super::super::Entities;
use super::*;

use std::collections::BTreeSet;

#[test]
fn incompatible_ids() {
    let mut activity_collection = Activities::new(Rc::new(
        rayon::ThreadPoolBuilder::new()
            .build()
            .expect("Could not build rayon::ThreadPool"),
    ));
    let id_a = activity_collection.add("a".to_owned()).id();
    let id_b = activity_collection.add("b".to_owned()).id();

    let mut entities = Entities::new();
    let entity_a = "A".to_owned();
    let entity_b = "B".to_owned();
    entities
        .add(entity_a.clone())
        .expect("Could not add entity");
    entities
        .add(entity_b.clone())
        .expect("Could not add entity");

    // Insert the same entity in both activities
    activity_collection
        .add_entity(id_a, entity_a.clone())
        .expect("Could not add entity to activity");
    activity_collection
        .add_entity(id_b, entity_a.clone())
        .expect("Could not add entity to activity");

    // At this point : id_a contains {a}, id_b contains {a}
    let incompatible_a = activity_collection
        .get_by_id(id_a)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    assert_eq!(incompatible_a.len(), 1);
    assert_eq!(incompatible_b.len(), 1);
    assert_eq!(incompatible_a[0], id_b);
    assert_eq!(incompatible_b[0], id_a);

    // Remove the entity in one activity
    activity_collection
        .remove_entity(id_a, &entity_a)
        .expect("Could not remove entity from activity");

    // At this point : id_a contains {}, id_b contains {a}
    let incompatible_a = activity_collection
        .get_by_id(id_a)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    assert_eq!(incompatible_a.len(), 0);
    assert_eq!(incompatible_b.len(), 0);

    // Add non-confictual entity
    activity_collection
        .add_entity(id_a, entity_b.clone())
        .expect("Could not add entity to activity");

    // At this point : id_a contains {b}, id_b contains {a}
    let incompatible_a = activity_collection
        .get_by_id(id_a)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    assert_eq!(incompatible_a.len(), 0);
    assert_eq!(incompatible_b.len(), 0);

    // Add conflictual entity again
    activity_collection
        .add_entity(id_b, entity_b)
        .expect("Could not add entity to activity");

    // At this point : id_a contains {b}, id_b contains {a, b}
    let incompatible_a = activity_collection
        .get_by_id(id_a)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    assert_eq!(incompatible_a.len(), 1);
    assert_eq!(incompatible_b.len(), 1);
    assert_eq!(incompatible_a[0], id_b);
    assert_eq!(incompatible_b[0], id_a);

    // Add third activity
    let id_c = activity_collection.add("c".to_owned()).id();
    activity_collection
        .add_entity(id_c, entity_a)
        .expect("Could not add entity to activity");

    // At this point : id_a contains {b}, id_b contains {a, b}, id_c contains {a}
    let incompatible_a = activity_collection
        .get_by_id(id_a)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    let incompatible_c = activity_collection
        .get_by_id(id_c)
        .expect("Could not get activity by id")
        .computation_data
        .incompatible_activity_ids();
    assert_eq!(incompatible_a.len(), 1);
    assert_eq!(incompatible_a[0], id_b);

    assert_eq!(incompatible_b.len(), 2);
    assert!(incompatible_b.contains(&id_a));
    assert!(incompatible_b.contains(&id_c));

    assert_eq!(incompatible_c.len(), 1);
    assert!(incompatible_c.contains(&id_b));
}

#[test]
fn test_fetch_computation() {
    let mut activity_collection = Activities::new(Rc::new(
        rayon::ThreadPoolBuilder::new()
            .build()
            .expect("Could not build rayon::ThreadPool"),
    ));
    activity_collection.add("0".to_owned());
    activity_collection.add("1".to_owned());
    activity_collection.add("2".to_owned());
    activity_collection.add("3".to_owned());
    activity_collection
        .remove(2)
        .expect("Could not remove activity");

    // Ids are [0, 1, 3]
    activity_collection
        .get_mut_by_id(0)
        .expect("Could not get activity by id")
        .computation_data
        .set_incompatible_activity_ids(vec![3]);
    activity_collection
        .get_mut_by_id(1)
        .expect("Could not get activity by id")
        .computation_data
        .set_incompatible_activity_ids(vec![0, 3]);
    activity_collection
        .get_mut_by_id(3)
        .expect("Could not get activity by id")
        .computation_data
        .set_incompatible_activity_ids(vec![1]);

    let activities: Vec<Activity> = activity_collection.activities.values().cloned().collect();
    let (static_data, insertion_data) = activity_collection.fetch_computation();

    for (activity, static_data) in activities.iter().zip(static_data) {
        // Test that the id => index translation is right
        // Assuming activities.values() returns the same order twice
        // (activities.values() called in fetch_computation)
        let mut ids = activity.computation_data.incompatible_activity_ids();
        let mut ids_from_indexes = static_data
            .indexes_of_incompatible_activities
            .iter()
            .map(|&index| activities[index].id())
            .collect::<Vec<ActivityId>>();
        ids.sort();
        ids_from_indexes.sort();
        assert_eq!(ids, ids_from_indexes);

        // Test that the duration translation is right
        assert_eq!(activity.duration().total_minutes(), static_data.duration_minutes);

        // Test that the possible insertions translation is right
        assert_eq!(activity
                   .computation_data
                   .possible_insertion_times_if_no_conflict()
                   .iter()
                   .map(|time| time.total_minutes())
                   .collect::<BTreeSet<_>>(),
                   static_data.possible_insertion_beginnings_minutes_sorted);
    }

    for (activity, insertion_data) in activities.iter().zip(insertion_data) {
        // Test that insertion is right
        assert_eq!(activity.insertion_interval()
                   .map(|interval| interval.beginning().total_minutes()),
                   insertion_data);
    }
}
