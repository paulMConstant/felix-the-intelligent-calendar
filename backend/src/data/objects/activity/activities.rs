use super::super::super::helpers::clean_string::clean;
use super::{Activity, ActivityMetadata, Time};
use std::collections::HashMap;
use std::convert::TryFrom;

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
        let id = self.generate_next_id();
        let activity = Activity::new(id, name);

        self.activities.insert(id, activity);
        Ok(&self.activities.get(&id).unwrap())
    }

    /// Generates the smallest unused id.
    ///
    /// # Panics
    ///
    /// Panics if there is no id available under 65536.
    #[must_use]
    fn generate_next_id(&self) -> u16 {
        // Fetch the ids in ascending order.
        let mut used_ids: Vec<&u16> = self.activities.keys().collect();
        used_ids.sort();

        // If 0 is unused, assign it.
        if used_ids.is_empty() || *used_ids[0] != 0 {
            0
        } else {
            // Compute the difference between neighbours to check for the first hole
            // Example : [0, 1, 2, 4, 5] -> [1, 1, 2, 1] -> tab[2] > 1 : 3 is the hole to fill
            if let Some(index) = used_ids.windows(2).map(|w| w[1] - w[0]).position(|i| i > 1) {
                // Found a hole ! Return its index + 1.
                match u16::try_from(index + 1) {
                    Ok(i) => i,
                    Err(_) => panic!("All 65536 ids have been used !"),
                }
            } else {
                // Hole not found : return the length of the used ids.
                match u16::try_from(used_ids.len()) {
                    Ok(i) => i,
                    Err(_) => panic!("All 65536 ids have been used !"),
                }
            }
        }
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
                .set_incompatible_activity_ids(
                    metadata_vec
                        .iter()
                        .filter(|other_metadata| {
                            metadata.id() != other_metadata.id()
                                && metadata
                                    .participants_as_set()
                                    .intersection(other_metadata.participants_as_set())
                                    .next()
                                    != None
                        })
                        .map(|other_metadata| other_metadata.id())
                        .collect(),
                );
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

#[cfg(test)]
mod tests {
    use super::super::super::super::Entities;
    use super::super::super::time::MIN_TIME_DISCRETIZATION;
    use super::*;

    #[test]
    fn add_remove_activity() {
        let mut activities = Activities::new();

        // Add activities
        let name = "Meeting";
        assert!(activities.add(name).is_ok());
        assert!(activities.add(name).is_ok());
        assert!(activities.add(name).is_ok());

        // Check ID incrementation OK
        let ids: Vec<u16> = activities
            .sorted_by_name()
            .iter()
            .map(|activity| activity.id())
            .collect();
        for id in 0..2 {
            assert!(ids.contains(&id));
        }

        // Check removal
        assert!(activities.remove(1).is_ok());
        let ids: Vec<u16> = activities
            .sorted_by_name()
            .iter()
            .map(|activity| activity.id())
            .collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&0));
        assert!(ids.contains(&2));
        assert_eq!(ids.contains(&1), false);

        // Add new activity and check id.
        // The ids are never sorted.
        assert!(activities.add(name).is_ok());
        let ids: Vec<u16> = activities
            .sorted_by_name()
            .iter()
            .map(|activity| activity.id())
            .collect();
        for id in 0..2 {
            assert!(ids.contains(&id));
        }
    }

    #[test]
    fn add_remove_participants() {
        let mut activities = Activities::new();
        let mut entities = Entities::new();

        let name = "Meeting";
        let activity = activities.add(name).unwrap();
        let id = activity.id();

        // Add participant
        let participant = entities.add("Z Participant").unwrap().name();
        assert!(activities.add_participant(id, participant.clone()).is_ok());
        let res = &activities.get_by_id(id).unwrap().participants_sorted()[0];
        assert_eq!(*res, participant);

        // Add the same participant to the same activtiy
        assert!(activities.add_participant(id, participant.clone()).is_err());
        assert_eq!(
            activities
                .get_by_id(id)
                .unwrap()
                .participants_sorted()
                .len(),
            1
        );

        // Add participant to invalid activity
        let invalid_id = id + 1;
        assert!(activities
            .add_participant(invalid_id, "New participant")
            .is_err());

        // Remove invalid participant
        let new_participant = entities.add("A New participant").unwrap().name();
        assert!(activities
            .remove_participant(id, new_participant.clone())
            .is_err());

        // Add new participant && check sorting
        assert!(activities
            .add_participant(id, new_participant.clone())
            .is_ok());
        let participants = activities.get_by_id(id).unwrap().participants_sorted();
        assert_eq!(participants[0], new_participant);
        assert_eq!(participants[1], participant);

        // Remove valid participant
        assert!(activities.remove_participant(id, participant).is_ok());
        assert_eq!(
            activities
                .get_by_id(id)
                .unwrap()
                .participants_sorted()
                .len(),
            1
        );
    }

    #[test]
    fn update_participants_in_all() {
        let mut activities = Activities::new();
        let mut entities = Entities::new();

        let id1 = activities.add("1").unwrap().id();
        let id2 = activities.add("2").unwrap().id();
        let name1 = entities.add("Entity 1").unwrap().name();
        let name2 = entities.add("Entity 2").unwrap().name();

        // Add participants
        activities.add_participant(id1, name1.clone()).unwrap();
        activities.add_participant(id1, name2.clone()).unwrap();
        activities.add_participant(id2, name1.clone()).unwrap();
        activities.add_participant(id2, name2.clone()).unwrap();

        // Rename participant in all
        let new_name = "0";
        assert!(activities
            .rename_participant_in_all(name1, new_name)
            .is_ok());
        let participants1 = activities.get_by_id(id1).unwrap().participants_sorted();
        let participants2 = activities.get_by_id(id2).unwrap().participants_sorted();
        assert_eq!(participants1[0], new_name);
        assert_eq!(participants1[1], name2);
        assert_eq!(participants2[0], new_name);
        assert_eq!(participants2[1], name2);

        // Rename participant with invalid name from all
        assert!(activities
            .rename_participant_in_all("  ", new_name.clone())
            .is_err());
        assert!(activities
            .rename_participant_in_all(new_name.clone(), "\t")
            .is_err());

        // Remove participant from all
        assert!(activities.remove_participant_from_all(new_name).is_ok());
        assert_eq!(
            activities
                .get_by_id(id1)
                .unwrap()
                .participants_sorted()
                .len(),
            1
        );
        assert_eq!(
            activities
                .get_by_id(id2)
                .unwrap()
                .participants_sorted()
                .len(),
            1
        );

        // Remove participant with invalid name from all
        assert!(activities.remove_participant_from_all("  ").is_err());
        // Remove participant with valid name but participates in none - this
        // has no effect but is ok
        assert!(activities.remove_participant_from_all("Valid name").is_ok());
    }

    #[test]
    fn set_activity_name() {
        let mut activities = Activities::new();
        let name = clean("Meeting").unwrap();

        let activity = activities.add(name.clone()).unwrap();
        let id = activity.id();
        let activity_name = activity.name();

        // Set invalid name
        assert!(activities.set_name(id, " \t").is_err());

        // Set valid name with invalid id
        let new_name = "New meeting";
        let invalid_id = id + 1;
        assert!(activities.set_name(invalid_id, new_name).is_err());

        // Original name still there
        assert_eq!(activity_name, name);

        // Set valid name activity
        assert!(activities.set_name(id, new_name).is_ok());

        // Check name formatting
        let activity_name = activities.get_by_id(id).unwrap().name();
        assert_ne!(activity_name, new_name);
        assert_eq!(activity_name, clean(new_name).unwrap());

        // Try renaming activity with invalid ID
        let invalid_id = 3;
        assert!(activities.set_name(invalid_id, name).is_err());
    }

    #[test]
    fn set_activity_duration() {
        let mut activities = Activities::new();
        let id = activities.add("Meeting").unwrap().id();

        // Set wrong duration
        let invalid_duration = Time::new(0, 0);
        assert!(activities.set_duration(id, invalid_duration).is_err());

        // Set duration with invalid id
        let invalid_id = id + 1;
        let valid_duration = Time::new(1, 30);
        assert!(activities.set_duration(invalid_id, valid_duration).is_err());
        assert_eq!(
            activities.get_by_id(id).unwrap().duration(),
            MIN_TIME_DISCRETIZATION
        );

        // Set duration of activity
        assert!(activities.set_duration(id, valid_duration).is_ok());
        assert_eq!(activities.get_by_id(id).unwrap().duration(), valid_duration);
    }

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
