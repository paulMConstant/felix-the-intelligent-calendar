mod error_checks;
mod inner;

use super::helpers::clean_string;
use crate::data::{Data, Entity, Time, TimeInterval};
use crate::errors::Result;

/// Operations on entities
impl Data {
    /// Returns vector of immutable references to the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<&Entity> {
        self.entities.sorted_by_name()
    }

    /// Gets a copy of the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// // name = "Jeanne" because of formatting
    /// let name = data.add_entity("jeanne").unwrap();
    /// assert!(data.entity(name).is_ok());
    ///
    /// let invalid_name = "Jean";
    /// assert!(data.entity(invalid_name).is_err());
    /// ```
    #[must_use]
    pub fn entity<S>(&self, name: S) -> Result<Entity>
    where
        S: Into<String>,
    {
        self.entities.get_by_name(&clean_string(name)?)
    }

    /// Adds an entity with the formatted given name.
    ///
    /// Returns the name of the added entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the name is already taken.
    ///
    /// # Example
    ///
    /// ```
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
    /// // Name already taken
    /// assert!(data.add_entity(name).is_err());
    /// ```
    #[must_use]
    pub fn add_entity<S>(&mut self, name: S) -> Result<String>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        // Check if a group has the same name
        self.check_name_taken_by_group(&name)?;
        self.entities.add(name.clone())?;
        let entity = self
            .entity(&name)
            .expect("Entity was just added so it exists");
        self.events().borrow_mut().emit_entity_added(self, &entity);
        Ok(name)
    }

    /// Removes the entity with the formatted given name.
    ///
    /// If the entity is taking part in any activity, it is removed from them.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
    /// assert!(data.remove_entity(name.clone()).is_ok());
    /// // Entity does not exist anymore
    /// assert!(data.remove_entity(name).is_err());
    /// ```
    #[must_use]
    pub fn remove_entity<S>(&mut self, name: S) -> Result<()>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        let position_of_removed_entity = self
            .entities_sorted()
            .into_iter()
            .position(|entity| entity.name() == name);
        // First, remove in entities to check for any error
        self.entities.remove(&name)?;
        // If the entity was successfuly removed in entities, remove it
        // in all activities and groups
        self.activities.remove_entity_from_all(&name);
        self.groups.remove_entity_from_all(&name);
        let position_of_removed_entity = position_of_removed_entity.expect(
            "If the entity was removed then it existed, therefore position should be valid",
        );
        self.events()
            .borrow_mut()
            .emit_entity_removed(self, position_of_removed_entity, &name);
        Ok(())
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
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
    /// let new_name = "Jean";
    ///
    /// assert!(data.set_entity_name(name.clone(), new_name).is_ok());
    ///
    /// let invalid_name = name;
    /// // No entity has this name anymore
    /// assert!(data.set_entity_name(invalid_name, "other name").is_err());
    /// ```
    #[must_use]
    pub fn set_entity_name<S1, S2>(&mut self, old_name: S1, new_name: S2) -> Result<String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let new_name = clean_string(new_name)?;
        self.check_name_taken_by_group(&new_name)?;

        // First, rename in entities to check for any error
        let old_name = clean_string(old_name)?;
        self.entities.set_name_of(&old_name, new_name.clone())?;

        // Then, rename in group (group/activities order does not matter)
        self.groups
            .rename_entity_in_all(&old_name, new_name.clone());
        // Then, rename in activities
        self.activities
            .rename_entity_in_all(&old_name, new_name.clone());

        let entity = self
            .entity(&new_name)
            .expect("Entity was renamed succesfuly so this is valid");

        self.events()
            .borrow_mut()
            .emit_entity_renamed(self, &entity, &old_name);
        Ok(new_name)
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
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
    /// let mail = "jeanne@xyz.com";
    ///
    /// assert!(data.set_entity_mail(name.clone(), mail).is_ok());
    /// assert_eq!(data.entity(name).unwrap().mail(), mail);
    /// ```
    #[must_use]
    pub fn set_entity_mail<S1, S2>(&mut self, entity_name: S1, mail: S2) -> Result<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.entities
            .set_mail_of(&clean_string(entity_name)?, mail.into())
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
    /// # use felix_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
    /// assert_eq!(data.entity(name.clone()).unwrap().send_me_a_mail(), false);
    ///
    /// let send = true;
    /// assert!(data.set_send_mail_to(name.clone(), send).is_ok());
    /// assert!(data.entity(name).unwrap().send_me_a_mail());
    /// ```
    #[must_use]
    pub fn set_send_mail_to<S>(&mut self, entity_name: S, send: bool) -> Result<()>
    where
        S: Into<String>,
    {
        self.entities
            .set_send_mail_to(&clean_string(entity_name)?, send)
    }

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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12,0));
    /// data.add_work_interval(morning_shift).unwrap();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
    /// let activity_id = data.add_activity("Activity").unwrap().id();
    /// let activity_duration = Time::new(1, 0);
    /// data.set_activity_duration(activity_id, activity_duration).unwrap();
    /// data.add_entity_to_activity(activity_id, name.clone());
    ///
    /// // Total time is 4 hours, time taken by activity is 1 hour.
    /// assert_eq!(data.free_time_of(name).unwrap(), Time::new(3, 0));
    /// ```
    #[must_use]
    pub fn free_time_of<S>(&self, entity_name: S) -> Result<Time>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;

        // total_available_time checks if the entity exists
        let total_duration = self.total_available_time(&entity_name)?;
        let activity_duration = self.time_taken_by_activities(&entity_name);
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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
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
    pub fn work_hours_of<S>(&self, entity_name: S) -> Result<Vec<TimeInterval>>
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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
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
    ) -> Result<()>
    where
        S: Into<String>,
    {
        // If this intervals overrides the global work hours,
        // check if the entity has enough free time
        let entity_name = clean_string(entity_name)?;
        self.check_entity_will_have_enough_time_with_custom_interval(
            &entity_name,
            interval.duration(),
        )?;
        self.entities
            .add_custom_work_interval_for(&entity_name, interval)?;

        self.events()
            .borrow_mut()
            .emit_custom_work_hours_changed(self);
        Ok(())
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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
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
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        // TODO Continue Here First

        self.check_entity_has_custom_interval(&entity_name, &interval)?;
        self.check_entity_will_have_enough_time_after_deletion_of_interval(
            &entity_name,
            interval.duration(),
        )?;
        self.entities
            .remove_custom_work_interval_for(&entity_name, interval)?;
        self.events()
            .borrow_mut()
            .emit_custom_work_hours_changed(self);
        Ok(())
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
    /// # use felix_backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap();
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
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        self.check_entity_has_custom_interval(&entity_name, &old_interval)?;
        self.check_entity_will_have_enough_time_after_update(
            &entity_name,
            old_interval.duration(),
            new_interval.duration(),
        )?;

        self.entities
            .update_custom_work_interval_for(&entity_name, old_interval, new_interval)?;
        self.events()
            .borrow_mut()
            .emit_custom_work_hours_changed(self);
        Ok(())
        // TODO update possible insertion times
    }
}
