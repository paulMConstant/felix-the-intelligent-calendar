mod error_checks;
mod inner;

use super::helpers::clean_string;
use crate::data::{Activity, ActivityID, Data, Time};
use crate::errors::Result;

/// Operations on activities
impl Data {
    /// Returns the activities, sorted by name.
    #[must_use]
    pub fn activities_sorted(&self) -> Vec<&Activity> {
        self.activities.sorted_by_name()
    }

    /// Returns an copy of the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub fn activity(&self, id: ActivityID) -> Result<Activity> {
        self.activities.get_by_id(id)
    }

    /// Returns the activities in which the given entity participates.
    #[must_use]
    pub fn activities_of<S>(&self, entity_name: S) -> Result<Vec<&Activity>>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        Ok(self
            .activities_sorted()
            .iter()
            .cloned()
            .filter(|activity| activity.entities_sorted().contains(&entity_name))
            .collect())
    }

    /// Adds an activity with the formatted given name.
    ///
    /// Automatically assigns a unique id.
    /// Returns a copy of the created activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use felix_backend::data::Data;
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
    pub fn add_activity<S>(&mut self, name: S) -> Result<Activity>
    where
        S: Into<String>,
    {
        let activity_id = self.activities.add(clean_string(name)?).id();
        let activity = self.activity(activity_id)?;
        self.events()
            .borrow_mut()
            .emit_activity_added(self, &activity);
        // No update of possible beginnings necessary
        Ok(activity)
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
    /// # use felix_backend::data::Data;
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
    pub fn remove_activity(&mut self, id: ActivityID) -> Result<()> {
        let position_of_removed_activity = self
            .activities_sorted()
            .into_iter()
            .position(|activity| activity.id() == id);
        self.activities.remove(id)?;

        // TODO for each entity in the activity, queue their work hours and invalidate all of their activities
        let position_of_removed_activity = position_of_removed_activity.expect(
            "If the activity was removed then it existed, therefore position should be valid",
        );
        self.events()
            .borrow_mut()
            .emit_activity_removed(self, position_of_removed_activity);
        Ok(())
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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity_name = data.add_entity("Bernard").unwrap();
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
    pub fn add_entity_to_activity<S>(&mut self, id: ActivityID, entity_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        self.check_has_enough_time_for_activity(id, &entity_name)?;
        self.activities.add_entity(id, entity_name)?;
        // TODO queue this entity and invalidate each of its activities
        self.events()
            .borrow_mut()
            .emit_entity_added_to_activity(self, &self.activity(id)?);
        Ok(())
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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity_name = data.add_entity("Bernard").unwrap();
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
    pub fn remove_entity_from_activity<S>(&mut self, id: ActivityID, entity_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // Check that the entity exists and get it formatted
        let entity_name = self.entity(entity_name)?.name();
        // Remove the entity from the activity
        self.activities.remove_entity(id, &entity_name)?;
        // TODO queue this entity and invalidate each of its activities
        self.events()
            .borrow_mut()
            .emit_entity_removed_from_activity(self, &self.activity(id)?);
        Ok(())
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
    /// # use felix_backend::data::Data;
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
    pub fn add_group_to_activity<S>(&mut self, id: ActivityID, group_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // Check that the group exists and get name formatted
        let group = self.group(group_name)?;
        // Fetch group and entities here as copies (dropping group reference for borrow checker)
        let entities = group.entities_sorted();
        let group_name = group.name();

        self.check_entity_without_enough_time_for_activity(id, &entities)?;

        // Add each entity in the group to the activity.
        // We do not care about the result: if the entity is already in the activity, it is fine.
        for entity_name in entities {
            // TODO if Err:AlreadyIn don't queue the work hours
            let _ = self.activities.add_entity(id, entity_name);
        }

        // Add the group to the activity
        self.activities.add_group(id, clean_string(group_name)?)?;

        // TODO queue added entities and invalidate all of their activities
        self.events()
            .borrow_mut()
            .emit_group_added_to_activity(self, &self.activity(id)?);
        Ok(())
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
    /// # use felix_backend::data::Data;
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
    pub fn remove_group_from_activity<S>(&mut self, id: ActivityID, group_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // Check that the group exists and get name formatted
        let group_name = self.group(group_name)?.name();

        let entities_to_remove =
            self.entities_participating_through_this_group_only(id, &group_name)?;

        for entity_name in &entities_to_remove {
            // The entity may not be in the activity if excluded from group.
            let _ = self.activities.remove_entity(id, entity_name);
            // TODO if err::NotIn then don't update
        }

        self.activities.remove_group(id, &group_name)?;
        // TODO invalidate activities and queue schedules of entities

        self.events()
            .borrow_mut()
            .emit_group_removed_from_activity(self, &self.activity(id)?);
        Ok(())
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
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    ///
    /// // new_name is formatted from "New name" to "New Name"
    /// let new_name = data.set_activity_name(activity_id, "New name").unwrap();
    /// assert_eq!(data.activity(activity_id).unwrap().name(), new_name);
    /// ```
    #[must_use]
    pub fn set_activity_name<S>(&mut self, id: ActivityID, name: S) -> Result<String>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        self.activities.set_name(id, name.clone())?;
        self.events()
            .borrow_mut()
            .emit_activity_renamed(self, &self.activity(id)?);
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
    /// # use felix_backend::data::{Data, Time, MIN_TIME_DISCRETIZATION};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let min_valid_duration = MIN_TIME_DISCRETIZATION;
    ///
    /// assert!(data.set_activity_duration(activity_id, min_valid_duration).is_ok());
    /// assert_eq!(data.activity(activity_id).unwrap().duration(), min_valid_duration);
    /// ```
    #[must_use]
    pub fn set_activity_duration(&mut self, id: ActivityID, new_duration: Time) -> Result<()> {
        // If the duration is longer than the previous one, check for conflicts
        self.check_entity_without_enough_time_to_set_duration(id, new_duration)?;
        self.activities.set_duration(id, new_duration)?;
        // TODO for each entity in the activity update schedules
        self.events()
            .borrow_mut()
            .emit_activity_duration_changed(self, &self.activity(id)?);
        Ok(())
    }
}
