mod error_checks;

use super::helpers::clean_string;
use crate::errors::Result;
use crate::{Data, Entity};

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
        self.work_hours
            .add_empty_custom_work_intervals_for(name.clone());
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
        self.work_hours.remove_custom_work_hours_of(&name);
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
        // Then, rename for custom work hours
        self.work_hours
            .rename_entity_for_custom_work_hours(&old_name, new_name.clone());

        let entity = self
            .entity(&new_name)
            .expect("Entity was renamed succesfuly so this is valid");

        self.events()
            .borrow_mut()
            .emit_entity_renamed(self, &entity, &old_name);
        Ok(new_name)
    }
}
