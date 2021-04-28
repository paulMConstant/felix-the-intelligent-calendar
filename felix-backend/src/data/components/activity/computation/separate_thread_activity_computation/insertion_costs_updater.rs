use crate::data::{
    Activity,
    InsertionCost,
    Time,
};

use felix_computation_api::compute_insertion_costs;

use super::{
    PossibleBeginningsPool, 
    super::activities_into_computation_data::{
        activities_into_computation_data,
        activities_sorted_filtered_for_computation
    },
};

use std::collections::{HashSet};
use std::sync::{Arc, Mutex};

/// Given the schedules of participants and all activities, for each activity,
/// fills the insertion costs for each activity and returns true
///
/// Returns true if the activities can be accessed, else false.
///
/// # Panic
///
/// If any activity does not have possible beginnings(None), panics. This is a logic error and should
/// never happen.
pub(super) fn poll_and_fuse_possible_beginnings(
    activities: Arc<Mutex<Vec<Activity>>>,
    possible_beginnings_pool: Arc<Mutex<PossibleBeginningsPool>>
) -> bool {
    if let Ok(activities) = activities.lock() {
        // Sort and filter activities in computation form
        let activities_sorted_filtered = activities_sorted_filtered_for_computation(&activities);

        let maybe_all_possible_beginnings_for_each_activity = 
            possible_beginnings_for_activities(possible_beginnings_pool, &activities_sorted_filtered);
        if let Some(all_possible_beginnings_for_each_activity) = maybe_all_possible_beginnings_for_each_activity {

            // Intersect all possible beginnings for each activity
            merge_beginnings_of_all_participants_of_each_activity(
                all_possible_beginnings_for_each_activity, 
                &activities_sorted_filtered
            );

            // Once every merge has been done, compute insertion costs
            compute_insertion_costs_for_each_activity(&activities_sorted_filtered);
        }
        true
    } else {
        false
    }
}

/// Fetches the possible beginnings of every activity, not taking conflicts into account.
/// If one result has not been computed, returns None (activities have been modified and this
/// function will be called again once the results have been computed).
/// Each activity has a Vec of HashSet of time, one per entity.
#[must_use]
fn possible_beginnings_for_activities(
    possible_beginnings_pool: Arc<Mutex<PossibleBeginningsPool>>,
    activities: &[Activity],
    ) -> Option<Vec<Vec<HashSet<Time>>>>
{
    let pool = possible_beginnings_pool.lock().unwrap();

    activities.iter()
        .map(|activity| {
            // Get possible beginnings
        activity.computation_data
            .schedules_of_participants()
            .iter()
            .map(|work_hours_and_activity_durations| {
                // HashMap<WorkHoursAndActivityDurationsSorted, HashMap<Time, HashSet<Time>>
                pool.get(work_hours_and_activity_durations)
                    // HashMap<Time, HashSet<Time>>
                    .and_then(|possible_beginnings_given_duration| {
                        possible_beginnings_given_duration.get(&activity.duration()).cloned()
                        // HashSet<Time>
                    })
                // Bring option out of the vec
            }).collect::<Option<Vec<_>>>()
        })
    .collect()
}

/// For each activity in the activity slice, fuses the possible beginnings of all its
/// participant (each participant has a set of times in which they can put the activity).
/// The result is stored directly in the activity.
///
/// The activities and possible beginnings are parallel arrays.
fn merge_beginnings_of_all_participants_of_each_activity(
    mut all_possible_beginnings: Vec<Vec<HashSet<Time>>>, 
    activities: &[Activity]) 
{
    assert!(activities.len() == all_possible_beginnings.len());

    // Merge beginnings with 0 cost
    for (possible_beginnings_for_this_activity, activity) in 
        all_possible_beginnings.iter_mut().zip(activities) {
        // Sort sets by ascending size so that fewer checks are done for intersections
        possible_beginnings_for_this_activity.sort_by_key(|a| a.len());

        let first_set = possible_beginnings_for_this_activity.first();

        let insertion_scores = Some(
            if let Some(first_set) = first_set {
               first_set.into_iter()
                .filter(|time| {
                    possible_beginnings_for_this_activity[1..]
                        .iter()
                        .all(|set| set.contains(time))
                })
                // Map into dummy scores to fetch computation and to calculate scores properly
                .map(|&time| InsertionCost { beginning: time, cost: 0 })
                .collect()
            } else {
                // Possible beginnings have been computed and there are none
                Vec::<InsertionCost>::new()
            }
        );

        // Check if any thread panicked while holding the mutex
        if let Ok(mut costs) = activity.computation_data.insertion_costs().lock() {
            *costs = insertion_scores;
        }
    }
}

/// For each activity, compute its insertion scores and stores them directly in the activity.
fn compute_insertion_costs_for_each_activity(activities: &[Activity]) {
    let (static_data, insertion_data) = activities_into_computation_data(activities);

    // We can iterate in the right order because activities are sorted the same way as they
    // are in computation form
    for (index, activity) in activities.iter().enumerate() {
        let insertion_costs = compute_insertion_costs(&static_data,
                                                      &insertion_data,
                                                      index)
            .into_iter()
            .map(|insertion_cost_minutes| {
                 InsertionCost::from_insertion_cost_minutes(insertion_cost_minutes)
            })
        .collect();

        if let Ok(mut costs) = activity.computation_data.insertion_costs().lock() {
            *costs = Some(insertion_costs);
        }
    }
}
