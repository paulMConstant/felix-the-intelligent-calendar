use super::{
    computation::{
        id_computation::{compute_incompatible_ids, generate_next_id},
        possible_beginnings_updater::PossibleBeginningsUpdater,
    },
    ActivityComputationData, ActivityMetadata,
};

use crate::data::{
    computation_structs::{ComputationDoneNotifier, WorkHoursAndActivityDurationsSorted},
    Activity, ActivityID, Time, MIN_TIME_DISCRETIZATION, MIN_TIME_DISCRETIZATION_MINUTES, RGBA,
};
use crate::errors::{does_not_exist::DoesNotExist, duration_too_short::DurationTooShort, Result};

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

/// Manages the collection of activities.
/// Makes sures there are no id duplicates.
#[derive(Debug)]
pub struct Activities {
    activities: HashMap<ActivityID, Activity>,
    possible_beginnings_updater: PossibleBeginningsUpdater,
}

impl Activities {
    /// Initializes the Activity collection.
    #[must_use]
    pub fn new(
        thread_pool: Rc<rayon::ThreadPool>,
        computation_done_notifier: Arc<ComputationDoneNotifier>,
    ) -> Activities {
        Activities {
            activities: HashMap::new(),
            possible_beginnings_updater: PossibleBeginningsUpdater::new(
                thread_pool,
                computation_done_notifier,
            ),
        }
    }

    /// Simple getter for the activity list, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Activity> {
        let mut activity_vec: Vec<&Activity> = self.activities.values().collect();
        activity_vec.sort_by(|a, b| a.name().cmp(&b.name()));
        activity_vec
    }

    /// Returns a copy of the activity with given id.
    #[must_use]
    pub fn get_by_id(&self, id: ActivityID) -> Result<Activity> {
        match self.activities.get(&id) {
            Some(activity) => Ok(activity.clone()),
            None => Err(DoesNotExist::activity_does_not_exist(id)),
        }
    }

    /// Simple private mutable getter for an activity.
    #[must_use]
    fn get_mut_by_id(&mut self, id: ActivityID) -> Result<&mut Activity> {
        match self.activities.get_mut(&id) {
            Some(activity) => Ok(activity),
            None => Err(DoesNotExist::activity_does_not_exist(id)),
        }
    }

    /// Adds an activity with the given name to the collection.
    /// Automatically assigns a unique id.
    /// Returns an immutable reference to the newly created activity.
    ///
    /// # Panics
    ///
    /// Panics if there is no id available under 65536.
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
    #[must_use]
    pub fn remove(&mut self, id: ActivityID) -> Result<()> {
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
    #[must_use]
    pub fn set_name(&mut self, id: ActivityID, name: String) -> Result<()> {
        Ok(self.get_mut_by_id(id)?.metadata.set_name(name))
    }

    /// Adds an entity to the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the entity is already
    /// taking part in the activity.
    #[must_use]
    pub fn add_entity(&mut self, id: ActivityID, entity: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.add_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Adds an entity in every activity which contains the given group.
    pub fn add_entity_to_activities_with_group(
        &mut self,
        group_name: &String,
        entity_name: String,
    ) {
        for activity in self
            .activities
            .values_mut()
            .filter(|activity| activity.groups_sorted().contains(group_name))
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
    #[must_use]
    pub fn remove_entity(&mut self, id: ActivityID, entity: &String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.remove_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
    }

    /// Removes the entity with given name from all activities.
    pub fn remove_entity_from_all(&mut self, entity: &String) {
        for activity in self.activities.values_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, that is what we want in the first place
            let _ = activity.metadata.remove_entity(entity);
        }
        self.update_incompatible_activities();
    }

    /// Renames the entity with given name in all activities.
    pub fn rename_entity_in_all(&mut self, old_name: &String, new_name: String) {
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
    #[must_use]
    pub fn add_group(&mut self, id: ActivityID, group_name: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.add_group(group_name)
    }

    /// Removes the group with the given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity si not found or if the group is not taking part in the
    /// activity.
    #[must_use]
    pub fn remove_group(&mut self, id: ActivityID, group_name: &String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.remove_group(group_name)
    }

    /// Removes the group with the given name from all activities.
    pub fn remove_group_from_all(&mut self, group: &String) {
        for activity in self.activities.values_mut() {
            // We don't care about the result: if the group is not in the activity, this
            // is what we want.
            let _ = activity.metadata.remove_group(group);
        }
        self.update_incompatible_activities();
    }

    /// Renames the group with given name in all activities.
    pub fn rename_group_in_all(&mut self, old_name: &String, new_name: String) {
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
    #[must_use]
    pub fn set_duration(&mut self, id: ActivityID, duration: Time) -> Result<()> {
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
    #[must_use]
    pub fn set_color(&mut self, id: ActivityID, color: RGBA) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.set_color(color);
        Ok(())
    }

    /// Triggers the computation of new possible beginnings for the given activities.
    pub fn must_update_possible_activity_beginnings(
        &mut self,
        schedules_of_participants: Vec<WorkHoursAndActivityDurationsSorted>,
        concerned_activity_ids: HashSet<ActivityID>,
    ) {
        self.possible_beginnings_updater
            .queue_work_hours_and_activity_durations(
                schedules_of_participants,
                concerned_activity_ids,
            );
    }

    /// Returns the possible beginnings of an activity if it is up to date or if
    /// the computation results are up.
    /// If neither is the case, returns None.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity with given id is not found.
    #[must_use]
    pub fn possible_insertion_times_of_activity(
        &mut self,
        schedules_of_participants: Vec<WorkHoursAndActivityDurationsSorted>,
        concerned_activity_id: ActivityID,
    ) -> Result<Option<HashSet<Time>>> {
        let concerned_activity = self.get_by_id(concerned_activity_id)?;

        let maybe_result = if self
            .possible_beginnings_updater
            .activity_beginnings_are_up_to_date(&concerned_activity_id)
        {
            // Result is up to date, no need to recalculate
            Some(
                concerned_activity
                    .computation_data
                    .possible_insertion_times_if_no_conflict()
                    .clone(),
            )
        } else {
            let maybe_result = self
                .possible_beginnings_updater
                .poll_and_fuse_possible_beginnings(schedules_of_participants, &concerned_activity);
            // TODO set_up_to_date as a separate function ??
            // The 'up to date' and 'result' values are separated. This is not good.

            if maybe_result.is_some() {
                // If the result is valid, store it into the activity computation data.
                let result = maybe_result
                    .clone()
                    .expect("Maybe result should be some but is not");
                let concerned_activity = self.get_mut_by_id(concerned_activity_id)?;
                concerned_activity
                    .computation_data
                    .set_possible_insertion_times_if_no_conflict(result);
            }
            maybe_result
        };

        let incompatible_activity_ids = concerned_activity
            .computation_data
            .incompatible_activity_ids();

        Ok(self.filter_insertion_times_for_conflicts(
            maybe_result,
            concerned_activity.duration(),
            incompatible_activity_ids,
        ))
    }

    /// Given the possible beginnings of an activity and the ids of incompatible activities,
    /// checks for conflicts and finally returns the real possible beginnings.
    #[must_use]
    fn filter_insertion_times_for_conflicts(
        &self,
        possible_beginnings: Option<HashSet<Time>>,
        activity_duration: Time,
        incompatible_activity_ids: Vec<ActivityID>,
    ) -> Option<HashSet<Time>> {
        // Offset with the duration of the activity
        // (e.g. if 11:00 - 12:00 is taken and our duration is 00:30, we cannot insert the activity
        // at 10:50.
        let offset_activity_duration = activity_duration - MIN_TIME_DISCRETIZATION;

        possible_beginnings.and_then(|mut possible_beginnings| {
            for incompatible_insertion_interval in incompatible_activity_ids
                .iter()
                .copied()
                .filter_map(|id| {
                    self.get_by_id(id)
                        .expect("Checking for conflict with invalid activity ID !")
                        .insertion_interval()
                })
                .filter(|interval| interval.beginning() > offset_activity_duration)
            {
                let mut current_time =
                    incompatible_insertion_interval.beginning() - offset_activity_duration;
                let end = incompatible_insertion_interval.end();

                while current_time < end {
                    possible_beginnings.remove(&current_time);
                    current_time.add_minutes(MIN_TIME_DISCRETIZATION_MINUTES as i8);
                }
            }
            Some(possible_beginnings)
        })
    }

    /// Inserts the activity with the given beginning.
    /// Checks are done by the Data module.
    ///
    /// # Errors
    /// Returns Err if the activity is not found.
    #[must_use]
    pub fn insert_activity(&mut self, id: ActivityID, beginning: Time) -> Result<()> {
        let activity = self.get_mut_by_id(id)?;
        activity.computation_data.insert(beginning);
        Ok(())
    }

    /// Returns data ready for auto-insertion of all activities.
    ///
    /// The ids of incompatible activities are turned into indexes.
    ///
    #[must_use]
    fn fetch_computation(&self) -> Vec<ActivityComputationData> {
        let activities = self.activities.values();
        let mut computation_data: Vec<ActivityComputationData> = activities
            .clone()
            .map(|activity| activity.computation_data.clone())
            .collect();

        let ids: Vec<_> = activities.map(|other| other.metadata.id()).collect();

        // Translate incompatible ids into incompatible indexes
        // This is not the most efficient but this operation is not critical,
        // the computation should be optimized, not this
        for data in &mut computation_data {
            let incompatible_ids = data.incompatible_activity_ids();
            let mut incompatible_indexes: Vec<ActivityID> =
                Vec::with_capacity(incompatible_ids.len());
            for (index_of_other, id_of_other) in ids.iter().enumerate() {
                // We don't care if we compare ourselves to ourselves,
                // we cannot be incompatible with ourselves
                if incompatible_ids.contains(&id_of_other) {
                    incompatible_indexes.push(index_of_other);
                }
            }
            data.set_incompatible_activity_ids(incompatible_indexes);
        }
        computation_data
    }
}

// Private, inner tests
#[cfg(test)]
mod tests {
    use super::super::super::super::Entities;
    use super::*;

    #[test]
    fn incompatible_ids() {
        let mut activity_collection = Activities::new(Rc::new(
            rayon::ThreadPoolBuilder::new()
                .build()
                .expect("Could not build rayon::ThreadPool"),
        ));
        let id_a = activity_collection.add("a".to_owned()).id();
        let id_b = activity_collection.add("b".to_owned()).id();

        let mut entities = Entities::new();
        let entity_a = "A".to_owned();
        let entity_b = "B".to_owned();
        entities
            .add(entity_a.clone())
            .expect("Could not add entity");
        entities
            .add(entity_b.clone())
            .expect("Could not add entity");

        // Insert the same entity in both activities
        activity_collection
            .add_entity(id_a, entity_a.clone())
            .expect("Could not add entity to activity");
        activity_collection
            .add_entity(id_b, entity_a.clone())
            .expect("Could not add entity to activity");

        // At this point : id_a contains {a}, id_b contains {a}
        let incompatible_a = activity_collection
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activity_collection
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_b.len(), 1);
        assert_eq!(incompatible_a[0], id_b);
        assert_eq!(incompatible_b[0], id_a);

        // Remove the entity in one activity
        activity_collection
            .remove_entity(id_a, &entity_a)
            .expect("Could not remove entity from activity");

        // At this point : id_a contains {}, id_b contains {a}
        let incompatible_a = activity_collection
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activity_collection
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 0);
        assert_eq!(incompatible_b.len(), 0);

        // Add non-confictual entity
        activity_collection
            .add_entity(id_a, entity_b.clone())
            .expect("Could not add entity to activity");

        // At this point : id_a contains {b}, id_b contains {a}
        let incompatible_a = activity_collection
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activity_collection
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 0);
        assert_eq!(incompatible_b.len(), 0);

        // Add conflictual entity again
        activity_collection
            .add_entity(id_b, entity_b)
            .expect("Could not add entity to activity");

        // At this point : id_a contains {b}, id_b contains {a, b}
        let incompatible_a = activity_collection
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activity_collection
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_b.len(), 1);
        assert_eq!(incompatible_a[0], id_b);
        assert_eq!(incompatible_b[0], id_a);

        // Add third activity
        let id_c = activity_collection.add("c".to_owned()).id();
        activity_collection
            .add_entity(id_c, entity_a)
            .expect("Could not add entity to activity");

        // At this point : id_a contains {b}, id_b contains {a, b}, id_c contains {a}
        let incompatible_a = activity_collection
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activity_collection
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_c = activity_collection
            .get_by_id(id_c)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_a[0], id_b);

        assert_eq!(incompatible_b.len(), 2);
        assert!(incompatible_b.contains(&id_a));
        assert!(incompatible_b.contains(&id_c));

        assert_eq!(incompatible_c.len(), 1);
        assert!(incompatible_c.contains(&id_b));
    }

    #[test]
    fn test_fetch_computation() {
        let mut activity_collection = Activities::new(Rc::new(
            rayon::ThreadPoolBuilder::new()
                .build()
                .expect("Could not build rayon::ThreadPool"),
        ));
        activity_collection.add("0".to_owned());
        activity_collection.add("1".to_owned());
        activity_collection.add("2".to_owned());
        activity_collection.add("3".to_owned());
        activity_collection
            .remove(2)
            .expect("Could not remove activity");

        // Ids are [0, 1, 3]
        activity_collection
            .get_mut_by_id(0)
            .expect("Could not get activity by id")
            .computation_data
            .set_incompatible_activity_ids(vec![3]);
        activity_collection
            .get_mut_by_id(1)
            .expect("Could not get activity by id")
            .computation_data
            .set_incompatible_activity_ids(vec![0, 3]);
        activity_collection
            .get_mut_by_id(3)
            .expect("Could not get activity by id")
            .computation_data
            .set_incompatible_activity_ids(vec![1]);

        let activities: Vec<Activity> = activity_collection.activities.values().cloned().collect();
        // Assuming activities.values() returns the same order twice
        // (activities.values() called in fetch_computation)
        let computation_data: Vec<ActivityComputationData> =
            activity_collection.fetch_computation();

        for (activity, computation) in activities.iter().zip(computation_data) {
            let mut ids = activity.computation_data.incompatible_activity_ids();
            let mut ids_from_indexes = computation
                .incompatible_activity_ids()
                .iter()
                .map(|&index| activities[index].id())
                .collect::<Vec<ActivityID>>();
            ids.sort();
            ids_from_indexes.sort();
            assert_eq!(ids, ids_from_indexes);
        }
    }
}

impl Clone for Activities {
    fn clone(&self) -> Self {
        Activities {
            activities: self.activities.clone(),
            possible_beginnings_updater: PossibleBeginningsUpdater::new(
                Rc::new(
                    rayon::ThreadPoolBuilder::new()
                        .build()
                        .expect("Could not build rayon::ThreadPool"),
                ),
                Arc::new(ComputationDoneNotifier::new()),
            ),
        }
    }
}

impl PartialEq for Activities {
    fn eq(&self, other: &Self) -> bool {
        self.activities == other.activities
    }
}
