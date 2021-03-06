use super::super::super::Entities;
use super::super::computation::activities_into_computation_data::activities_into_computation_data;
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
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
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
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
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
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
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
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
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
        .computation_data
        .incompatible_activity_ids();
    let incompatible_b = activity_collection
        .get_by_id(id_b)
        .computation_data
        .incompatible_activity_ids();
    let incompatible_c = activity_collection
        .get_by_id(id_c)
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
fn test_activities_into_computation_data() {
    let activity_collection = Activities::new();
    activity_collection.add("0".to_owned());
    activity_collection.add("1".to_owned());
    activity_collection.add("2".to_owned());
    activity_collection.add("3".to_owned());
    activity_collection.remove(2);

    // We will add one participant to each activity, if we don't, then the activity will be
    // filtered out
    let participant = "Participant".to_string();

    // Ids are [0, 1, 3]
    activity_collection.mutate_activity(0, |activity1| {
        activity1
            .computation_data
            .set_incompatible_activity_ids(vec![3]);
        activity1.computation_data.set_duration(Time::new(0, 30));
        activity1
            .metadata
            .add_entity(participant.clone())
            .expect("Could not add entity");
        *activity1.computation_data.insertion_costs().lock().unwrap() = Some(Vec::new());
    });

    let activity1 = activity_collection.get_by_id(0);

    activity_collection.mutate_activity(1, |activity2| {
        activity2
            .computation_data
            .set_incompatible_activity_ids(vec![0, 3]);
        activity2.computation_data.set_duration(Time::new(0, 20));
        activity2
            .metadata
            .add_entity(participant.clone())
            .expect("Could not add entity");
        *activity2.computation_data.insertion_costs().lock().unwrap() = Some(Vec::new());
    });
    let activity2 = activity_collection.get_by_id(1);

    activity_collection.mutate_activity(3, |activity3| {
        activity3
            .computation_data
            .set_incompatible_activity_ids(vec![1]);

        activity3.computation_data.set_duration(Time::new(0, 15));
        activity3.computation_data.insert(Some(Time::new(1, 0)));
        activity3
            .metadata
            .add_entity(participant.clone())
            .expect("Could not add entity");
        *activity3.computation_data.insertion_costs().lock().unwrap() = Some(Vec::new());
    });

    let activity3 = activity_collection.get_by_id(3);

    let (static_data, insertion_data) =
        activities_into_computation_data(&activity_collection.get_not_sorted());

    // Order should be Activity3 (inserted), activity2(harder to insert - 20 mins * 2 incompatible
    // activities), activity1

    let index_to_id_translation = index_to_id_map(&activity_collection.get_not_sorted());
    assert_eq!(index_to_id_translation[&2], 0);
    assert_eq!(index_to_id_translation[&1], 1);
    assert_eq!(index_to_id_translation[&0], 3);

    let activity3_static_data = &static_data[0];
    let activity2_static_data = &static_data[1];
    let activity1_static_data = &static_data[2];
    // Check ids
    // Activity3 - incompatible with activity 2 which is second
    assert_eq!(
        activity3_static_data.indexes_of_incompatible_activities,
        vec![1]
    );
    // Activity2 - incompatible with activities 3 and 1 which are last and first
    assert_eq!(
        activity2_static_data.indexes_of_incompatible_activities,
        vec![0, 2]
    );
    // Activity1 - incompatible with activity 3 which is first
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
            .insertion_costs()
            .expect("Insertion costs have not been computed")
            .iter()
            .map(|insertion_cost| insertion_cost.beginning.total_minutes())
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

/// The purpose of this test is to make sure that activities which are ordered differently in data
/// (by order of addition)
/// and in felix-computation-api (by difficulty of insertion) are not inverted.
#[test]
fn test_insertion_costs_of_activity() {
    let activity_collection = Activities::new();
    activity_collection.add("0".to_owned());
    activity_collection.add("1".to_owned());
    activity_collection.add("2".to_owned());

    let activity1_insertion_costs = (0..=10)
        .step_by(5)
        .map(|n_minutes| Time::from_total_minutes(n_minutes))
        .map(|beginning| InsertionCost { beginning, cost: 0 })
        .collect::<Vec<_>>();

    activity_collection.mutate_activity(0, |activity1| {
        activity1
            .computation_data
            .set_incompatible_activity_ids(vec![]);

        activity1.computation_data.set_duration(Time::new(0, 30));
        *activity1.computation_data.insertion_costs().lock().unwrap() =
            Some(activity1_insertion_costs.clone());
    });

    activity_collection.mutate_activity(1, |activity2| {
        activity2
            .computation_data
            .set_incompatible_activity_ids(vec![0, 3]);

        activity2.computation_data.set_duration(Time::new(0, 20));
        activity2.computation_data.insert(Some(Time::new(2, 0)));
    });

    activity_collection.mutate_activity(2, |activity3| {
        activity3.computation_data.set_duration(Time::new(0, 20));
        activity3.computation_data.insert(Some(Time::new(1, 0)));
    });

    // Activity 1 will be reordered internally.
    // Check that its beginnings are the ones we fetch (id != index)
    let result = activity_collection.get_by_id(0).insertion_costs();

    let expected = Some(activity1_insertion_costs);

    assert_eq!(result, expected);
}
