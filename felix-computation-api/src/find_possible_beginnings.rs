//! Algorithm to find the possible beginnings of activities given the activity durations and the
//! work hours.
//!
//! General overview:
//! 1 - Fetch the durations as u16 for faster calculations
//! 2 - Compute all possible duration sums (see tests)
//! 3 - For each activity, try to insert one activity in every slot
//! 4 - If the rest of the activities can be inserted in the remaining slots, then the time is
//!   valid.
//! 5 - The rest of the activities can be inserted in the remaining slots if their is a combination
//!   of duration sums which fit in the remaining slots.

use std::collections::{HashMap, HashSet};

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
pub type ActivityBeginnignsGivenDuration = HashMap<u16, HashSet<u16>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumAndDurationIndexes {
    pub sum_minutes: u16,
    pub indexes: HashSet<u16>,
}

impl SumAndDurationIndexes {
    pub fn new() -> SumAndDurationIndexes {
        SumAndDurationIndexes {
            sum_minutes: 0,
            indexes: HashSet::new(),
        }
    }
}

/// TODO doc
/// TODO make a struct {beginning, end}
pub fn find_possible_beginnings(
    work_hour_beginnings: Vec<u16>,
    work_hour_ends: Vec<u16>,
    work_hour_durations: Vec<u16>,
    mut activity_durations: Vec<u16>,
    minute_step: usize,
) -> ActivityBeginnignsGivenDuration {
    // Init result
    let mut activity_beginnings = ActivityBeginnignsGivenDuration::new();

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
            for mins_from_start in (0..last_time_we_need_to_check).step_by(minute_step) {
                let mut new_work_hour_durations = work_hour_durations.clone();
                // Reduce the duration of the work interval by the duration of the activity
                new_work_hour_durations[work_hour_index] -= activity_duration + mins_from_start;
                if mins_from_start != 0 {
                    // We have to put back the minutes we took above in a separate duration
                    // because we split the work hour in two
                    new_work_hour_durations.push(mins_from_start);
                }

                // Sort to use the biggest work hours first.
                // Sort ascending because we take the last element of the work hours each time.
                new_work_hour_durations.sort();

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
/// The sums are sorted decreasingly if the durations are sorted increasingly.
///
/// See the tests for examples.
///
/// # Panics
///
/// Panics if the combinatorial is too high.
/// Panics if the durations are not sorted in ascending order.
pub fn compute_all_sums(durations: &[u16]) -> Vec<SumAndDurationIndexes> {
    let pow_base: usize = 2;
    if let Some(set_size) = pow_base.checked_pow(durations.len() as u32) {
        let mut res = vec![SumAndDurationIndexes::new(); set_size];

        // Run counter from 000..0 to 111..1
        for counter in 0..set_size {
            for duration_index in 0..durations.len() {
                if counter & (1 << duration_index) == 0 {
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

/// Returns true if the given durations can fit in the given time intervals.
pub fn can_fit_in_schedule(
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

    // We instantly remove the last work interval.
    // As we try to fit the biggest possible duration combination into it,
    // no activity will fit in the remaining time.
    // The time which remains is wasted.
    let work_interval_duration = work_interval_durations
        .pop()
        .expect("Popping from empty work interval duration ! This case should be handled before");

    // Because the sums are sorted decreasingly, any sum that is shorter than this one will
    // waste too much time to continue.
    let min_acceptable_duration_sum = work_interval_duration - time_which_can_be_wasted;

    for (index, duration_sum) in all_duration_sums
        .iter()
        .filter(|duration_sum| duration_sum.sum_minutes <= work_interval_duration)
        .enumerate()
    {
        if duration_sum.sum_minutes < min_acceptable_duration_sum {
            // Early stop: the duration will waste too much time;
            // the next durations are too big for the remaining work hours.
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
        // Shortcut: we do not want to iterate over all durations if we know that they are greater
        // than the next work hour.
        let new_duration_sums = if let Some(&duration) = work_interval_durations.last() {
            if duration < duration_sum.sum_minutes {
                // We can start from here
                &all_duration_sums[index..]
            } else {
                // The next work hour may be enough for durations before the one we chose
                all_duration_sums
            }
        } else {
            all_duration_sums
        };
        if can_fit_in_schedule(
            n_activity_durations,
            new_duration_sums,
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
