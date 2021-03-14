use crate::{
    structs::{ActivityComputationStaticData, ActivityInsertionBeginningMinutes},
    MIN_TIME_DISCRETIZATION_MINUTES,
};

use std::collections::BTreeSet;

/// Given the static data and insertion data of all activities (parallel arrays)
/// as well as the index of the activity for which insertion times are computed,
/// computes the insertion times taking conflicts into account.
///
/// # Safety
///
/// For performance reasons, no bounds checks are performed on index_of_activity_to_check.
pub fn filter_insertion_times_for_conflicts(
    static_data: &[ActivityComputationStaticData],
    insertion_data: &[ActivityInsertionBeginningMinutes],
    index_of_activity_to_check: usize,
) -> BTreeSet<u16> {
    unsafe {
        let activity_data = static_data.get_unchecked(index_of_activity_to_check);

        let mut possible_beginnings = activity_data
            .possible_insertion_beginnings_minutes_sorted
            .clone();

        // Offset with the duration of the activity
        // (e.g. if 11:00 - 12:00 is taken and our duration is 00:30, we cannot insert the activity
        // at 10:50.
        let offset_check_before_activity =
            activity_data.duration_minutes - MIN_TIME_DISCRETIZATION_MINUTES;

        for (incompatible_beginning, incompatible_end) in activity_data
            .indexes_of_incompatible_activities
            .iter()
            .copied()
            .filter_map(|index| {
                if let Some(incompatible_beginning) = insertion_data.get_unchecked(index) {
                    Some((
                        *incompatible_beginning,
                        incompatible_beginning + static_data.get_unchecked(index).duration_minutes,
                    ))
                } else {
                    None
                }
            })
        {
            let incompatible_beginning = if incompatible_beginning < offset_check_before_activity {
                0
            } else {
                incompatible_beginning - offset_check_before_activity
            };

            for value in incompatible_beginning..incompatible_end {
                possible_beginnings.remove(&value);
            }
        }
        possible_beginnings
    }
}
