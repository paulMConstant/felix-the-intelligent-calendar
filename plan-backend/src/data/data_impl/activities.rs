use crate::data::{Activity, Data, Time};

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
        self.activities.add(name)
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

    /// Adds the participant with given name to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the participant is not found,
    /// if the participant does not have enough time left
    /// or the participant is already taking part in the activity.
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
    /// // Make sure the participant has enough time !
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(morning_shift).unwrap();
    ///
    /// assert!(data.add_participant_to_activity(activity_id, entity_name.clone()).is_ok());
    ///
    /// let participants = data.activity(activity_id).unwrap().participants_sorted();
    /// assert_eq!(participants.len(), 1);
    /// assert_eq!(participants[0], entity_name);
    /// ```
    #[must_use]
    pub fn add_participant_to_activity<S>(
        &mut self,
        id: u16,
        participant_name: S,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the entity exists and formats name
        let participant_name = self.entity(participant_name)?.name();
        // Check its free time
        let free_time = self.free_time_of(participant_name.clone())?;
        if free_time < self.activity(id)?.duration() {
            Err(format!(
                "{} does not have enough time left for this activity.",
                participant_name
            ))
        } else {
            // Add the entity to the activity
            self.activities.add_participant(id, participant_name)
        }
    }

    /// Removes the participant with given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the participant is not found
    /// or the participant is not taking part in the activity.
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
    /// // Make sure the participant has enough time before adding him to an activity
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(entity_name.clone(), morning_shift).unwrap();
    ///
    /// data.add_participant_to_activity(activity_id, entity_name.clone()).unwrap();
    /// assert!(data.remove_participant_from_activity(activity_id, entity_name).is_ok());
    /// assert!(data.activity(activity_id).unwrap().participants_sorted().is_empty());
    /// ```
    #[must_use]
    pub fn remove_participant_from_activity<S>(
        &mut self,
        id: u16,
        participant_name: S,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the entity exists
        let participant_name = self.entity(participant_name)?.name();
        // Remove the entity from the activity
        self.activities.remove_participant(id, participant_name)
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
        self.activities.set_name(id, name)
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
            // Duration is longer - check if it conflicts with participant's schedule
            let required_free_time = new_duration - current_duration; // > 0
            if let Some(entity_name) = activity
                .participants_sorted()
                .iter()
                // Call to unwrap() : we are sure that all participants in the activity exist.
                .find(|entity_name| {
                    self.free_time_of(entity_name.clone()).unwrap() < required_free_time
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
