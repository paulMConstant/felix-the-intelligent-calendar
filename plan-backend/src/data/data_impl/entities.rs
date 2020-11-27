use super::helpers::entity_time_computation::{time_taken_by_activities, total_available_time};
use crate::data::{Data, Entity, Time, TimeInterval};

/// Operations on entities
impl Data {
    /// Returns vector of immutable references to the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<&Entity> {
        self.entities.sorted_by_name()
    }

    /// Gets an immutable reference to the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// // name = "Jeanne" because of formatting
    /// let name = data.add_entity("jeanne").unwrap().name();
    /// assert!(data.entity(name).is_ok());
    ///
    /// let invalid_name = "Jean";
    /// assert!(data.entity(invalid_name).is_err());
    /// ```
    #[must_use]
    pub fn entity<S>(&self, name: S) -> Result<&Entity, String>
    where
        S: Into<String>,
    {
        self.entities.get_by_name(name)
    }

    /// Adds an entity with the formatted given name.
    ///
    /// Returns an immutable reference to the added entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the name is already taken.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// // Name already taken
    /// assert!(data.add_entity(name).is_err());
    /// ```
    #[must_use]
    pub fn add_entity<S>(&mut self, name: S) -> Result<&Entity, String>
    where
        S: Into<String>,
    {
        self.entities.add(name)
    }

    /// Removes the entity with the formatted given name.
    ///
    /// If the entity is taking part in any activity, it is removed from them.
    /// Returns the name of the removed entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// assert!(data.remove_entity(name.clone()).is_ok());
    /// // Entity does not exist anymore
    /// assert!(data.remove_entity(name).is_err());
    /// ```
    #[must_use]
    pub fn remove_entity<S>(&mut self, name: S) -> Result<String, String>
    where
        S: Into<String>,
    {
        let name = name.into();
        // First, remove in entities to check for any error
        self.entities.remove(name.clone())?;
        // If the entity was successfuly removed in entities, remove it
        // in all activities
        Ok(self.activities.remove_participant_from_all(name)?)
    }

    /// Renames the entity with the formatted given name.
    ///
    /// If the entity is taking part in any activity, it is renamed there as well.
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or if the name is already taken.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// let new_name = "Jean";
    ///
    /// assert!(data.set_entity_name(name.clone(), new_name).is_ok());
    ///
    /// let invalid_name = name;
    /// // No entity has this name anymore
    /// assert!(data.set_entity_name(invalid_name, "other name").is_err());
    /// ```
    #[must_use]
    pub fn set_entity_name<S1, S2>(&mut self, old_name: S1, new_name: S2) -> Result<String, String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        // First, rename in entities to check for any error
        let old_name = old_name.into();
        let new_name = self.entities.set_name_of(old_name.clone(), new_name)?;
        // Then, rename in activities
        Ok(self
            .activities
            .rename_participant_in_all(old_name, new_name.clone())?)
    }

    /// Sets the mail of the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// let mail = "jeanne@xyz.com";
    ///
    /// assert!(data.set_entity_mail(name.clone(), mail).is_ok());
    /// assert_eq!(data.entity(name).unwrap().mail(), mail);
    /// ```
    #[must_use]
    pub fn set_entity_mail<S1, S2>(&mut self, entity_name: S1, mail: S2) -> Result<(), String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.entities.set_mail_of(entity_name, mail)
    }

    /// Set to true to send mails to the entity with formatted given name.
    ///
    /// # Errors
    ///
    /// Returs Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// assert_eq!(data.entity(name.clone()).unwrap().send_me_a_mail(), false);
    ///
    /// let send = true;
    /// assert!(data.set_send_mail_to(name.clone(), send).is_ok());
    /// assert!(data.entity(name).unwrap().send_me_a_mail());
    /// ```
    #[must_use]
    pub fn set_send_mail_to<S>(&mut self, entity_name: S, send: bool) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.entities.set_send_mail_to(entity_name, send)
    }

    // *** Custom Work Hours ***
    // - Individual getter
    // - Add / Remove

    /// Returns the free time of an entity (total time in work hours - time taken by activities).
    ///
    /// The activities should never take more time than the total time ; should that happen,
    /// Time::new(0, 0) is returned.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12,0));
    /// data.add_work_interval(morning_shift).unwrap();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// let activity_id = data.add_activity("Activity").unwrap().id();
    /// let activity_duration = Time::new(1, 0);
    /// data.set_activity_duration(activity_id, activity_duration).unwrap();
    /// data.add_participant_to_activity(activity_id, name.clone());
    ///
    /// // Total time is 4 hours, time taken by activity is 1 hour.
    /// assert_eq!(data.free_time_of(name).unwrap(), Time::new(3, 0));
    /// ```
    #[must_use]
    pub fn free_time_of<S>(&self, entity_name: S) -> Result<Time, String>
    where
        S: Into<String>,
    {
        let entity_name = entity_name.into();

        // total_available_time checks if the entity exists
        let total_duration = total_available_time(&self, &entity_name)?;
        let activity_duration = time_taken_by_activities(&self, &entity_name);
        Ok(if total_duration < activity_duration {
            Time::new(0, 0)
        } else {
            total_duration - activity_duration
        })
    }

    /// Returns the work hours of the entity with the formatted given name.
    ///
    /// If the entity has custom work hours, returns them, else returns the global work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity with given name is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    ///
    /// let regular_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(regular_work_interval);
    /// assert_eq!(data.work_hours_of(name.clone()).unwrap(), data.work_hours());
    ///
    /// let custom_work_interval = TimeInterval::new(Time::new(10, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(name.clone(), custom_work_interval);
    /// let expected = data.entity(name.clone()).unwrap().custom_work_hours();
    /// assert_eq!(data.work_hours_of(name).unwrap(), expected);
    /// ```
    #[must_use]
    pub fn work_hours_of<S>(&self, entity_name: S) -> Result<Vec<TimeInterval>, String>
    where
        S: Into<String>,
    {
        let custom_work_hours = self.entity(entity_name)?.custom_work_hours();
        Ok(if custom_work_hours.len() == 0 {
            self.work_hours().clone()
        } else {
            custom_work_hours
        })
    }

    /// Adds a custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval overlaps with others
    /// or if the entity does not have enough free time.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    ///
    /// let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// let overlapping_interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    ///
    /// assert!(data.add_custom_work_interval_for(name.clone(), custom_work_interval).is_ok());
    /// assert!(data.add_custom_work_interval_for(name.clone(), overlapping_interval).is_err());
    /// assert_eq!(data.entity(name).unwrap().custom_work_hours()[0], custom_work_interval);
    /// ```
    #[must_use]
    pub fn add_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        // If this intervals overrides the global work hours,
        // check if the entity has enough free time
        let entity_name = entity_name.into();
        if self
            .entity(entity_name.clone())? // Check if entity exists here
            .custom_work_hours()
            .is_empty()
        {
            let activity_duration = time_taken_by_activities(&self, &entity_name);
            if interval.duration() < activity_duration {
                return Err(format!(
                    "{} will not have enough time for their activities using these custom work hours.",
                    entity_name
                ));
            }
        }
        self.entities
            .add_custom_work_interval_for(entity_name, interval)
        // TODO update possible insertion times
    }

    /// Removes the given custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    ///
    /// let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(name.clone(), custom_work_interval).unwrap();
    ///
    /// assert_eq!(data.work_hours_of(name.clone()).unwrap().len(), 1);
    /// assert!(data.remove_custom_work_interval_for(name.clone(), custom_work_interval).is_ok());
    /// assert!(data.entity(name.clone()).unwrap().custom_work_hours().is_empty());
    ///
    /// let nonexistent_interval = custom_work_interval;
    /// assert!(data.remove_custom_work_interval_for(name, nonexistent_interval).is_err());
    /// ```
    #[must_use]
    pub fn remove_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        let entity_name = entity_name.into();
        // First, check if the entity has a corresponding custom work interval
        if self
            .entity(entity_name.clone())?
            .custom_work_hours()
            .contains(&interval)
            == false
        {
            return Err("The given time interval was not found.".to_owned());
        }
        // Check if the entity has enough free time
        let entity_free_time = self.free_time_of(entity_name.clone())?;
        if entity_free_time < interval.duration() {
            return Err(format!(
                "{} will not have enough time for their activities once this interval is removed.",
                entity_name
            ));
        }
        self.entities
            .remove_custom_work_interval_for(entity_name, interval)
        // TODO update possible insertion times
    }

    /// Replaces the given time interval with the new one for the entity with the given formatted
    /// name
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found, if the interval is not found, if the
    /// time interval can't be updated because the entity does not have enough time left
    /// or if the updated interval overlaps with other intervals.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Name").unwrap().name();
    /// let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(name.clone(), interval).unwrap();
    ///
    /// let new_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// assert!(data.update_custom_work_interval_for(name.clone(), interval, new_interval).is_ok());
    /// assert_eq!(data.work_hours_of(name).unwrap()[0], new_interval);
    /// ```
    #[must_use]
    pub fn update_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        let entity_name = entity_name.into();
        // First, check if the entity has a corresponding custom work interval
        if self
            .entity(entity_name.clone())?
            .custom_work_hours()
            .contains(&old_interval)
            == false
        {
            return Err("The given time interval was not found.".to_owned());
        }
        // If the interval is shorter, check that the entity will still have time left
        if new_interval.duration() < old_interval.duration() {
            let required_free_time = old_interval.duration() - new_interval.duration();
            if self.free_time_of(entity_name.clone())? < required_free_time {
                return Err(format!(
                    "{} does not have enough free time to reduce this interval.",
                    entity_name
                ));
            }
        }
        self.entities
            .update_custom_work_interval_for(entity_name, old_interval, new_interval)
        // TODO update possible insertion times
    }
}
