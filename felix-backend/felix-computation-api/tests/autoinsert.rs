use felix_computation_api::{autoinsert, structs::ActivityComputationStaticData};

use std::collections::BTreeSet;

/// Makes sure sending all inserted activities works
#[test]
fn test_autoinsert_everything_inserted() {
    let static_data = vec![ActivityComputationStaticData {
        possible_insertion_beginnings_minutes_sorted: (0..=0).collect(),
        indexes_of_incompatible_activities: vec![],
        duration_minutes: 20,
    }];
    let insertion_data = vec![0];
    let handle = autoinsert(&static_data, &insertion_data);
    assert_eq!(
        handle
            .get_result()
            .expect("No autoinsertion result where there should be one"),
        vec![0]
    );
}

/// Makes sure sending activities which can be inserted instantly works
/// (not enough data for init_nodes, need to expand once)
#[test]
fn test_autoinsert_instant_result_1() {
    let static_data = vec![ActivityComputationStaticData {
        possible_insertion_beginnings_minutes_sorted: (0..=0).collect(),
        indexes_of_incompatible_activities: vec![],
        duration_minutes: 20,
    }];
    let insertion_data = vec![];
    let handle = autoinsert(&static_data, &insertion_data);
    assert_eq!(
        handle
            .get_result()
            .expect("No autoinsertion result where there should be one"),
        vec![0]
    );
}

/// Makes sure sending activities which can be inserted instantly works
/// (not enough data for init_nodes, need to expand twice)
#[test]
fn test_autoinsert_instant_result_2() {
    let static_data = vec![
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 20,
        },
    ];
    let insertion_data = vec![];

    let handle = autoinsert(&static_data, &insertion_data);
    assert_eq!(
        handle
            .get_result()
            .expect("No autoinsertion result where there should be one"),
        vec![0, 10]
    );
}

#[test]
fn test_basic_autoinsert() {
    let static_data = vec![
        // 0
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=1000).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1, 2],
            duration_minutes: 10,
        },
        // 1
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 20,
        },
        // 2
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0, 3],
            duration_minutes: 10,
        },
        // 3
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..=200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![2],
            duration_minutes: 20,
        },
        // 4
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![5],
            duration_minutes: 10,
        },
        // 5
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![4],
            duration_minutes: 20,
        },
    ];
    let insertion_data = vec![0];

    let handle = autoinsert(&static_data, &insertion_data);
    let result = handle
        .get_result()
        .expect("No autoinsertion result where there should be one");
    assert_eq!(result[0], 0);
}

/// Makes sure autoinsertion fails early if not enough nodes to init and no solution
#[test]
fn test_autoinsert_instant_no_solution() {
    let static_data = vec![ActivityComputationStaticData {
        possible_insertion_beginnings_minutes_sorted: BTreeSet::new(),
        indexes_of_incompatible_activities: vec![],
        duration_minutes: 15,
    }];
    let insertion_data = vec![];

    let handle = autoinsert(&static_data, &insertion_data);
    assert!(handle.get_result().is_none());
}

#[test]
fn test_autoinsert_no_solution() {
    let static_data = vec![
        // This activity has many insertion spots - fire up many workers
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1],
            duration_minutes: 15,
        },
        // These activities are incompatible no matter what comes before them
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![2],
            duration_minutes: 15,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..=10).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1],
            duration_minutes: 20,
        },
    ];
    let insertion_data = vec![];

    let handle = autoinsert(&static_data, &insertion_data);
    assert!(handle.get_result().is_none());
}
