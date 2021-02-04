use felix_computation_api::find_possible_beginnings::{
    can_fit_in_schedule, compute_all_sums, SumAndDurationIndexes,
};

use std::collections::HashSet;

#[test]
fn test_compute_all_sums() {
    let durations = &[30, 80];
    let expected = vec![
        SumAndDurationIndexes {
            sum_minutes: 110,
            indexes: [0, 1].iter().map(|&i| i as u16).collect::<HashSet<_>>(),
        },
        SumAndDurationIndexes {
            sum_minutes: 80,
            indexes: [1].iter().map(|&i| i as u16).collect::<HashSet<_>>(),
        },
        SumAndDurationIndexes {
            sum_minutes: 30,
            indexes: [0].iter().map(|&i| i as u16).collect::<HashSet<_>>(),
        },
        SumAndDurationIndexes {
            sum_minutes: 0,
            indexes: HashSet::new(),
        },
    ];

    assert_eq!(compute_all_sums(durations), expected);
}

#[test]
fn test_can_fit_in_schedule() {
    // ARRAYS SORTED ASCENDING
    test_case_can_fit_in_schedule(vec![30, 50], &[20, 40], true);
    test_case_can_fit_in_schedule(vec![30, 39], &[20, 40], false);
    test_case_can_fit_in_schedule(vec![30, 39, 50], &[20, 39, 40], true);
    test_case_can_fit_in_schedule(vec![30, 39, 50], &[10, 20, 39, 40], true);
    test_case_can_fit_in_schedule(vec![30, 39, 50], &[11, 20, 39, 40], false);
}

fn test_case_can_fit_in_schedule(
    work_hour_durations: Vec<u16>,
    activity_durations: &[u16],
    expected: bool,
) {
    let all_activity_sums = compute_all_sums(activity_durations);
    let time_which_can_be_wasted =
        work_hour_durations.iter().sum::<u16>() - activity_durations.iter().sum::<u16>();
    assert_eq!(
        can_fit_in_schedule(
            activity_durations.len(),
            &all_activity_sums,
            work_hour_durations,
            time_which_can_be_wasted,
            HashSet::new()
        ),
        expected
    );
}

#[test]
fn test_find_possible_beginnings() {
    // TODO
}
