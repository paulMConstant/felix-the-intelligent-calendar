#[cfg(test)]
mod tests;

use super::{
    computation::{
        activities_into_computation_data::index_to_id_map,
        id_computation::{compute_incompatible_ids, generate_next_id},
        separate_thread_activity_computation::SeparateThreadActivityComputation,
    },
    ActivityMetadata,
};

use crate::data::{
    computation_structs::{InsertionCost, WorkHoursAndActivityDurationsSorted},
    Activity, ActivityId, Rgba, Time,
};

use crate::errors::Result;

use felix_computation_api::structs::ActivityBeginningMinutes;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};

pub(crate) type ActivitiesAndOldInsertionBeginnings = HashMap<ActivityId, Time>;

/// Manages the collection of activities.
/// Makes sures there are no id duplicates.
#[derive(Debug, Serialize, Deserialize)]
pub struct Activities {
    activities: Vec<Activity>,
    #[serde(skip)]
    separate_thread_computation: SeparateThreadActivityComputation,
    #[serde(skip)]
    activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings,
}

impl Activities {
    /// Initializes the Activity collection.
    #[must_use]
    pub fn new() -> Activities {
        Activities {
            activities: Vec::new(),
            separate_thread_computation: SeparateThreadActivityComputation::new(),
            activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings::new(
            ),
        }
    }

    /// Simple getter for the activity list, sorted by name.
    #[must_use]
    pub fn get_sorted_by_name(&self) -> Vec<Activity> {
        let mut activities = self.activities.clone();
        activities.sort_by_key(|a| a.name());
        activities
    }

    /// Getter for all activities, not sorted.
    #[must_use]
    pub fn get_not_sorted(&self) -> &Vec<Activity> {
        &self.activities
    }

    /// Returns a copy of the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn get_by_id(&self, id: ActivityId) -> &Activity {
        self.activities
            .iter()
            .find(|activity| activity.id() == id)
            .expect("Asking for activity which does not exist")
    }

    /// Simple private mutable getter for an activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    fn get_mut_by_id(&mut self, id: ActivityId) -> &mut Activity {
        self.activities
            .iter_mut()
            .find(|activity| activity.id() == id)
            .expect("Asking for activity which does not exist")
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
        let used_ids = self
            .activities
            .iter()
            .map(|activity| activity.id())
            .collect();
        let id = generate_next_id(used_ids);
        let activity = Activity::new(id, name);

        self.separate_thread_computation
            .register_new_activity(id, activity.computation_data.insertion_costs());

        self.activities.insert(id, activity);
        self.get_by_id(id)
    }

    /// Removes the activity with given id from the collection.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn remove(&mut self, id: ActivityId) {
        let position = self
            .activities
            .iter()
            .position(|activity| activity.id() == id)
            .expect("Activity with given ID does not exist");

        self.activities.swap_remove(position);

        self.update_incompatible_activities();
        self.separate_thread_computation
            .register_activity_removed(id);
    }

    /// Changes the name of the activity with the given id to the given name.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn set_name(&mut self, id: ActivityId, name: String) {
        self.get_mut_by_id(id).metadata.set_name(name);
    }

    /// Adds an entity to the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is already taking part in the activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn add_entity(&mut self, id: ActivityId, entity: String) -> Result<()> {
        self.get_mut_by_id(id).metadata.add_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Adds an entity in every activity which contains the given group.
    pub fn add_entity_to_activities_with_group(&mut self, group_name: &str, entity_name: String) {
        for activity in self
            .activities
            .iter_mut()
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
            .iter()
            .map(|activity| activity.metadata.clone())
            .collect();

        // 2. Iterate over the copied metadata to fill incompatible ids (activities which
        // have at least one entity in common are incompatible).
        // If the activity has the same id, it is the same activity, don't add it
        for metadata in &metadata_vec {
            self.get_mut_by_id(metadata.id())
                .computation_data
                .set_incompatible_activity_ids(compute_incompatible_ids(&metadata, &metadata_vec));
        }
    }

    /// Removes an entity from the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not taking part in the activtiy.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn remove_entity(&mut self, id: ActivityId, entity: &str) -> Result<()> {
        self.get_mut_by_id(id).metadata.remove_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Removes the entity with given name from all activities.
    pub fn remove_entity_from_all(&mut self, entity: &str) {
        for activity in self.activities.iter_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, that is what we want in the first place
            let _ = activity.metadata.remove_entity(entity);
        }
        self.update_incompatible_activities();
    }

    /// Renames the entity with given name in all activities.
    pub fn rename_entity_in_all(&mut self, old_name: &str, new_name: String) {
        for activity in self.activities.iter_mut() {
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
        self.get_mut_by_id(id).metadata.add_group(group_name)
    }

    /// Removes the group with the given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not taking part in the activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn remove_group(&mut self, id: ActivityId, group_name: &str) -> Result<()> {
        self.get_mut_by_id(id).metadata.remove_group(group_name)
    }

    /// Removes the group with the given name from all activities.
    pub fn remove_group_from_all(&mut self, group: &str) {
        for activity in self.activities.iter_mut() {
            // We don't care about the result: if the group is not in the activity, this
            // is what we want.
            let _ = activity.metadata.remove_group(group);
        }
        self.update_incompatible_activities();
    }

    /// Renames the group with given name in all activities.
    pub fn rename_group_in_all(&mut self, old_name: &str, new_name: String) {
        for activity in self.activities.iter_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, it does not need to be renamed
            let _ = activity.metadata.rename_group(old_name, new_name.clone());
        }
    }

    /// Sets the duration of the activity with the given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn set_duration(&mut self, id: ActivityId, duration: Time) {
        self.get_mut_by_id(id)
            .computation_data
            .set_duration(duration);
    }

    /// Sets the color of the activity with the given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn set_color(&mut self, id: ActivityId, color: Rgba) {
        self.get_mut_by_id(id).metadata.set_color(color);
    }

    /// Triggers the computation of new possible beginnings for the given activities.
    pub fn trigger_update_possible_activity_beginnings(
        &mut self,
        schedules_of_participants: Vec<WorkHoursAndActivityDurationsSorted>,
        concerned_activity_ids: HashSet<Activity>,
    ) {
        //self.separate_thread_computation
        //.queue_work_hours_and_activity_durations(
        //schedules_of_participants,
        //concerned_activity_ids,
        //);
        // TODO compute possible insertion times if no conflict
    }

    /// Inserts the activity with the given beginning.
    /// If None is given, the activity is removed from the schedule.
    /// Checks are done by the Data module.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID is not found.
    pub fn insert_activity(&mut self, id: ActivityId, beginning: Option<Time>) {
        let activity = self.get_mut_by_id(id);
        activity.computation_data.insert(beginning);
    }

    /// Keeps the insertion time of an activity which was removed due to an increase of its
    /// duration. The activity will then be inserted in the closest spot if possible.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not inserted anywhere or the activity with given ID does not
    /// exist.
    pub(crate) fn store_activity_was_inserted(&mut self, id: ActivityId) {
        let activity = self.get_by_id(id);
        let insertion_beginning = activity
            .insertion_interval()
            .expect("Storing insertion time of activity which is not inserted anywhere")
            .beginning();

        self.activities_removed_because_duration_increased
            .insert(id, insertion_beginning);
    }

    /// Tries to insert the given activity in the spot which is the closest to the given beginning.
    /// If the activity is inserted succesfuly, returns true.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not found. This should never happen as this function is only
    /// called internally (no invalid user input).
    pub fn insert_activity_in_spot_closest_to(
        &mut self,
        id: ActivityId,
        ideal_beginning: Time,
        possible_beginnings: BTreeSet<InsertionCost>,
    ) -> bool {
        // We remove this activity from the list of activities to insert back.
        self.activities_removed_because_duration_increased
            .remove(&id);

        // We try to insert the activity.
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
            // Min is the closest time distance.
            // If two distances are equal, takes the one with the smallest beginning (default tuple
            // Ord behaviour).
            .min()
        {
            self.insert_activity(id, Some(closest_spot.1));
            true
        } else {
            false
        }
    }

    /// Associates each computation data to its rightful activity then overwrites it.
    pub fn overwrite_insertion_data(&mut self, insertion_data: Vec<ActivityBeginningMinutes>) {
        let index_to_id_map = index_to_id_map(self.get_not_sorted());
        for (index, insertion) in insertion_data.into_iter().enumerate() {
            let id = index_to_id_map[&index];
            self.get_mut_by_id(id)
                .computation_data
                .insert(Some(Time::from_total_minutes(insertion)));
        }
    }
}

/// Used only for testing.
impl Clone for Activities {
    fn clone(&self) -> Self {
        Activities {
            activities: self.activities.clone(),
            separate_thread_computation: SeparateThreadActivityComputation::default(),
            activities_removed_because_duration_increased:
                ActivitiesAndOldInsertionBeginnings::default(),
        }
    }
}

impl PartialEq for Activities {
    fn eq(&self, other: &Self) -> bool {
        self.activities == other.activities
    }
}
