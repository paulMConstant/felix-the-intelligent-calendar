mod inner;
#[cfg(test)]
mod tests;

use crate::Activity;

use super::computation::{
    activities_into_computation_data::index_to_id_map, id_computation::generate_next_id,
    separate_thread_activity_computation::SeparateThreadActivityComputation,
};

use felix_datatypes::{
    Time,
    ActivityId,
    Rgba,
    WorkHoursAndActivityDurationsSorted,
    ActivityBeginningMinutes,
    InsertionCost,
};

use felix_errors::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) type ActivitiesAndOldInsertionBeginnings = HashMap<ActivityId, Time>;

/// Manages the collection of activities.
/// Makes sures there are no id duplicates.
#[derive(Debug, Serialize, Deserialize)]
pub struct Activities {
    // Reason for Arc:
    // Writing in the activities to update the insertion costs asynchronously
    // Reading the activities collection to do this computation asynchronously
    activities: Arc<Mutex<Vec<Activity>>>,
    #[serde(skip)]
    separate_thread_computation: SeparateThreadActivityComputation,
    #[serde(skip)]
    activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings,
}

impl Activities {
    /// Initializes the Activity collection.
    #[must_use]
    pub fn new() -> Activities {
        let activities = Arc::new(Mutex::new(Vec::new()));

        // This thread will silently update the activities insertion costs when durations,
        // entities or work hours change
        let separate_thread_computation = SeparateThreadActivityComputation::new();

        Activities {
            activities,
            separate_thread_computation,
            activities_removed_because_duration_increased: ActivitiesAndOldInsertionBeginnings::new(
            ),
        }
    }

    pub fn run_separate_thread_computation(&self) {
        self.separate_thread_computation
            .run_update_insertion_costs_thread(self.activities.clone());
    }

    /// Simple getter for the activity list, sorted by name.
    #[must_use]
    pub fn get_sorted_by_name(&self) -> Vec<Activity> {
        let mut activities = self.activities.lock().unwrap().clone();
        activities.sort_by_key(|a| a.name());
        activities
    }

    /// Getter for all activities, not sorted.
    #[must_use]
    pub fn get_not_sorted(&self) -> Vec<Activity> {
        self.activities.lock().unwrap().clone()
    }

    /// Returns a copy of the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn get_by_id(&self, id: ActivityId) -> Activity {
        self.activities
            .lock()
            .expect("Some thread panicked with the activities lock. This is a bug.")
            .iter()
            .find(|activity| activity.id() == id)
            .expect("Asking for activity which does not exist")
            .clone()
    }

    /// Getter for activities which were removed from the schedule because their duration
    /// increased.
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
    /// Returns a copy of the created activity.
    pub fn add(&self, name: String) -> Activity {
        let mut activities = self.activities.lock().unwrap();

        let used_ids = activities.iter().map(|activity| activity.id()).collect();
        let id = generate_next_id(used_ids);
        let activity = Activity::new(id, name);

        activities.insert(id, activity);
        // Free lok
        drop(activities);
        self.get_by_id(id)
    }

    /// Removes the activity with given id from the collection.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn remove(&self, id: ActivityId) {
        let mut activities = self.activities.lock().unwrap();

        let position = activities
            .iter()
            .position(|activity| activity.id() == id)
            .expect("Activity with given ID does not exist");

        activities.swap_remove(position);

        // Free lock
        drop(activities);
        self.update_incompatible_activities();
    }

    /// Changes the name of the activity with the given id to the given name.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn set_name(&mut self, id: ActivityId, name: String) {
        self.mutate_activity(id, |a| a.metadata.set_name(name));
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
        self.mutate_activity(id, |a| a.metadata.add_entity(entity))?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Adds an entity in every activity which contains the given group.
    pub fn add_entity_to_activities_with_group(&mut self, group_name: &str, entity_name: String) {
        for activity in self
            .activities
            .lock()
            .unwrap()
            .iter_mut()
            .filter(|activity| activity.groups_sorted().contains(&group_name.into()))
        {
            // We do not care about errors : we want the activity to contain the entity, if it
            // is already the case, it is fine
            let _ = activity.metadata.add_entity(entity_name.clone());
        }
        self.update_incompatible_activities();
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
        self.mutate_activity(id, |a| -> Result<()> {
            a.metadata.remove_entity(entity)?;
            if a.metadata.entities_sorted().is_empty() {
                // TODO comment out this line and make tests fail
                *a.computation_data.insertion_costs().lock().unwrap() = Some(Vec::new());
            }
            Ok(())
        })?;
        self.update_incompatible_activities();
        Ok(())
    }

    // TODO remove this ! All operations should pass by the data collection.
    /// Removes the entity with given name from all activities.
    pub fn remove_entity_from_all(&mut self, entity: &str) {
        let ids = self
            .activities
            .lock()
            .unwrap()
            .iter()
            .map(|activity| activity.id())
            .collect::<Vec<_>>();

        for id in ids {
            // We don't care about the result : if the entity is not
            // taking part in the activity, that is what we want in the first place
            let _ = self.remove_entity(id, &entity);
        }
    }

    /// Renames the entity with given name in all activities.
    pub fn rename_entity_in_all(&mut self, old_name: &str, new_name: String) {
        for activity in self.activities.lock().unwrap().iter_mut() {
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
        self.mutate_activity(id, |a| a.metadata.add_group(group_name))
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
        self.mutate_activity(id, |a| a.metadata.remove_group(group_name))?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Renames the group with given name in all activities.
    pub fn rename_group_in_all(&mut self, old_name: &str, new_name: String) {
        for activity in self.activities.lock().unwrap().iter_mut() {
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
        self.mutate_activity(id, |a| {
            a.computation_data.set_duration(duration);
            // Empty duration => Set insertion costs to computed but empty
            if duration == Time::new(0, 0) {
                *a.computation_data.insertion_costs().lock().unwrap() = Some(Vec::new());
            }
        });
    }

    /// Sets the color of the activity with the given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn set_color(&mut self, id: ActivityId, color: Rgba) {
        self.mutate_activity(id, |a| a.metadata.set_color(color));
    }

    /// Triggers the computation of new possible beginnings for the given activities.
    pub fn trigger_update_possible_activity_beginnings(
        &mut self,
        schedules_of_participants: Vec<WorkHoursAndActivityDurationsSorted>,
    ) {
        self.separate_thread_computation
            .queue_work_hours_and_activity_durations(
                schedules_of_participants,
                self.activities.clone(),
            );
    }

    /// Inserts the activity with the given beginning.
    /// If None is given, the activity is removed from the schedule.
    /// Checks are done by the Data module.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID is not found.
    pub fn insert_activity(&mut self, id: ActivityId, beginning: Option<Time>) {
        self.mutate_activity(id, |a| {
            a.computation_data.insert(beginning);
            if a.metadata.entities_sorted().is_empty() {
                // No participants => Set insertion costs to computed but empty
                *a.computation_data.insertion_costs().lock().unwrap() = Some(Vec::new());
            }
        });
    }

    /// Updates the schedules of the participants of the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity does not exist.
    pub fn update_schedules_of_participants_of_activity(
        &mut self,
        id: ActivityId,
        schedules: Vec<WorkHoursAndActivityDurationsSorted>,
    ) {
        self.mutate_activity(id, |activity| {
            activity
                .computation_data
                .update_schedules_of_participants(schedules)
        });
    }

    /// Keeps the insertion time of an activity which was removed due to an increase of its
    /// duration. The activity will then be inserted in the closest spot if possible.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not inserted anywhere or the activity with given ID does not
    /// exist.
    pub fn store_activity_was_inserted(&mut self, id: ActivityId) {
        let activity = self.get_by_id(id);
        let insertion_beginning = activity
            .insertion_interval()
            .expect("Storing insertion time of activity which is not inserted anywhere")
            .beginning();

        self.activities_removed_because_duration_increased
            .insert(id, insertion_beginning);
    }

    /// Returns the closest insertion spot to the given beginning for the given activity.
    /// If the activity cannot be inserted, returns None.
    ///
    /// # Panics
    ///
    /// Panics if the id is invalid.
    pub fn get_closest_spot_to_insert_activity(
        &mut self,
        id: ActivityId,
        ideal_beginning: Time,
        possible_beginnings: Vec<InsertionCost>,
    ) -> Option<Time> {
        // We remove this activity from the list of activities to insert back.
        self.activities_removed_because_duration_increased
            .remove(&id);

        // We try to insert the activity.
        possible_beginnings
            .into_iter()
            // Map into (time_difference, beginning) tuples
            .map(|insertion_cost| {
                let beginning = insertion_cost.beginning;
                // Abs with usize => avoid substract with overflow
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
            // Only return the beginning and not the time difference
            .map(|closest_spot| closest_spot.1)
    }

    /// Associates each computation data to its rightful activity then overwrites it.
    pub fn overwrite_insertion_data(&mut self, insertion_data: Vec<ActivityBeginningMinutes>) {
        let index_to_id_map = index_to_id_map(&self.get_not_sorted());
        for (index, insertion) in insertion_data.into_iter().enumerate() {
            let id = index_to_id_map[&index];
            self.mutate_activity(id, |a| {
                a.computation_data
                    .insert(Some(Time::from_total_minutes(insertion)))
            });
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
        // Clone to make sure that both are not locked at the same time if under the same mutex
        let activities = self.activities.lock().unwrap().clone();
        activities == *other.activities.lock().unwrap()
    }
}

impl Default for Activities {
    fn default() -> Self {
        Self::new()
    }
}
