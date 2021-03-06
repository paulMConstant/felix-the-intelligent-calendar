use felix_computation_api::{
    compute_insertion_costs, compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::ActivityComputationStaticData,
};
use felix_datatypes::InsertionCostsMinutes;

use std::collections::BTreeSet;

#[test]
fn test_filter_conflicts() {
    let static_data = vec![
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![2],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![2],
            duration_minutes: 15,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[
                0, 5, 10, 20, 35, 45, 50,
            ]),
            indexes_of_incompatible_activities: vec![0, 1],
            duration_minutes: 10,
        },
    ];

    const INSERTION_BEGINNING_MINUTES1: u16 = 5;
    const INSERTION_BEGINNING_MINUTES2: u16 = 30;
    let insertion_data = vec![INSERTION_BEGINNING_MINUTES1, INSERTION_BEGINNING_MINUTES2];

    let expected = btreeset_from_slice(&[20, 45, 50]);
    assert_eq!(
        get_activity_beginnings_with_conflicts(&static_data, &insertion_data, 2),
        expected
    );
}

#[test]
fn test_filter_conflicts_real_life() {
    // 4 activities, 08:00 - 10:15
    let static_data = vec![
        // 0
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![3, 1, 2],
            duration_minutes: 35,
        },
        // 1
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![3, 0, 2],
            duration_minutes: 35,
        },
        // 2
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![3, 0, 1],
            duration_minutes: 25,
        },
        // 3
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (480..615)
                .step_by(5)
                .map(|i| i as u16)
                .collect::<BTreeSet<_>>(),
            indexes_of_incompatible_activities: vec![0, 1, 2],
            duration_minutes: 40,
        },
    ];

    let insertion_data = vec![580, 480, 555];

    let insertion_costs = compute_insertion_costs(&static_data, &insertion_data, 3);
    assert_eq!(
        insertion_costs,
        vec![InsertionCostsMinutes {
            beginning_minutes: 515,
            cost: 0,
        }]
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

    let insertion_data = Vec::new();

    let insertion_costs0 = compute_insertion_costs(&static_data, &insertion_data, 0)
        .iter()
        .map(|insertion_cost| insertion_cost.cost)
        .collect::<Vec<_>>();

    assert_eq!(insertion_costs0[0], 10000); // 0 Blocks 0, 5
                                            // -> 2 blocked * 1 incompatible activities
    assert_eq!(insertion_costs0[1], 30000); // 5 Blocks 0, 5, 10
                                            // -> 3 blocked * 1 incompatible activities
    assert_eq!(insertion_costs0[2], 30000); // 10 Blocks 0, 5, 10
    assert_eq!(insertion_costs0[3], 30000); // 15 Blocks 5, 10 20
    assert_eq!(insertion_costs0[4], 10000); // 20 Blocks 10, 20
    assert_eq!(insertion_costs0[5], 0); // 35 Blocks nothing
    assert_eq!(insertion_costs0[6], 0); // 45 Blocks nothing
    assert_eq!(insertion_costs0[7], 0); // 50 Blocks nothing

    let insertion_costs1 = compute_insertion_costs(&static_data, &insertion_data, 1)
        .iter()
        .map(|insertion_cost| insertion_cost.cost)
        .collect::<Vec<_>>();

    assert_eq!(insertion_costs1[0], 6000); // 0 Blocks 0, 5, 10
                                           // -> 3 blocked * 1 incompatible activities
    assert_eq!(insertion_costs1[1], 10000); // 5 Blocks 0, 5, 10, 15
                                            // -> 4 blocked * 1 incompatible activities
    assert_eq!(insertion_costs1[2], 10000); // 10 Blocks 5, 10, 15, 20
    assert_eq!(insertion_costs1[3], 3333); // 15 Blocks 10, 20
                                           // -> 2 blocked * 1 incompatible activities
}

fn btreeset_from_slice(slice: &[u16]) -> BTreeSet<u16> {
    slice.iter().map(|&i| i as u16).collect::<BTreeSet<_>>()
}
