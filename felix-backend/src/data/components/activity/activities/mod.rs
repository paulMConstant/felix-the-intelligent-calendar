#[cfg(test)]
mod tests;

use super::{
    computation::{
        id_computation::{compute_incompatible_ids, generate_next_id},
        possible_beginnings_updater::PossibleBeginningsUpdater,
    },
    ActivityMetadata,
};

use crate::data::{
    computation_structs::{InsertionCost, WorkHoursAndActivityDurationsSorted},
    Activity, ActivityId, Rgba, Time, MIN_TIME_DISCRETIZATION,
};

use crate::errors::{does_not_exist::DoesNotExist, duration_too_short::DurationTooShort, Result};
use felix_computation_api::{
    compute_insertion_costs,
    structs::{ActivityComputationStaticData, ActivityInsertionBeginningMinutes},
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::{BTreeSet, HashMap, HashSet};

pub(crate) type ActivitiesAndOldInsertionBeginnings = HashMap<ActivityId, Time>;

/// Manages the collection of activities.
/// Makes sures there are no id duplicates.
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Activities {
    // Cannot serialize with non string keys => turn into vec
    #[serde_as(as = "Vec<(_, _)>")]
    activities: HashMap<ActivityId, Activity>,
    #[serde(skip)]
    possible_beginnings_updater: PossibleBeginningsUpdater,
    #[serde(skip)]
    activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings,
    /// Keeps track of the last id to index translation done in self.fetch\_computation.
    ///
    /// # Context:
    ///
    /// When we get parallel arrays from fetch\_computation, we translate the ids of the activities
    /// into indexes. This allows for faster access to incompatible ids for autocompletion
    /// algorithm.
    #[serde(skip)]
    last_fetch_computation_id_to_index_map: HashMap<ActivityId, usize>,
}

impl Activities {
    /// Initializes the Activity collection.
    #[must_use]
    pub fn new() -> Activities {
        Activities {
            activities: HashMap::new(),
            possible_beginnings_updater: PossibleBeginningsUpdater::new(),
            activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings::new(
            ),
            last_fetch_computation_id_to_index_map: HashMap::new(),
        }
    }

    /// Simple getter for the activity list, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Activity> {
        let mut activity_vec: Vec<&Activity> = self.activities.values().collect();
        activity_vec.sort_by_key(|a| a.name());
        activity_vec
    }

    /// Returns a copy of the activity with given id.
    pub fn get_by_id(&self, id: ActivityId) -> Result<Activity> {
        match self.activities.get(&id) {
            Some(activity) => Ok(activity.clone()),
            None => Err(DoesNotExist::activity_does_not_exist(id)),
        }
    }

    /// Simple private mutable getter for an activity.
    fn get_mut_by_id(&mut self, id: ActivityId) -> Result<&mut Activity> {
        match self.activities.get_mut(&id) {
            Some(activity) => Ok(activity),
            None => Err(DoesNotExist::activity_does_not_exist(id)),
        }
    }

    /// Getter for activities which were removed from the schedule because their duration increased.
    pub fn get_activities_removed_because_duration_increased(
        &self,
    ) -> ActivitiesAndOldInsertionBeginnings {
        self.activities_removed_because_duration_increased.clone()
    }

    /// Empties the list of activities which were removed because their duration increased.
    pub fn clear_activities_removed_because_duration_increased(&mut self) {
        self.activities_removed_because_duration_increased.clear();
    }

    /// Adds an activity with the given name to the collection.
    /// Automatically assigns a unique id.
    /// Returns an immutable reference to the newly created activity.
    pub fn add(&mut self, name: String) -> &Activity {
        let used_ids = self.activities.keys().collect();
        let id = generate_next_id(used_ids);
        let activity = Activity::new(id, name);

        self.activities.insert(id, activity);
        self.possible_beginnings_updater.notify_new_activity(id);

        &self
            .activities
            .get(&id)
            .expect("Either the activity was not inserted or getter does not work")
    }

    /// Removes the activity with given id from the collection.
    ///
    /// # Errors
    ///
    /// Returns Err if there is no activity with the given id.
    pub fn remove(&mut self, id: ActivityId) -> Result<()> {
        match self.activities.remove(&id) {
            None => Err(DoesNotExist::activity_does_not_exist(id)),
            Some(_) => {
                self.update_incompatible_activities();
                self.possible_beginnings_updater.notify_activity_removed(id);
                Ok(())
            }
        }
    }

    /// Changes the name of the activity with the given id to the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if there is no activity with the given id.
    pub fn set_name(&mut self, id: ActivityId, name: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.set_name(name);
        Ok(())
    }

    /// Adds an entity to the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the entity is already
    /// taking part in the activity.
    pub fn add_entity(&mut self, id: ActivityId, entity: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.add_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Adds an entity in every activity which contains the given group.
    pub fn add_entity_to_activities_with_group(&mut self, group_name: &str, entity_name: String) {
        for activity in self
            .activities
            .values_mut()
            .filter(|activity| activity.groups_sorted().contains(&group_name.into()))
        {
            // We do not care about errors : we want the activity to contain the entity, if it
            // is already the case, it is fine
            let _ = activity.metadata.add_entity(entity_name.clone());
        }
        self.update_incompatible_activities();
    }

    /// Updates the incompatible activity ids of each activity.
    ///
    /// Used for internal computation only.
    fn update_incompatible_activities(&mut self) {
        // 1. Create a copy of the metadata
        let metadata_vec: Vec<ActivityMetadata> = self
            .activities
            .values()
            .map(|activity| activity.metadata.clone())
            .collect();

        // 2. Iterate over the copied metadata to fill incompatible ids (activities which
        // have at least one entity in common are incompatible).
        // If the activity has the same id, it is the same activity, don't add it
        for metadata in &metadata_vec {
            self.activities
                .get_mut(&metadata.id())
                .expect("Metadata has id which is not recognized in activiites.get_mut !")
                .computation_data
                .set_incompatible_activity_ids(compute_incompatible_ids(&metadata, &metadata_vec));
        }
    }

    /// Removes an entity from the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the entity is not
    /// taking part in the activtiy.
    pub fn remove_entity(&mut self, id: ActivityId, entity: &str) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.remove_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Removes the entity with given name from all activities.
    pub fn remove_entity_from_all(&mut self, entity: &str) {
        for activity in self.activities.values_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, that is what we want in the first place
            let _ = activity.metadata.remove_entity(entity);
        }
        self.update_incompatible_activities();
    }

    /// Renames the entity with given name in all activities.
    pub fn rename_entity_in_all(&mut self, old_name: &str, new_name: String) {
        for activity in self.activities.values_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, it does not need to be renamed
            let _ = activity.metadata.rename_entity(old_name, new_name.clone());
        }
    }

    /// Adds the group with the given name to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if
    /// the group is already taking part in the activity.
    pub fn add_group(&mut self, id: ActivityId, group_name: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.add_group(group_name)
    }

    /// Removes the group with the given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity si not found or if the group is not taking part in the
    /// activity.
    pub fn remove_group(&mut self, id: ActivityId, group_name: &str) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.remove_group(group_name)
    }

    /// Removes the group with the given name from all activities.
    pub fn remove_group_from_all(&mut self, group: &str) {
        for activity in self.activities.values_mut() {
            // We don't care about the result: if the group is not in the activity, this
            // is what we want.
            let _ = activity.metadata.remove_group(group);
        }
        self.update_incompatible_activities();
    }

    /// Renames the group with given name in all activities.
    pub fn rename_group_in_all(&mut self, old_name: &str, new_name: String) {
        for activity in self.activities.values_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, it does not need to be renamed
            let _ = activity.metadata.rename_group(old_name, new_name.clone());
        }
    }

    /// Sets the duration of the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the duration is too short
    /// (< MIN\_TIME\_DISCRETIZATION).
    pub fn set_duration(&mut self, id: ActivityId, duration: Time) -> Result<()> {
        if duration < MIN_TIME_DISCRETIZATION {
            return Err(DurationTooShort::new());
        }

        self.get_mut_by_id(id)?
            .computation_data
            .set_duration(duration);
        Ok(())
    }

    /// Sets the color of the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    pub fn set_color(&mut self, id: ActivityId, color: Rgba) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.set_color(color);
        Ok(())
    }

    /// Triggers the computation of new possible beginnings for the given activities.
    pub fn trigger_update_possible_activity_beginnings(
        &mut self,
        schedules_of_participants: &[WorkHoursAndActivityDurationsSorted],
        concerned_activity_ids: HashSet<ActivityId>,
    ) {
        self.possible_beginnings_updater
            .queue_work_hours_and_activity_durations(
                schedules_of_participants,
                concerned_activity_ids,
            );
    }

    /// Updates the possible insertion times of an activity.
    /// Returns true if the possible insertion times were computed, else false.
    /// This function must be called before
    /// possible_insertion_times_of_activity_with_associated_cost.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity with given id is not found.
    pub(crate) fn update_possible_insertion_times_of_activity(
        &mut self,
        schedules_of_participants: &[WorkHoursAndActivityDurationsSorted],
        concerned_activity_id: ActivityId,
    ) -> Result<bool> {
        let concerned_activity = self.get_by_id(concerned_activity_id)?;

        let possible_beginnings_are_computed = if self
            .possible_beginnings_updater
            .activity_beginnings_are_up_to_date(&concerned_activity_id)
        {
            true
        } else {
            let maybe_possible_beginnings = self
                .possible_beginnings_updater
                .poll_and_fuse_possible_beginnings(&schedules_of_participants, &concerned_activity);

            if maybe_possible_beginnings.is_some() {
                // If the result is valid, store it into the activity computation data.
                let result =
                    maybe_possible_beginnings.expect("Maybe result should be some but is not");
                let activity = self
                    .get_mut_by_id(concerned_activity_id)
                    .expect("Getting activity which does not exist");
                activity
                    .computation_data
                    .set_possible_insertion_times_if_no_conflict(result);
                true
            } else {
                false
            }
        };
        Ok(possible_beginnings_are_computed)
    }

    /// Returns the possible beginnings of an activity if it is up to date or if
    /// the computation results are up.
    /// If neither is the case, returns None.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity with given id is not found.
    pub fn possible_insertion_times_of_activity_with_associated_cost(
        &mut self,
        schedules_of_participants: &[WorkHoursAndActivityDurationsSorted],
        concerned_activity_id: ActivityId,
    ) -> Result<Option<BTreeSet<InsertionCost>>> {
        let possible_beginnings_are_computed = self.update_possible_insertion_times_of_activity(
            schedules_of_participants,
            concerned_activity_id,
        )?;

        let possible_beginnings = if possible_beginnings_are_computed {
            // Filter & compute the cost of each possible beginning
            let (static_data, insertion_data) = self.fetch_computation();

            let index_of_activity =
                self.last_fetch_computation_id_to_index_map[&concerned_activity_id];

            let insertion_costs_of_activity =
                &compute_insertion_costs(&static_data, &insertion_data)[index_of_activity];

            Some(
                insertion_costs_of_activity
                    .iter()
                    .map(|insertion_cost_minutes| InsertionCost {
                        beginning: Time::from_total_minutes(
                            insertion_cost_minutes.beginning_minutes,
                        ),
                        cost: insertion_cost_minutes.cost,
                    })
                    .collect::<BTreeSet<_>>(),
            )
        } else {
            None
        };
        Ok(possible_beginnings)
    }

    /// Inserts the activity with the given beginning.
    /// If None is given, the activity is removed from the schedule.
    /// Checks are done by the Data module.
    ///
    /// # Errors
    /// Returns Err if the activity is not found.
    pub fn insert_activity(&mut self, id: ActivityId, beginning: Option<Time>) -> Result<()> {
        let activity = self.get_mut_by_id(id)?;
        activity.computation_data.insert(beginning);
        Ok(())
    }

    /// Keeps the insertion time of an activity which was removed due to an increase of its
    /// duration. The activity will then be inserted in the closest spot if possible.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not inserted anywhere (this should never happen - logic error).
    pub(crate) fn store_activity_was_inserted(&mut self, id: ActivityId) -> Result<()> {
        let activity = self.get_by_id(id)?;
        let insertion_beginning = activity
            .insertion_interval()
            .expect("Storing insertion time of activity which is not inserted anywhere")
            .beginning();
        self.activities_removed_because_duration_increased
            .insert(id, insertion_beginning);
        Ok(())
    }

    /// Tries to insert the given activity in the spot which is the closest to the given beginning.
    /// If the activity is inserted succesfuly, returns Some(()).
    ///
    /// # Panics
    ///
    /// Panics if the activity is not found. This should never happen as this function is
    /// crate-local.
    pub(crate) fn insert_activity_in_spot_closest_to(
        &mut self,
        id: ActivityId,
        ideal_beginning: Time,
        possible_beginnings: BTreeSet<InsertionCost>,
    ) -> Option<()> {
        // We insert the activity (or at least we try. If we fail, we will fail again).
        // Therefore, remove this activity from the list of activities to insert back
        self.activities_removed_because_duration_increased
            .remove(&id);

        if let Some(closest_spot) = possible_beginnings
            .into_iter()
            // Map into (time_distance, beginning) tuples
            .map(|insertion_cost| {
                let beginning = insertion_cost.beginning;
                if beginning > ideal_beginning {
                    (beginning - ideal_beginning, beginning)
                } else {
                    (ideal_beginning - beginning, beginning)
                }
            })
            // Tuples implement Ord. (2, 3) > (1, 5) and (2, 2) < (2, 3)
            .min()
        {
            self.insert_activity(id, Some(closest_spot.1))
                .expect("The given activity does not exist !");
            Some(())
        } else {
            None
        }
    }

    /// Returns data ready for auto-insertion of all activities.
    ///
    /// The ids of incompatible activities are turned into indexes.
    #[must_use]
    fn fetch_computation(
        &mut self,
    ) -> (
        Vec<ActivityComputationStaticData>,
        Vec<ActivityInsertionBeginningMinutes>,
    ) {
        let mut static_data_vec = Vec::with_capacity(self.activities.len());
        let mut insertion_data_vec = Vec::with_capacity(self.activities.len());

        let ids = self
            .activities
            .values()
            .map(|other| other.metadata.id())
            .collect::<Vec<_>>();
        self.last_fetch_computation_id_to_index_map.clear();

        // Translate incompatible ids into incompatible indexes
        // This is not the most efficient but this operation is not critical:
        // the computation should be optimized, not this
        for (index, activity) in self.activities.values().enumerate() {
            let computation_data = &activity.computation_data;
            let incompatible_ids = computation_data.incompatible_activity_ids();
            let mut incompatible_indexes: Vec<ActivityId> =
                Vec::with_capacity(incompatible_ids.len());
            for (index_of_other, id_of_other) in ids.iter().enumerate() {
                // We don't care if we compare ourselves to ourselves,
                // we cannot be incompatible with ourselves
                if incompatible_ids.contains(&id_of_other) {
                    incompatible_indexes.push(index_of_other);
                }
            }

            let static_data = ActivityComputationStaticData {
                possible_insertion_beginnings_minutes_sorted: computation_data
                    .possible_insertion_times_if_no_conflict()
                    .iter()
                    .map(|time| time.total_minutes())
                    .collect(),
                indexes_of_incompatible_activities: incompatible_indexes,
                duration_minutes: computation_data.duration().total_minutes(),
            };

            let insertion_data = computation_data
                .insertion_interval()
                .map(|interval| interval.beginning().total_minutes());

            static_data_vec.push(static_data);
            insertion_data_vec.push(insertion_data);

            // Keep track of the id -> index translation to revert it
            self.last_fetch_computation_id_to_index_map
                .insert(activity.id(), index);
        }
        (static_data_vec, insertion_data_vec)
    }

    /// TODO
    /// Associates each computation data to its rightful activity then overwrites it.
    fn overwrite_insertion_data(&mut self) {}
}

/// Used only for testing.
impl Clone for Activities {
    fn clone(&self) -> Self {
        Activities {
            activities: self.activities.clone(),
            possible_beginnings_updater: PossibleBeginningsUpdater::new(),
            activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings::new(
            ),
            last_fetch_computation_id_to_index_map: self
                .last_fetch_computation_id_to_index_map
                .clone(),
        }
    }
}

impl PartialEq for Activities {
    fn eq(&self, other: &Self) -> bool {
        self.activities == other.activities
    }
}
