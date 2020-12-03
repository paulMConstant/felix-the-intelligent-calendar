use super::helpers::clean_string::clean;
use crate::data::{Activity, Data, Time};
use std::collections::HashSet;

/// Operations on activities
impl Data {
    /// Returns the activities, sorted by name.
    #[must_use]
    pub fn activities_sorted(&self) -> Vec<&Activity> {
        self.activities.sorted_by_name()
    }

    /// Returns an immutable reference to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub fn activity(&self, id: u16) -> Result<&Activity, String> {
        self.activities.get_by_id(id)
    }

    /// Adds an activity with the formatted given name.
    ///
    /// Automatically assigns a unique id.
    /// Returns an immutable reference to the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_name = "Activity";
    /// let activity_id = data.add_activity(activity_name).unwrap().id();
    ///
    /// let activities = data.activities_sorted();
    /// assert_eq!(activities.len(), 1);
    /// assert_eq!(activities[0].id(), activity_id);
    /// ```
    #[must_use]
    pub fn add_activity<S>(&mut self, name: S) -> Result<&Activity, String>
    where
        S: Into<String>,
    {
        Ok(self.activities.add(clean(name)?))
    }

    /// Removes the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let invalid_id = activity_id + 1;
    ///
    /// assert!(data.remove_activity(invalid_id).is_err());
    /// assert_eq!(data.activities_sorted().len(), 1);
    /// assert!(data.remove_activity(activity_id).is_ok());
    /// assert!(data.activities_sorted().is_empty());
    /// ```
    #[must_use]
    pub fn remove_activity(&mut self, id: u16) -> Result<(), String> {
        self.activities.remove(id)
    }

    /// Adds the entity with given name to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the entity is not found,
    /// if the entity does not have enough time left
    /// or the entity is already taking part in the activity.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity_name = data.add_entity("Bernard").unwrap().name();
    ///
    /// // Make sure the entity has enough time !
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(morning_shift).unwrap();
    ///
    /// assert!(data.add_entity_to_activity(activity_id, entity_name.clone()).is_ok());
    ///
    /// let entities = data.activity(activity_id).unwrap().entities_sorted();
    /// assert_eq!(entities.len(), 1);
    /// assert_eq!(entities[0], entity_name);
    /// ```
    #[must_use]
    pub fn add_entity_to_activity<S>(&mut self, id: u16, entity_name: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        let entity_name = clean(entity_name)?;
        if self.has_enough_time_for_activity(id, &entity_name)? {
            // Add the entity to the activity
            self.activities.add_entity(id, entity_name)
        } else {
            Err(format!(
                "{} does not have enough time left for this activity.",
                entity_name
            ))
        }
    }

    /// Removes the entity with given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the entity is not found,
    /// if the name is empty or the entity is not taking part in the activity.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity_name = data.add_entity("Bernard").unwrap().name();
    ///
    /// // Make sure the entity has enough time before adding him to an activity
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(entity_name.clone(), morning_shift).unwrap();
    ///
    /// data.add_entity_to_activity(activity_id, entity_name.clone()).unwrap();
    /// assert!(data.remove_entity_from_activity(activity_id, entity_name).is_ok());
    /// assert!(data.activity(activity_id).unwrap().entities_sorted().is_empty());
    /// ```
    #[must_use]
    pub fn remove_entity_from_activity<S>(&mut self, id: u16, entity_name: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the entity exists and get it formatted
        let entity_name = self.entity(entity_name)?.name();
        // Remove the entity from the activity
        self.activities.remove_entity(id, &entity_name)
    }

    /// Adds the group with the formatted given name to the activity with the given id.
    ///
    /// Any activity currently in the group will be added to the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the group is not found,
    /// if the name is empty or if the group is already taking part in the activity.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let id = data.add_activity("Activity").unwrap().id();
    /// let group_name = data.add_group("Group").unwrap();
    ///
    /// data.add_group_to_activity(id, group_name.clone()).unwrap();
    /// let groups = data.activity(id).unwrap().groups_sorted();
    /// assert_eq!(groups[0], group_name);
    /// ```
    #[must_use]
    pub fn add_group_to_activity<S>(&mut self, id: u16, group_name: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the group exists and get name formatted
        let group = self.group(group_name)?;
        // Fetch group and entities here as copies (dropping group reference for borrow checker)
        let entities: Vec<String> = group.entities_sorted().into_iter().cloned().collect();
        let group_name = group.name();

        // Check that each entity has enough time
        for entity_name in &entities {
            if self.has_enough_time_for_activity(id, entity_name)? == false {
                return Err(format!(
                    "'{}' does not have enough time left for this activity.",
                    entity_name
                ));
            }
        }

        // Add each entity in the group to the activity.
        // We do not care about the result: if the entity is already in the activity,
        // it is fine.
        for entity_name in entities {
            let _ = self.activities.add_entity(id, entity_name);
        }

        // Add the group to the activity
        self.activities.add_group(id, group_name)
        // TODO update possible insertion times
    }

    /// Removes the group with the formatted given name from the activity with the given id.
    ///
    /// The group will be removed from the activities.
    /// Any entity participating in activities only through this group will be removed from the
    /// activities.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the group is not found,
    /// if the name is empty or if the group is not taking part in the activity.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let id = data.add_activity("Activity").unwrap().id();
    /// let group_name = data.add_group("Group").unwrap();
    /// data.add_group_to_activity(id, group_name.clone()).unwrap();
    ///
    /// data.remove_group_from_activity(id, group_name.clone()).unwrap();
    /// let groups = data.activity(id).unwrap().groups_sorted();
    /// assert!(groups.is_empty());
    /// ```
    #[must_use]
    pub fn remove_group_from_activity<S>(&mut self, id: u16, group_name: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the group exists and get name formatted
        let group = self.group(group_name)?;
        // Fetch group and entities here as copies (dropping group reference for borrow checker)
        let entities: Vec<String> = group.entities_sorted().into_iter().cloned().collect();
        let group_name = group.name();

        // TODO create new file for helpers
        let entities_participating_through_other_groups = self
            .activity(id)?
            .groups_sorted()
            .iter()
            .filter(|&other_group_name| other_group_name != &group_name)
            .flat_map(|group_name|
                // Expect is safe to use here: we are sure that the activtiy contains valid groups
                self.group(group_name).expect("Could not get group which is in an activity").entities_sorted())
            .cloned()
            .collect::<HashSet<String>>();

        let entities_to_remove = entities.iter().filter(|entity_name| {
            entities_participating_through_other_groups.contains(*entity_name) == false
        });

        // Remove any entity which is not participanting through other groups from the activity
        for entity_name in entities_to_remove {
            self.activities.remove_entity(id, entity_name)?;
        }

        // Remove the group from the activity
        self.activities.remove_group(id, &group_name)
        // TODO update possible insertion times
    }

    /// Sets the name of the activity with given id with the formatted given name.
    ///
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    ///
    /// // new_name is formatted from "New name" to "New Name"
    /// let new_name = data.set_activity_name(activity_id, "New name").unwrap();
    /// assert_eq!(data.activity(activity_id).unwrap().name(), new_name);
    /// ```
    #[must_use]
    pub fn set_activity_name<S>(&mut self, id: u16, name: S) -> Result<String, String>
    where
        S: Into<String>,
    {
        let name = clean(name)?;
        self.activities.set_name(id, name.clone())?;
        Ok(name)
    }

    /// Sets the duration of the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or the duration is too short
    /// (< MIN\_TIME\_DISCRETIZATION) or an entity does not have enough time left.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, MIN_TIME_DISCRETIZATION};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let min_valid_duration = MIN_TIME_DISCRETIZATION;
    ///
    /// assert!(data.set_activity_duration(activity_id, min_valid_duration).is_ok());
    /// assert_eq!(data.activity(activity_id).unwrap().duration(), min_valid_duration);
    /// ```
    #[must_use]
    pub fn set_activity_duration(&mut self, id: u16, new_duration: Time) -> Result<(), String> {
        // If the duration is longer than the previous one, check for conflicts
        let activity = self.activity(id)?;
        let current_duration = activity.duration();
        if new_duration > current_duration {
            // Duration is longer - check if it conflicts with entity's schedule
            let required_free_time = new_duration - current_duration; // > 0
            if let Some(entity_name) = activity
                .entities_sorted()
                .iter()
                // Call to expect() : we are sure that all entities in the activity exist.
                .find(|entity_name| {
                    self.free_time_of(entity_name.clone())
                        .expect("Could not get entity participating in an activity")
                        < required_free_time
                })
            {
                return Err(format!(
                    "{} does not have enough time for the new duration.",
                    entity_name
                ));
            }
        };
        self.activities.set_duration(id, new_duration)
    }
}
