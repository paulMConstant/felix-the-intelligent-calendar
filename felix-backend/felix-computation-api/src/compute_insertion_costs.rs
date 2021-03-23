use crate::{
    structs::{
        ActivityBeginningMinutes, ActivityComputationStaticData, ActivityInsertionBeginningMinutes,
        InsertionCostsMinutes,
    },
    MIN_TIME_DISCRETIZATION_MINUTES,
};

use std::collections::BTreeSet;

/// Given the static data and insertion data of all activities (parallel arrays)
/// as well as the index of the activity for which insertion times are computed,
/// computes the insertion times taking conflicts into account.
pub fn compute_insertion_costs(
    static_data: &[ActivityComputationStaticData],
    insertion_data: &[ActivityInsertionBeginningMinutes],
    index_of_activity: usize,
) -> Vec<InsertionCostsMinutes> {
    let activity_beginnings_with_conflicts =
        get_activity_beginnings_with_conflicts(static_data, insertion_data);

    get_activity_insertion_costs(
        static_data,
        insertion_data,
        activity_beginnings_with_conflicts,
        index_of_activity,
    )
}

/// Given activity data, computes the possible insertion times so that no activities
/// cannot overlap.
pub fn get_activity_beginnings_with_conflicts(
    static_data: &[ActivityComputationStaticData],
    insertion_data: &[ActivityInsertionBeginningMinutes],
) -> Vec<BTreeSet<ActivityBeginningMinutes>> {
    // Preallocate data
    let mut beginnings_for_all_activities = Vec::with_capacity(static_data.len());

    for activity_static_data in static_data {
        let mut possible_beginnings = activity_static_data
            .possible_insertion_beginnings_minutes_sorted
            .clone();

        // 1 - Fetch invalid beginnings
        // Offset with the duration of the activity
        // (e.g. if 11:00 - 12:00 is taken and our duration is 00:30, we cannot insert the activity
        // at 10:50.
        let offset_check_before_activity =
            activity_static_data.duration_minutes - MIN_TIME_DISCRETIZATION_MINUTES;

        for (incompatible_beginning, incompatible_end) in activity_static_data
            .indexes_of_incompatible_activities
            .iter()
            .copied()
            .filter_map(|index| unsafe {
                if let Some(incompatible_beginning) = insertion_data.get_unchecked(index) {
                    // The activity is inserted.
                    // Use it to filter out conflicts
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

            // 2 - Remove invalid beginnings
            for beginning in activity_static_data
                .possible_insertion_beginnings_minutes_sorted
                .range(incompatible_beginning..incompatible_end)
            {
                possible_beginnings.remove(&beginning);
            }
        }
        // 3 - Insert possible beginnings
        beginnings_for_all_activities.push(possible_beginnings);
    }
    beginnings_for_all_activities
}

/// Given activity data and possible insertion times taking conflicts into account,
/// computes new possible insertion times associated with a cost.
pub fn get_activity_insertion_costs(
    static_data: &[ActivityComputationStaticData],
    insertion_data: &[ActivityInsertionBeginningMinutes],
    possible_insertions_with_conflicts: Vec<BTreeSet<ActivityBeginningMinutes>>,
    index_of_activity: usize,
) -> Vec<InsertionCostsMinutes> {

    let possible_beginnings = (|| unsafe {
            possible_insertions_with_conflicts.get_unchecked(index_of_activity)
    })();
    let activity_static_data = (|| unsafe {
        static_data.get_unchecked(index_of_activity)
    })();

    // 1 - Calculate scores for the remaining beginnings
    let mut cost_for_all_beginnings = Vec::with_capacity(possible_beginnings.len());

    // Copy u16
    for beginning in possible_beginnings.iter().copied() {
        let end = beginning + activity_static_data.duration_minutes;

        let mut cost = 0;
        let mut beginning_will_block_other_activities = false;

        for (
            incompatible_activities_static_data,
            incompatible_activities_insertions_with_conflict,
        ) in activity_static_data
            .indexes_of_incompatible_activities
            .iter()
            .copied()
            .filter_map(|index| unsafe {
                if insertion_data.get_unchecked(index).is_some() {
                    // The activity is inserted, don't count it (we won't block it)
                    None
                } else {
                    Some((
                        static_data.get_unchecked(index),
                        possible_insertions_with_conflicts.get_unchecked(index),
                    ))
                }
            })
        {
            let offset_check_before_activity = incompatible_activities_static_data
                .duration_minutes
                - MIN_TIME_DISCRETIZATION_MINUTES;

            let beginning_with_duration_offset = if beginning < offset_check_before_activity {
                0
            } else {
                beginning - offset_check_before_activity
            };

            let nb_beginnings_blocked = incompatible_activities_insertions_with_conflict
                .range(beginning_with_duration_offset..end)
                .count();

            let nb_possible_beginnings = incompatible_activities_insertions_with_conflict.len();

            let nb_remaining_beginnings = nb_possible_beginnings - nb_beginnings_blocked;

            if 0 == nb_remaining_beginnings {
                beginning_will_block_other_activities = true;
                break;
            } else {
                // Is at least one (at least this activity is incompatible)
                let nb_incompatible_activities = incompatible_activities_static_data
                    .indexes_of_incompatible_activities
                    .len();

                const SIGNIFICANT_DIGIT_MULTIPLIER: usize = 1000;
                cost += SIGNIFICANT_DIGIT_MULTIPLIER
                    * nb_beginnings_blocked
                    * nb_incompatible_activities
                    / nb_remaining_beginnings;
            }
        }
        // The activity can be inserted
        if !beginning_will_block_other_activities {
            cost_for_all_beginnings.push(InsertionCostsMinutes {
                beginning_minutes: beginning,
                cost,
            });
        }
    }
    cost_for_all_beginnings
}
