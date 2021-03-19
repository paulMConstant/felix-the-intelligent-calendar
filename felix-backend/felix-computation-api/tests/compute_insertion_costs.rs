use felix_computation_api::{
    compute_insertion_costs, compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::ActivityComputationStaticData,
};

use std::collections::BTreeSet;

#[test]
fn test_filter_conflicts() {
    let static_data = vec![
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[
                0, 5, 10, 20, 35, 45, 50,
            ]),
            indexes_of_incompatible_activities: vec![1, 2],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 15,
        },
    ];

    const INSERTION_BEGINNING_MINUTES1: u16 = 5;
    const INSERTION_BEGINNING_MINUTES2: u16 = 30;
    let insertion_data = vec![
        None,
        Some(INSERTION_BEGINNING_MINUTES1),
        Some(INSERTION_BEGINNING_MINUTES2),
    ];

    let expected = btreeset_from_slice(&[20, 45, 50]);
    assert_eq!(
        get_activity_beginnings_with_conflicts(&static_data, &insertion_data)[0],
        expected
    );
}

#[test]
fn test_insertion_costs_simplest() {
    let static_data = vec![
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[
                0, 5, 10, 15, 20, 35, 45, 50,
            ]),
            indexes_of_incompatible_activities: vec![1],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[0, 5, 10, 20]),
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 15,
        },
    ];

    let insertion_data = vec![None, None];

    let insertion_costs = compute_insertion_costs(&static_data, &insertion_data)
        .iter()
        .map(|vec_insertion_cost| {
            vec_insertion_cost
                .iter()
                .map(|insertion_cost| insertion_cost.cost)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let insertion_costs0 = &insertion_costs[0];
    assert_eq!(insertion_costs0[0], 1000); // 0 Blocks 0, 5
                                          // -> 2 blocked * 1 incompatible activities / 2 remaining spots
    assert_eq!(insertion_costs0[1], 3000); // 5 Blocks 0, 5, 10
                                          // -> 3 blocked * 1 incompatible activities / 1 remaining spots
    assert_eq!(insertion_costs0[2], 3000); // 10 Blocks 0, 5, 10
    assert_eq!(insertion_costs0[3], 3000); // 15 Blocks 5, 10 20
    assert_eq!(insertion_costs0[4], 1000); // 20 Blocks 10, 20
    assert_eq!(insertion_costs0[5], 0); // 35 Blocks nothing
    assert_eq!(insertion_costs0[6], 0); // 45 Blocks nothing
    assert_eq!(insertion_costs0[7], 0); // 50 Blocks nothing

    let insertion_costs1 = &insertion_costs[1];
    assert_eq!(insertion_costs1[0], 600); // 0 Blocks 0, 5, 10
                                         // -> 3 blocked * 1 incompatible activities / 5 remaining spots
    assert_eq!(insertion_costs1[1], 1000); // 5 Blocks 0, 5, 10, 15
                                          // -> 4 blocked * 1 incompatible activities / 4 remaining spots
    assert_eq!(insertion_costs1[2], 1000); // 10 Blocks 5, 10, 15, 20
    assert_eq!(insertion_costs1[3], 333); // 15 Blocks 10, 20
                                         // -> 2 blocked * 1 incompatible activities / 6 remaining spots
}

fn btreeset_from_slice(slice: &[u16]) -> BTreeSet<u16> {
    slice.iter().map(|&i| i as u16).collect::<BTreeSet<_>>()
}
