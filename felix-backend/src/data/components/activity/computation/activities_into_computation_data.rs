use crate::data::{Activity, ActivityId};
use felix_computation_api::structs::{ActivityBeginningMinutes, ActivityComputationStaticData};

use std::collections::HashMap;

/// Returns data ready for auto-insertion of all activities.
///
/// The ids of incompatible activities are turned into indexes.
#[must_use]
pub fn activities_into_computation_data(
    activities: &[Activity],
) -> (
    Vec<ActivityComputationStaticData>,
    Vec<ActivityBeginningMinutes>,
) {
    let mut static_data_vec = Vec::with_capacity(activities.len());
    // Insertion data is only as large as the number of inserted activities
    let mut insertion_data_vec = Vec::new();

    let sorted_activities = activities_sorted_for_computation(activities);

    let ids = sorted_activities
        .iter()
        .map(|other| other.metadata.id())
        .collect::<Vec<_>>();

    for activity in sorted_activities {
        let computation_data = &activity.computation_data;
        let incompatible_ids = computation_data.incompatible_activity_ids();

        // Translate incompatible ids into incompatible indexes
        // This is not the most efficient but this operation is not critical:
        // the computation should be optimized, not this
        let mut incompatible_indexes: Vec<ActivityId> = Vec::with_capacity(incompatible_ids.len());
        for (index_of_other, id_of_other) in ids.iter().enumerate() {
            // We don't care if we compare ourselves to ourselves,
            // we cannot be incompatible with ourselves
            if incompatible_ids.contains(&id_of_other) {
                incompatible_indexes.push(index_of_other);
            }
        }

        let possible_insertion_beginnings_minutes_sorted = activity
            .insertion_costs()
            .expect(
                "Fetching computation even though activity beginnings have not been computed yet",
            )
            .iter()
            .map(|insertion_cost| insertion_cost.beginning.total_minutes())
            .collect();

        let static_data = ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted,
            indexes_of_incompatible_activities: incompatible_indexes,
            duration_minutes: computation_data.duration().total_minutes(),
        };

        static_data_vec.push(static_data);

        // TODO Don't put already inserted activities in !
        // Instead, reduce the id of incompatible activities
        // This does not matter for now (no logic issue)
        // but it will make for faster computations

        // Only the first activities are inserted - this can be optimized
        if let Some(insertion_beginning) = computation_data
            .insertion_interval()
            .map(|interval| interval.beginning().total_minutes())
        {
            insertion_data_vec.push(insertion_beginning);
        }
    }
    (static_data_vec, insertion_data_vec)
}

/// Given a number of activities, returns the id -> index conversion performed when activities are
/// turned into computation data.
#[must_use]
pub fn id_to_index_map(activities: &[Activity]) -> HashMap<ActivityId, usize> {
    let index_to_id_map = index_to_id_map(activities);
    let mut id_to_index_map = HashMap::new();

    // Reverse the index to id map
    for (index, id) in index_to_id_map {
        id_to_index_map.insert(id, index);
    }

    id_to_index_map
}

/// Given a number of activities, returns the index -> id conversion performed when activities are
/// turned into computation data.
#[must_use]
pub fn index_to_id_map(activities: &[Activity]) -> HashMap<usize, ActivityId> {
    let mut index_to_id_map = HashMap::new();
    let sorted_activities = activities_sorted_for_computation(activities);

    for (index, sorted_activity) in sorted_activities.iter().enumerate() {
        index_to_id_map.insert(index, sorted_activity.id());
    }

    index_to_id_map
}

/// From a slice of activities, returns them in a sorted order.
/// This is the order which is used for computation.
#[must_use]
pub fn activities_sorted_for_computation(activities: &[Activity]) -> Vec<Activity> {
    // Split inserted and non inserted activities.
    // Inserted activities are put first as the insertion order is fixed.
    let inserted_activities = activities
        .iter()
        .filter(|activity| activity.computation_data.insertion_interval().is_some());

    let mut non_inserted_activities = activities
        .iter()
        .filter(|activity| activity.computation_data.insertion_interval().is_none())
        .collect::<Vec<_>>();

    // Harder to insert activities should be inserted first - insertion order is fixed
    non_inserted_activities.sort_by_key(|activity| {
        std::cmp::Reverse(
            activity.computation_data.duration().total_minutes() as usize
                * activity.computation_data.incompatible_activity_ids().len(),
        )
    });

    inserted_activities
        .chain(non_inserted_activities)
        .cloned()
        .collect()
}
