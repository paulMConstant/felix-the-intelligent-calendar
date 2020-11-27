use super::super::helpers::clean_string::clean;
use super::helpers::id_computation::{compute_incompatible_ids, generate_next_id};
use super::ActivityMetadata;
use crate::data::{Activity, Time};
use std::collections::HashMap;

/// Manages the collection of activities.
/// Makes sures there are no id duplicates.
pub struct Activities {
    activities: HashMap<u16, Activity>,
}

impl Activities {
    /// Initializes the Activity collection.
    #[must_use]
    pub fn new() -> Activities {
        Activities {
            activities: HashMap::new(),
        }
    }

    // Code organization :
    // - Getter for collection
    // - Getter for individual element
    // - Add
    // - Remove
    // - Modify

    /// Simple getter for the activity list, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Activity> {
        let mut activity_vec: Vec<&Activity> = self.activities.values().collect();
        activity_vec.sort_by(|a, b| a.name().cmp(&b.name()));
        activity_vec
    }

    /// Simple getter for an activity.
    #[must_use]
    pub fn get_by_id(&self, id: u16) -> Result<&Activity, String> {
        match self.activities.get(&id) {
            Some(activity) => Ok(activity),
            None => Err(format!("Cannot get activity with id {}.", id)),
        }
    }

    /// Simple private mutable getter for an activity.
    #[must_use]
    fn get_mut_by_id(&mut self, id: u16) -> Result<&mut Activity, String> {
        match self.activities.get_mut(&id) {
            Some(activity) => Ok(activity),
            None => Err(format!("Cannot get activity with id {}.", id)),
        }
    }

    /// Adds an activity with the formatted given name to the collection.
    /// Automatically assigns a unique id.
    /// Returns an immutable reference to the newly created activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted given name is empty.
    ///
    /// # Panics
    ///
    /// Panics if there is no id available under 65536.
    #[must_use]
    pub fn add<S>(&mut self, name: S) -> Result<&Activity, String>
    where
        S: Into<String>,
    {
        let name = clean(name)?;
        let used_ids = self.activities.keys().collect();
        let id = generate_next_id(used_ids);
        let activity = Activity::new(id, name);

        self.activities.insert(id, activity);
        Ok(&self.activities.get(&id).unwrap())
    }

    /// Removes the activity with given id from the collection.
    ///
    /// # Errors
    ///
    /// Returns Err if there is no activity with the given id.
    #[must_use]
    pub fn remove(&mut self, id: u16) -> Result<(), String> {
        match self.activities.remove(&id) {
            None => Err(format!("The activity with id {} does not exist !", id)),
            Some(_) => {
                self.update_incompatible_activities();
                // TODO update possible insertion times
                Ok(())
            }
        }
    }

    /// Changes the name of the activity with the given id to the formatted given name.
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if there is no activity with the given id, or the formatted given name is
    /// empty.
    #[must_use]
    pub fn set_name<S>(&mut self, id: u16, name: S) -> Result<String, String>
    where
        S: Into<String>,
    {
        let activity = self.get_mut_by_id(id)?;
        let name = clean(name)?;
        activity.metadata.set_name(name.clone());
        Ok(name)
    }

    /// Adds a participant to the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the participant is already
    /// taking part in the activity.
    #[must_use]
    pub fn add_participant<S>(&mut self, id: u16, participant: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.get_mut_by_id(id)?
            .metadata
            .add_participant(participant)?;
        self.update_incompatible_activities();
        Ok(())
        // TODO update possible insertion times
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
        // have at least one participant in common are incompatible).
        // If the activity has the same id, it is the same activity, don't add it
        for metadata in &metadata_vec {
            self.activities
                .get_mut(&metadata.id())
                .unwrap()
                .computation_data
                .set_incompatible_activity_ids(compute_incompatible_ids(&metadata, &metadata_vec));
        }
    }

    /// Removes a participant from the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the participant is not
    /// taking part in the activtiy.
    #[must_use]
    pub fn remove_participant<S>(&mut self, id: u16, participant: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.get_mut_by_id(id)?
            .metadata
            .remove_participant(participant)?;
        self.update_incompatible_activities();
        Ok(())
        // TODO update possible insertion times
    }

    /// Removes the participant with formatted given name from all activities.
    ///
    /// Returns the name of removed participant.
    ///
    /// # Errors
    ///
    /// Returns Err if the given name is empty after formatting.
    #[must_use]
    pub fn remove_participant_from_all<S>(&mut self, participant: S) -> Result<String, String>
    where
        S: Into<String>,
    {
        let participant = clean(participant)?;
        for activity in self.activities.values_mut() {
            // We don't care about the result : it is fine if the participant is not
            // taking part in the activity, that is what we want in the first place
            let _ = activity.metadata.remove_participant(participant.clone());
        }
        self.update_incompatible_activities();
        Ok(participant)
        // TODO update possible insertion times
    }

    /// Renames the participant with formatted given name in all activities.
    ///
    /// Returns the formatted new name of the participant.
    ///
    /// # Errors
    ///
    /// Returns Err if any given name is empty after formatting.
    #[must_use]
    pub fn rename_participant_in_all<S1, S2>(
        &mut self,
        old_name: S1,
        new_name: S2,
    ) -> Result<String, String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let old_name = clean(old_name)?;
        let new_name = clean(new_name)?;
        for activity in self.activities.values_mut() {
            // We don't care about the result : it is fine if the participant is not
            // taking part in the activity, this will yield no conflict when it is renamed
            let _ = activity
                .metadata
                .rename_participant(old_name.clone(), new_name.clone());
        }
        Ok(new_name)
    }

    /// Sets the duration of the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the duration is too short
    /// (< MIN\_TIME\_DISCRETIZATION).
    #[must_use]
    pub fn set_duration(&mut self, id: u16, duration: Time) -> Result<(), String> {
        self.get_mut_by_id(id)?
            .computation_data
            .set_duration(duration)
        // TODO update possible insertion times
    }

    // TODO
    // * Make a copy of the computation data in a vector
    // * Turn the incompatible ids into incompatible indexes
    // * Return the result for computation
    // fn fetch_computation(&self) -> Vec<ActivityComputationData>
}

// Private, inner tests
#[cfg(test)]
mod tests {
    use super::super::super::super::Entities;
    use super::*;

    #[test]
    fn incompatible_ids() {
        let mut activities = Activities::new();
        let id_a = activities.add("a").unwrap().id();
        let id_b = activities.add("b").unwrap().id();

        let mut entities = Entities::new();
        let entity_a = entities.add("a").unwrap().name();
        let entity_b = entities.add("b").unwrap().name();

        // Insert the same entity in both activities
        activities.add_participant(id_a, entity_a.clone()).unwrap();
        activities.add_participant(id_b, entity_a.clone()).unwrap();

        // At this point : id_a contains {a}, id_b contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_b.len(), 1);
        assert_eq!(incompatible_a[0], id_b);
        assert_eq!(incompatible_b[0], id_a);

        // Remove the entity in one activity
        activities
            .remove_participant(id_a, entity_a.clone())
            .unwrap();

        // At this point : id_a contains {}, id_b contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 0);
        assert_eq!(incompatible_b.len(), 0);

        // Add non-confictual entity
        activities.add_participant(id_a, entity_b.clone()).unwrap();

        // At this point : id_a contains {b}, id_b contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 0);
        assert_eq!(incompatible_b.len(), 0);

        // Add conflictual entity again
        activities.add_participant(id_b, entity_b).unwrap();

        // At this point : id_a contains {b}, id_b contains {a, b}
        let incompatible_a = activities
            .get_by_id(id_a)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_b.len(), 1);
        assert_eq!(incompatible_a[0], id_b);
        assert_eq!(incompatible_b[0], id_a);

        // Add third activity
        let id_c = activities.add("c").unwrap().id();
        activities.add_participant(id_c, entity_a).unwrap();

        // At this point : id_a contains {b}, id_b contains {a, b}, id_c contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .unwrap()
            .computation_data
            .incompatible_activity_ids();
        let incompatible_c = activities
            .get_by_id(id_c)
            .unwrap()
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
}
