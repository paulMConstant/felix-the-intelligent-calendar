use felix_computation_api::{
    find_possible_beginnings::{can_fit_in_schedule, compute_all_sums, find_possible_beginnings},
    structs::SumAndDurationIndexes,
};
use felix_datatypes::{ActivityBeginningsGivenDurationMinutes, WorkHourInMinutes};

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
    test_case_can_fit_in_schedule(&[30, 50], &[20, 40], true);
    test_case_can_fit_in_schedule(&[30, 39], &[20, 40], false);
    test_case_can_fit_in_schedule(&[30, 39, 50], &[20, 39, 40], true);
    test_case_can_fit_in_schedule(&[30, 39, 50], &[10, 20, 39, 40], true);
    test_case_can_fit_in_schedule(&[30, 39, 50], &[11, 20, 39, 40], false);
    // Way more time can be wasted than the work hour duration
    test_case_can_fit_in_schedule(&[240, 30], &[10, 40], true);
}

fn test_case_can_fit_in_schedule(
    work_hour_durations: &[u16],
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
    // Activity fits perfectly in the work hour
    let res = find_possible_beginnings(&[WorkHourInMinutes::new(10, 30)], &[20]);
    let expected = activity_beginnings_given_duration(&[20], &[&[10]]);
    assert_eq!(res, expected);

    // Two activities with same duration - tests symmetry
    let res = find_possible_beginnings(&[WorkHourInMinutes::new(10, 30)], &[10, 10]);
    let expected = activity_beginnings_given_duration(&[10], &[&[10, 20]]);
    assert_eq!(res, expected);

    // Activity bigger than a work hour
    // Work hours are sorted in ascending order.
    // Note that the sum of activity durations should always be less than that of the work hours.
    //     This should be tested in felix_backend::Data.
    let res = find_possible_beginnings(
        &[
            WorkHourInMinutes::new(200, 220),
            WorkHourInMinutes::new(300, 400),
        ],
        &[100],
    );
    let expected = activity_beginnings_given_duration(&[100], &[&[300]]);
    assert_eq!(res, expected);

    // Two different activity durations
    let res = find_possible_beginnings(
        &[
            WorkHourInMinutes::new(1300, 1400),
            WorkHourInMinutes::new(1000, 1200),
        ],
        &[50, 150],
    );
    let expected = activity_beginnings_given_duration(
        &[50, 150],
        &[
            // Possible beginnings for 50
            &[
                1300, 1305, 1310, 1315, 1320, 1325, 1330, 1335, 1340, 1345, 1350, 1000, 1150,
            ],
            // Possible beginnings for 150
            &[
                1000, 1005, 1010, 1015, 1020, 1025, 1030, 1035, 1040, 1045, 1050,
            ],
        ],
    );
    assert_eq!(res, expected);

    // Activity can fit nowhere
    let res = find_possible_beginnings(
        &[
            WorkHourInMinutes::new(300, 350),
            WorkHourInMinutes::new(100, 200),
        ],
        &[125],
    );
    let expected = activity_beginnings_given_duration(&[125], &[&[]]);
    assert_eq!(res, expected);

    // Result which used to be a problem - bug has been resolved since then, but keep it
    let res = find_possible_beginnings(&[WorkHourInMinutes::new(480, 700)], &[20, 35, 40, 45]);
    let expected =
        activity_beginnings_given_duration(&[40], &[&(480..=660).step_by(5).collect::<Vec<u16>>()]);
    assert_eq!(res[&40], expected[&40]);

    // Duplicate values
    let res = find_possible_beginnings(&[WorkHourInMinutes::new(480, 615)], &[25, 35, 35, 40]);
    let expected = activity_beginnings_given_duration(&[40], &[&[480, 505, 515, 540, 550, 575]]);
    assert_eq!(res[&40], expected[&40])
}

/// Given activity durations and possible beginnings for each duration (parallel slices),
/// create the corresponding ActivityBeginningsGivenDurationMinutes struct.
fn activity_beginnings_given_duration(
    activity_durations: &[u16],
    possible_beginnings: &[&[u16]],
) -> ActivityBeginningsGivenDurationMinutes {
    let mut res = ActivityBeginningsGivenDurationMinutes::new();
    for (index, duration) in activity_durations.iter().enumerate() {
        res.insert(*duration, hashset_from_slice(possible_beginnings[index]));
    }
    res
}

fn hashset_from_slice(slice: &[u16]) -> HashSet<u16> {
    slice.iter().map(|&i| i as u16).collect::<HashSet<_>>()
}
