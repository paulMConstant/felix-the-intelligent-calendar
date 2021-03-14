use crate::{
    structs::{ActivityComputationStaticData, ActivityInsertionBeginningMinutes, InsertionCosts},
    MIN_TIME_DISCRETIZATION_MINUTES,
};

use std::collections::BTreeSet;

/// Given the static data and insertion data of all activities (parallel arrays)
/// as well as the index of the activity for which insertion times are computed,
/// computes the insertion times taking conflicts into account.
pub fn compute_insertion_costs(
    static_data: &[ActivityComputationStaticData],
    insertion_data: &[ActivityInsertionBeginningMinutes],
) -> Vec<Vec<InsertionCosts>> {
    // Preallocate data
    let mut scores_for_all_activities = Vec::with_capacity(static_data.len());

    for activity_static_data in static_data {
        let mut possible_beginnings = Vec::with_capacity(
            activity_static_data
                .possible_insertion_beginnings_minutes_sorted
                .len(),
        );

        // 1 - Fetch incompatible beginnings

        // Offset with the duration of the activity
        // (e.g. if 11:00 - 12:00 is taken and our duration is 00:30, we cannot insert the activity
        // at 10:50.
        let offset_check_before_activity =
            activity_static_data.duration_minutes - MIN_TIME_DISCRETIZATION_MINUTES;

        let mut incompatible_beginnings = BTreeSet::new();

        for (incompatible_beginning, incompatible_end) in activity_static_data
            .indexes_of_incompatible_activities
            .iter()
            .copied()
            .filter_map(|index| unsafe {
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

            incompatible_beginnings = incompatible_beginnings
                .union(
                    &activity_static_data
                        .possible_insertion_beginnings_minutes_sorted
                        .range(incompatible_beginning..incompatible_end)
                        .copied()
                        .collect(),
                )
                .copied()
                .collect();
        }

        // 2 - Calculate scores for the remaining activities
        for beginning in activity_static_data
            .possible_insertion_beginnings_minutes_sorted
            .iter()
            .copied()
            .filter(|beginning| !incompatible_beginnings.contains(beginning))
        {
            possible_beginnings.push(InsertionCosts {
                beginning_minutes: beginning,
                cost: 1, // TODO
            });
        }
        scores_for_all_activities.push(possible_beginnings);
    }
    scores_for_all_activities
}
