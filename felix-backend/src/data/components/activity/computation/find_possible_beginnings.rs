use crate::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION_MINUTES};

use std::collections::{HashMap, HashSet};

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
type ActivityBeginnignsGivenDuration = HashMap<u16, HashSet<u16>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct SumAndDurationIndexes {
    sum_minutes: u16,
    indexes: HashSet<u16>,
}

impl SumAndDurationIndexes {
    pub fn new() -> SumAndDurationIndexes {
        SumAndDurationIndexes {
            sum_minutes: 0,
            indexes: HashSet::new(),
        }
    }
}

/// Given the activities of an entity, computes the possible beginnings for a set duration.
///
/// This is a pre-computation: it takes into account entities separately.
fn find_possible_beginnings(
    work_hours: &[TimeInterval],
    activity_durations: &[Time],
) -> ActivityBeginnignsGivenDuration {
    // Init result
    let mut activity_beginnings = ActivityBeginnignsGivenDuration::new();

    // Turn time structs into minutes
    let mut activity_durations = activity_durations
        .iter()
        .map(|time| time.total_minutes())
        .collect::<Vec<_>>();

    let work_hour_beginnings = work_hours
        .iter()
        .map(|&time_interval| time_interval.beginning().total_minutes())
        .collect::<Vec<_>>();

    let work_hour_ends = work_hours
        .iter()
        .map(|&time_interval| time_interval.end().total_minutes())
        .collect::<Vec<_>>();

    let work_hour_durations = work_hours
        .iter()
        .map(|&time_interval| time_interval.duration().total_minutes())
        .collect::<Vec<_>>();

    let n_activity_durations = activity_durations.len();

    // 1 - Compute all possible sums of activity durations (see tests)
    // Activity durations need to be sorted so that compute_all_sums output is sorted
    activity_durations.sort();
    let all_duration_sums = compute_all_sums(&activity_durations);
    let time_which_can_be_wasted =
        work_hour_durations.iter().sum::<u16>() - activity_durations.iter().sum::<u16>();

    // 2 - Try to put every different duration in every possible starting time and check if the
    //   rest of the durations can be put in the rest of the work hours.
    //   If it is possible, then the starting time is added to the result.

    let mut activity_durations_checked = HashSet::new();

    for duration_index in 0..activity_durations.len() {
        // If the computation has already been done for one duration, skip it
        let activity_duration = activity_durations[duration_index];
        if activity_durations_checked.contains(&activity_duration) {
            continue;
        }
        activity_durations_checked.insert(activity_duration);
        let mut possible_beginnings = HashSet::new();

        for work_hour_index in 0..work_hour_durations.len() {
            let work_hour_duration = work_hour_durations[work_hour_index];

            // Check only the first half of the work hour because of symmetry
            let last_time_we_need_to_check = work_hour_duration / 2;

            // Iterate over each possible starting time in the work hour
            for mins_from_start in
                (0..last_time_we_need_to_check).step_by(MIN_TIME_DISCRETIZATION_MINUTES as usize)
            {
                let mut new_work_hour_durations = work_hour_durations.clone();
                // Reduce the duration of the work interval by the duration of the activity
                new_work_hour_durations[work_hour_index] -= activity_duration + mins_from_start;
                if mins_from_start != 0 {
                    // We have to put back the minutes we took above in a separate duration
                    // because we split the work hour in two
                    new_work_hour_durations.push(mins_from_start);
                }

                // Sort to use the biggest work hours first.
                // Sort decreasingly because we take the last element of the work hours each time.
                new_work_hour_durations.sort_by(|a, b| b.cmp(a));

                // Check if the rest of the activities fit in the schedule.
                if can_fit_in_schedule(
                    n_activity_durations,
                    &all_duration_sums,
                    new_work_hour_durations,
                    time_which_can_be_wasted,
                    [duration_index]
                        .iter()
                        .map(|&i| i as u16)
                        .collect::<HashSet<_>>(),
                ) {
                    // The rest of the activities fit in the schedule.
                    // This insertion time is valid for the given duration.
                    possible_beginnings
                        .insert(work_hour_beginnings[work_hour_index] + mins_from_start);
                    // Add the symmetry
                    possible_beginnings.insert(work_hour_ends[work_hour_index] - mins_from_start);
                }
            }
        }
        activity_beginnings.insert(activity_duration, possible_beginnings);
    }
    activity_beginnings
}

/// Given an array of durations, computes all possible sums using every combination.
/// The sums are sorted in ascending order assuming the durations are sorted.
///
/// See the tests for examples.
///
/// # Panics
///
/// Panics if the combinatorial is too high.
/// Panics if the durations are not sorted in ascending order.
fn compute_all_sums(durations: &[u16]) -> Vec<SumAndDurationIndexes> {
    assert!(is_sorted(&durations));
    let pow_base: usize = 2;
    if let Some(set_size) = pow_base.checked_pow(durations.len() as u32) {
        let mut res = vec![SumAndDurationIndexes::new(); set_size];

        // Run coutner from 000..0 to 111..1
        for counter in 0..set_size {
            for duration_index in 0..durations.len() {
                if counter & (1 << duration_index) > 0 {
                    // The index was included in the counter. Add it to the result.
                    res[counter].indexes.insert(duration_index as u16);
                    res[counter].sum_minutes += durations[duration_index];
                }
            }
        }
        res
    } else {
        panic!("Overflow : too many activities !");
    }
}

/// Checks that a slice is sorted (not strictly) by ascending order.
fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

/// Returns true if the given durations can fit in the given time intervals.
fn can_fit_in_schedule(
    n_activity_durations: usize,
    all_duration_sums: &[SumAndDurationIndexes],
    mut work_interval_durations: Vec<u16>,
    time_which_can_be_wasted: u16,
    used_indexes: HashSet<u16>,
) -> bool {
    if used_indexes.len() == n_activity_durations {
        // We have inserted all activities
        return true;
    }
    if work_interval_durations.is_empty() {
        // Not all activities have been inserted yet we have run out of work intervals.
        return false;
    }

    let work_interval_duration = work_interval_durations
        .pop()
        .expect("Popping from empty work interval duration ! This case should be handled before");
    // Because the sums are sorted decreasingly, any sum that is shorter than this one will
    // waste too much time to continue.
    let min_acceptable_duration_sum = work_interval_duration - time_which_can_be_wasted;

    for duration_sum in all_duration_sums
        .iter()
        .filter(|duration_sum| duration_sum.sum_minutes >= min_acceptable_duration_sum)
    {
        if duration_sum.sum_minutes > work_interval_duration {
            // Early stop: the duration is too big to fit in this work interval.
            // As durations are sorted by ascending order and intervals by descending order,
            // the duration will not fit in any interval.
            return false;
        }
        if duration_sum
            .indexes
            .intersection(&used_indexes)
            .next()
            .is_some()
        {
            // The sum does not fit in the work hour or one duration of the sum has already
            // been used before.
            continue;
        }

        let new_used_indexes = used_indexes.union(&duration_sum.indexes).copied().collect();
        let new_time_which_can_be_wasted =
            time_which_can_be_wasted - (work_interval_duration - duration_sum.sum_minutes);
        if can_fit_in_schedule(
            n_activity_durations,
            all_duration_sums,
            work_interval_durations.clone(),
            new_time_which_can_be_wasted,
            new_used_indexes,
        ) {
            // Yay !
            return true;
        }
    }
    // At this point, we did not fit every duration in the interval.
    false
}

// TODO tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&[1, 2, 2, 4]));
        assert_eq!(is_sorted(&[1, 2, 2, 4, 3]), false);
    }

    #[test]
    fn test_compute_all_sums() {
        let durations = vec![30, 80];
        let expected = vec![
            SumAndDurationIndexes {
                sum_minutes: 0,
                indexes: HashSet::new(),
            },
            SumAndDurationIndexes {
                sum_minutes: 30,
                indexes: [0].iter().map(|&i| i as usize).collect::<HashSet<_>>(),
            },
            SumAndDurationIndexes {
                sum_minutes: 80,
                indexes: [1].iter().map(|&i| i as usize).collect::<HashSet<_>>(),
            },
            SumAndDurationIndexes {
                sum_minutes: 110,
                indexes: [0, 1].iter().map(|&i| i as usize).collect::<HashSet<_>>(),
            },
        ];

        assert_eq!(compute_all_sums(&durations), expected);
    }
}
