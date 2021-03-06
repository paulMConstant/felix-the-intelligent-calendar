mod error_checks;
mod inner;

use super::helpers::clean_string;
use crate::data::{Activity, ActivityID, Data, Time, RGBA};
use crate::errors::{invalid_insertion::InvalidInsertion, Result};

use std::collections::HashSet;

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

    /// Returns the possible insertion times of an activity.
    /// If they are not calculated, returns Ok(None).
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub fn possible_insertion_times_of_activity(
        &mut self,
        id: ActivityID,
    ) -> Result<Option<HashSet<Time>>> {
        let activity = self.activities.get_by_id(id)?;
        let participants = activity.entities_sorted();

        self.activities.possible_insertion_times_of_activity(
            self.work_hours_and_activity_durations_from_entities(&participants)?,
            id,
        )
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
    /// use felix_backend::data::Data;
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
    /// use felix_backend::data::Data;
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
        let activities = self.activities_sorted();
        let position_of_removed_activity = activities
            .into_iter()
            .position(|activity| activity.id() == id);

        let impacted_entities = self.activity(id)?.entities_sorted();
        self.activities.remove(id)?;

        self.queue_entities(impacted_entities)?;

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
    /// use felix_backend::data::{Data, Time, TimeInterval};
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
        self.activities.add_entity(id, entity_name.clone())?;
        self.queue_entities(vec![entity_name])?;

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
    /// use felix_backend::data::{Data, Time, TimeInterval};
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

        self.queue_entities(vec![entity_name])?;
        // Queue the activity because it is not included in the entities' activities anymore
        let activity = self.activity(id)?;
        self.queue_activity_participants(&activity)?;

        self.events()
            .borrow_mut()
            .emit_entity_removed_from_activity(self, &activity);
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
    /// use felix_backend::data::Data;
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
            let _ = self.activities.add_entity(id, entity_name);
        }

        // Add the group to the activity
        self.activities.add_group(id, clean_string(group_name)?)?;

        let activity = self.activity(id)?;
        self.queue_activity_participants(&activity)?;

        self.events()
            .borrow_mut()
            .emit_group_added_to_activity(self, &activity);
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
    /// use felix_backend::data::Data;
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
        }

        self.activities.remove_group(id, &group_name)?;

        let activity = self.activity(id)?;
        self.queue_activity_participants(&activity)?;

        self.events()
            .borrow_mut()
            .emit_group_removed_from_activity(self, &activity);
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
    /// use felix_backend::data::Data;
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
    /// use felix_backend::data::{Data, Time, MIN_TIME_DISCRETIZATION};
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
        let activity = self.activity(id)?;
        if new_duration > activity.duration() {
            self.check_entity_without_enough_time_to_set_duration(id, new_duration)?;
            if activity.insertion_interval().is_some() {
                self.activities.store_activity_was_inserted(id)?;
                self.insert_activity(id, None)?;
            }
        }
        self.activities.set_duration(id, new_duration)?;

        let activity = self.activity(id)?;
        self.queue_activity_participants(&activity)?;
        self.events()
            .borrow_mut()
            .emit_activity_duration_changed(self, &activity);
        Ok(())
    }

    /// Sets the color of the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    ///
    /// # Example
    /// ```
    /// use felix_backend::data::{Data, RGBA};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let color = RGBA { red: 1.0, green: 0.5, blue: 0.3, alpha: 1.0 };
    /// data.set_activity_color(activity_id, color).unwrap();
    /// assert_eq!(color, data.activity(activity_id).unwrap().color());
    /// ```
    #[must_use]
    pub fn set_activity_color(&mut self, id: ActivityID, color: RGBA) -> Result<()> {
        self.activities.set_color(id, color)?;
        let activity = self.activity(id)?;
        self.events()
            .borrow_mut()
            .emit_activity_color_changed(self, &activity);
        Ok(())
    }

    /// Tries to insert the activity with given id with the given beginning.
    /// If None is given, the activity is removed from the schedule.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or if the insertion time is not available.
    ///
    /// # Example
    ///
    /// ```
    /// use felix_backend::data::{Data, Time, TimeInterval, MIN_TIME_DISCRETIZATION};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity = data.add_entity("Paul").unwrap();
    /// let work_interval = TimeInterval::new(Time::new(8, 0), Time::new(10, 15));
    /// let work_hours = data.add_work_interval(work_interval).unwrap();
    /// data.add_entity_to_activity(activity_id, entity).unwrap();
    ///
    /// while data.possible_insertion_times_of_activity(activity_id).unwrap().is_none() {
    /// // For the purpose of this test, wait for asynchronous computation of possible beginnings.
    /// }
    ///
    /// let insertion_time = Time::new(8, 0);
    /// assert!(data.insert_activity(activity_id, Some(insertion_time)).is_ok());
    ///
    /// let activity = data.activity(activity_id).unwrap();
    /// let end = insertion_time + activity.duration();
    /// let expected_insertion_interval = TimeInterval::new(insertion_time, end);
    /// assert_eq!(activity.insertion_interval().unwrap(), expected_insertion_interval);
    /// ```
    #[must_use]
    pub fn insert_activity(&mut self, id: ActivityID, insertion_time: Option<Time>) -> Result<()> {
        if let Some(insertion_time) = insertion_time {
            // We want to insert the activity
            if let Some(possible_insertion_times) = self.possible_insertion_times_of_activity(id)? {
                if possible_insertion_times.contains(&insertion_time) {
                    self.activities.insert_activity(id, Some(insertion_time))?;
                    self.events()
                        .borrow_mut()
                        .emit_activity_inserted(self, &self.activity(id)?);
                    Ok(())
                } else {
                    // The given insertion time is not valid.
                    Err(InvalidInsertion::insertion_not_in_computed_insertions(
                        self.activity(id)?.name(),
                        insertion_time,
                    ))
                }
            } else {
                // Computation is not finished
                Err(InvalidInsertion::insertions_not_computed_yet(
                    self.activity(id)?.name(),
                ))
            }
        } else {
            // Remove activity from schedule
            self.activities.insert_activity(id, None)?;
            self.events()
                .borrow_mut()
                .emit_activity_inserted(self, &self.activity(id)?);
            Ok(())
        }
    }

    /// If activities were removed from the schedule because their duration was increased, insert
    /// them back into the schedule in the closest spot we find.
    ///
    /// # Example
    ///
    /// ```
    /// use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let work_interval = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    /// data.add_work_interval(work_interval).unwrap();
    ///
    /// let id_will_move = data.add_activity("Will Move").unwrap().id();
    /// let id_static = data.add_activity("Static").unwrap().id();
    /// let entity = data.add_entity("Jean").unwrap();
    ///
    /// data.add_entity_to_activity(id_static, &entity);
    /// data.add_entity_to_activity(id_will_move, &entity);
    ///
    /// data.set_activity_duration(id_static, Time::new(1, 0));
    /// data.insert_activity(id_static, Some(Time::new(9, 0)));
    ///
    /// data.set_activity_duration(id_will_move, Time::new(0, 30));
    /// data.insert_activity(id_will_move, Some(Time::new(8, 30)));
    /// assert!(data.activity(id_will_move).unwrap().insertion_interval().is_some());
    ///
    /// data.set_activity_duration(id_will_move, Time::new(0, 45));
    /// // We increased the duration of an inserted activity.
    /// // As we have to compute first to check if the activity can still be inserted,
    /// // it is removed from the schedule.
    /// assert_eq!(data.activity(id_will_move).unwrap().insertion_interval(), None);
    ///
    /// // Wait for computation result...
    /// while data.possible_insertion_times_of_activity(id_will_move).unwrap().is_none() {
    /// // For the purpose of this test, wait for asynchronous computation of possible beginnings.
    /// }
    ///
    /// // Try to insert the activity again
    /// data.insert_activities_removed_because_duration_increased_in_closest_spot();
    /// let new_beginning =
    /// data.activity(id_will_move).unwrap().insertion_interval()
    ///     .expect("Activity was not inserted !").beginning();
    /// assert_eq!(new_beginning, Time::new(8, 15));
    /// ```
    pub fn insert_activities_removed_because_duration_increased_in_closest_spot(&mut self) {
        let activity_ids_and_old_beginnings = self
            .activities
            .get_activities_removed_because_duration_increased();

        for (id, old_beginning) in activity_ids_and_old_beginnings {
            if let Some(possible_insertion_times) = self
                .possible_insertion_times_of_activity(id)
                .expect("Asking for possible times of activity which does not exist")
            {
                if self
                    .activities
                    .insert_activity_in_spot_closest_to(id, old_beginning, possible_insertion_times)
                    .is_some()
                {
                    self.events().borrow_mut().emit_activity_inserted(
                        self,
                        &self
                            .activity(id)
                            .expect("The given activity does not exist"),
                    );
                }
            }
        }
    }

    /// Clears the list of activities which were removed because their duration increased.
    /// This means that we give up on reinserting them automatically (for instance, it's been too
    /// long since the activity was removed).
    pub fn clear_list_activities_removed_because_duration_increased(&mut self) {
        self.activities
            .clear_activities_removed_because_duration_increased();
    }
}
