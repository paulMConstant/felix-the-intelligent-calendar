use super::super::super::super::Entities;
use super::*;

use std::collections::BTreeSet;

#[test]
fn incompatible_ids() {
    let mut activity_collection = Activities::new();
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
    let mut activity_collection = Activities::new();
    activity_collection.add("0".to_owned());
    activity_collection.add("1".to_owned());
    activity_collection.add("2".to_owned());
    activity_collection.add("3".to_owned());
    activity_collection
        .remove(2)
        .expect("Could not remove activity");

    // Ids are [0, 1, 3]
    let activity1 = activity_collection
        .get_mut_by_id(0)
        .expect("Could not get activity by id");

    activity1
        .computation_data
        .set_incompatible_activity_ids(vec![3]);
    activity1.computation_data.set_duration(Time::new(0, 30));
    let activity1 = activity1.clone();

    let activity2 = activity_collection
        .get_mut_by_id(1)
        .expect("Could not get activity by id");

    activity2
        .computation_data
        .set_incompatible_activity_ids(vec![0, 3]);
    activity2.computation_data.set_duration(Time::new(0, 20));
    let activity2 = activity2.clone();

    let activity3 = activity_collection
        .get_mut_by_id(3)
        .expect("Could not get activity by id");

    activity3
        .computation_data
        .set_incompatible_activity_ids(vec![1]);
    activity3.computation_data.set_duration(Time::new(0, 15));
    activity3.computation_data.insert(Some(Time::new(1, 0)));
    let activity3 = activity3.clone();

    let (static_data, insertion_data) = activity_collection.fetch_computation();

    // Order should be Activity3 (inserted), activity2(harder to insert - 20 mins * 2 incompatible
    // activities), activity1

    let id_to_index_translation = &activity_collection.last_fetch_computation_id_to_index_map;
    assert_eq!(id_to_index_translation[&0], 2);
    assert_eq!(id_to_index_translation[&1], 1);
    assert_eq!(id_to_index_translation[&3], 0);

    let activity3_static_data = &static_data[0];
    let activity2_static_data = &static_data[1];
    let activity1_static_data = &static_data[2];
    // Check ids
    // Activity3
    assert_eq!(
        activity3_static_data.indexes_of_incompatible_activities,
        vec![1]
    );
    // Activity2
    assert_eq!(
        activity2_static_data.indexes_of_incompatible_activities,
        vec![0, 2]
    );
    // Activity1
    assert_eq!(
        activity1_static_data.indexes_of_incompatible_activities,
        vec![0]
    );

    // Test that the duration translation is right
    assert_eq!(
        activity1_static_data.duration_minutes,
        activity1.duration().total_minutes()
    );
    assert_eq!(
        activity2_static_data.duration_minutes,
        activity2.duration().total_minutes()
    );
    assert_eq!(
        activity3_static_data.duration_minutes,
        activity3.duration().total_minutes()
    );

    // Test that the possible insertions translation is right
    fn possible_insertion_times_if_no_conflict_minutes(
        activity: &Activity,
    ) -> BTreeSet<ActivityBeginningMinutes> {
        activity
            .computation_data
            .possible_insertion_times_if_no_conflict()
            .iter()
            .map(|time| time.total_minutes())
            .collect::<BTreeSet<_>>()
    }
    assert_eq!(
        possible_insertion_times_if_no_conflict_minutes(&activity1),
        activity1_static_data.possible_insertion_beginnings_minutes_sorted
    );
    assert_eq!(
        possible_insertion_times_if_no_conflict_minutes(&activity2),
        activity2_static_data.possible_insertion_beginnings_minutes_sorted
    );
    assert_eq!(
        possible_insertion_times_if_no_conflict_minutes(&activity3),
        activity3_static_data.possible_insertion_beginnings_minutes_sorted
    );

    // Test that insertion is right
    assert_eq!(
        insertion_data[0],
        activity3
            .insertion_interval()
            .unwrap()
            .beginning()
            .total_minutes()
    );
    assert_eq!(insertion_data.len(), 1);
}
