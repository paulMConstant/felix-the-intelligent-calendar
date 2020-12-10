use super::computation::id_computation::{compute_incompatible_ids, generate_next_id};
use super::ActivityMetadata;
use crate::data::{Activity, Time};
use crate::errors::{does_not_exist::DoesNotExist, Result};
use std::collections::HashMap;

/// Manages the collection of activities.
/// Makes sures there are no id duplicates.
#[derive(Clone, Debug, PartialEq, Eq)]
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

    /// Simple getter for the activity list, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Activity> {
        let mut activity_vec: Vec<&Activity> = self.activities.values().collect();
        activity_vec.sort_by(|a, b| a.name().cmp(&b.name()));
        activity_vec
    }

    /// Simple getter for an activity.
    #[must_use]
    pub fn get_by_id(&self, id: u16) -> Result<&Activity> {
        match self.activities.get(&id) {
            Some(activity) => Ok(activity),
            None => Err(DoesNotExist::activity_does_not_exist(id)),
        }
    }

    /// Simple private mutable getter for an activity.
    #[must_use]
    fn get_mut_by_id(&mut self, id: u16) -> Result<&mut Activity> {
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
    #[must_use]
    pub fn add(&mut self, name: String) -> &Activity {
        let used_ids = self.activities.keys().collect();
        let id = generate_next_id(used_ids);
        let activity = Activity::new(id, name);

        self.activities.insert(id, activity);
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
    pub fn remove(&mut self, id: u16) -> Result<()> {
        match self.activities.remove(&id) {
            None => Err(DoesNotExist::activity_does_not_exist(id)),
            Some(_) => {
                self.update_incompatible_activities();
                // TODO update possible insertion times
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
    pub fn set_name(&mut self, id: u16, name: String) -> Result<()> {
        Ok(self.get_mut_by_id(id)?.metadata.set_name(name))
    }

    /// Adds an entity to the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the entity is already
    /// taking part in the activity.
    #[must_use]
    pub fn add_entity(&mut self, id: u16, entity: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.add_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
        // TODO update possible insertion times
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
    pub fn remove_entity(&mut self, id: u16, entity: &String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.remove_entity(entity)?;
        self.update_incompatible_activities();
        Ok(())
        // TODO update possible insertion times
    }

    /// Removes the entity with given name from all activities.
    pub fn remove_entity_from_all(&mut self, entity: &String) {
        for activity in self.activities.values_mut() {
            // We don't care about the result : if the entity is not
            // taking part in the activity, that is what we want in the first place
            let _ = activity.metadata.remove_entity(entity);
        }
        self.update_incompatible_activities();
        // TODO update possible insertion times
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
    pub fn add_group(&mut self, id: u16, group_name: String) -> Result<()> {
        self.get_mut_by_id(id)?.metadata.add_group(group_name)
    }

    /// Removes the group with the given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity si not found or if the group is not taking part in the
    /// activity.
    #[must_use]
    pub fn remove_group(&mut self, id: u16, group_name: &String) -> Result<()> {
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
    pub fn set_duration(&mut self, id: u16, duration: Time) -> Result<()> {
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
        let id_a = activities.add("a".to_owned()).id();
        let id_b = activities.add("b".to_owned()).id();

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
        activities
            .add_entity(id_a, entity_a.clone())
            .expect("Could not add entity to activity");
        activities
            .add_entity(id_b, entity_a.clone())
            .expect("Could not add entity to activity");

        // At this point : id_a contains {a}, id_b contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_b.len(), 1);
        assert_eq!(incompatible_a[0], id_b);
        assert_eq!(incompatible_b[0], id_a);

        // Remove the entity in one activity
        activities
            .remove_entity(id_a, &entity_a)
            .expect("Could not remove entity from activity");

        // At this point : id_a contains {}, id_b contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 0);
        assert_eq!(incompatible_b.len(), 0);

        // Add non-confictual entity
        activities
            .add_entity(id_a, entity_b.clone())
            .expect("Could not add entity to activity");

        // At this point : id_a contains {b}, id_b contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 0);
        assert_eq!(incompatible_b.len(), 0);

        // Add conflictual entity again
        activities
            .add_entity(id_b, entity_b)
            .expect("Could not add entity to activity");

        // At this point : id_a contains {b}, id_b contains {a, b}
        let incompatible_a = activities
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        assert_eq!(incompatible_a.len(), 1);
        assert_eq!(incompatible_b.len(), 1);
        assert_eq!(incompatible_a[0], id_b);
        assert_eq!(incompatible_b[0], id_a);

        // Add third activity
        let id_c = activities.add("c".to_owned()).id();
        activities
            .add_entity(id_c, entity_a)
            .expect("Could not add entity to activity");

        // At this point : id_a contains {b}, id_b contains {a, b}, id_c contains {a}
        let incompatible_a = activities
            .get_by_id(id_a)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_b = activities
            .get_by_id(id_b)
            .expect("Could not get activity by id")
            .computation_data
            .incompatible_activity_ids();
        let incompatible_c = activities
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
}
