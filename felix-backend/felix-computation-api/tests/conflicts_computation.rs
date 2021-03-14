use felix_computation_api::{
    filter_insertion_times_for_conflicts::filter_insertion_times_for_conflicts,
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
        filter_insertion_times_for_conflicts(&static_data, &insertion_data, 0),
        expected
    );
}

fn btreeset_from_slice(slice: &[u16]) -> BTreeSet<u16> {
    slice.iter().map(|&i| i as u16).collect::<BTreeSet<_>>()
}
