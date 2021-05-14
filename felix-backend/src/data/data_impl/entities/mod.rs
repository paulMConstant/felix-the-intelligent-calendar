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
    pub fn work_hours_of<S>(&self, entity_name: S) -> Result<Vec<TimeInterval>>
    where
        S: Into<String>,
    {
        let custom_work_hours = self.entity(entity_name)?.custom_work_hours();
        Ok(if custom_work_hours.is_empty() {
            self.work_hours()
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
        self.check_no_activity_inserted()?;
        self.check_entity_will_have_enough_time_with_custom_interval(
            &entity_name,
            interval.duration(),
        )?;
        self.entities
            .add_custom_work_interval_for(&entity_name, interval)?;
        self.notify_work_hours_changed();

        Ok(())
    }

    /// Removes the given custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval is not found.
    pub fn remove_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;

        self.check_entity_has_custom_interval(&entity_name, &interval)?;
        self.check_no_activity_inserted()?;
        self.check_entity_will_have_enough_time_after_deletion_of_interval(
            &entity_name,
            interval.duration(),
        )?;
        self.entities
            .remove_custom_work_interval_for(&entity_name, interval)?;

        self.notify_work_hours_changed();
        Ok(())
    }

    /// Replaces the given time interval with the new one for the entity with the given formatted
    /// name
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found, if the interval is not found, if the
    /// time interval can't be updated because the entity does not have enough time left
    /// or if the updated interval overlaps with other intervals.
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
        self.check_no_activity_inserted()?;
        self.check_entity_has_custom_interval(&entity_name, &old_interval)?;
        self.check_entity_will_have_enough_time_after_update(
            &entity_name,
            old_interval.duration(),
            new_interval.duration(),
        )?;

        self.entities
            .update_custom_work_interval_for(&entity_name, old_interval, new_interval)?;

        self.notify_work_hours_changed();
        Ok(())
    }
}
