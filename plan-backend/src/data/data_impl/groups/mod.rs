mod error_checks;
mod inner;

use super::helpers::clean_string;
use crate::data::{Data, Group};
use crate::errors::Result;

/// Operations on groups
impl Data {
    /// Returns the groups, sorted by name.
    #[must_use]
    pub fn groups_sorted(&self) -> Vec<&Group> {
        self.groups.sorted_by_name()
    }

    /// Returns an immutable reference to the group with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the group does not exist.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// // name = "Group" because of formatting
    /// let name = data.add_group("group").unwrap();
    /// assert!(data.group(name).is_ok());
    ///
    /// let invalid_name = "Group which does not exist";
    /// assert!(data.entity(invalid_name).is_err());
    /// ```
    #[must_use]
    pub fn group<S>(&self, name: S) -> Result<&Group>
    where
        S: Into<String>,
    {
        self.groups.get_by_name(&clean_string(name)?)
    }

    /// Adds a new group with the given formatted name.
    ///
    /// # Errors
    ///
    /// Returns Err if the group has the same name as another group or entity
    /// or if the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// data.add_group("New group").unwrap();
    /// assert_eq!(data.groups_sorted().len(), 1);
    /// ```
    #[must_use]
    pub fn add_group<S>(&mut self, name: S) -> Result<String>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        self.check_name_taken_by_entity(&name)?;

        self.groups.add(name.clone())?;
        self.events().emit_group_added(self);
        Ok(name)
    }

    /// Removes a group with the given formatted name.
    ///
    /// If the group is taking part in any activity, it is removed from them.
    ///
    /// # Errors
    ///
    /// Returns Err if the group does not exist or if the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let group_name = data.add_group("New group").unwrap();
    /// data.remove_group(group_name).unwrap();
    /// assert!(data.groups_sorted().is_empty());
    /// ```
    #[must_use]
    pub fn remove_group<S>(&mut self, name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // TODO remove group in activity
        let name = clean_string(name)?;
        self.groups.remove(&name)?;
        self.activities.remove_group_from_all(&name);
        self.events().emit_group_removed(self);
        Ok(())
    }

    /// Adds the entity with the given name to the group with the given name.
    /// Every name is formatted before use.
    ///
    /// # Errors
    ///
    /// Returns Err if the group does not exist, if any formatted name is empty,
    /// if the entity does not have enough time for the activities of the group,
    /// if the entity does not exist or if the entity is already part of the group.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let group_name = data.add_group("New group").unwrap();
    /// let entity_name = data.add_entity("Entity").unwrap();
    ///
    /// data.add_entity_to_group(group_name.clone(), entity_name.clone());
    /// let entities = data.group(group_name).unwrap().entities_sorted();
    /// assert_eq!(entities[0], entity_name);
    /// ```
    #[must_use]
    pub fn add_entity_to_group<S1, S2>(&mut self, group_name: S1, entity_name: S2) -> Result<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        // Check if the entity exists and format name
        let entity_name = self.entity(entity_name)?.name();
        let group_name = clean_string(group_name)?;

        // If the groups takes part in activities in which the entity does not,
        // we need to make sure the entity has time for them.
        self.check_has_enough_time_for_group(&group_name, &entity_name)?;
        self.groups
            .add_entity_to_group(&group_name, entity_name.clone())?;
        // Add the entity to every activity of the group
        self.activities
            .add_entity_to_activities_with_group(&group_name, entity_name);
        self.events().emit_entity_added_to_group(self);
        Ok(())
    }

    /// Removes the entity with the given name from the group with the given name.
    /// Every name is formatted before use.
    ///
    /// The entity is removed from any activity in which its group participates,
    /// unless it is participating in the activity through another group.
    ///
    /// # Errors
    ///
    /// Returns Err if the group does not exist, if any formatted name is empty,
    /// if the entity does not exist or if the entity is not part of the group.
    ///
    /// # Example
    ///
    /// ```
    /// # use plan_backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let group_name = data.add_group("Group").unwrap();
    /// let entity_name = data.add_entity("Entity").unwrap();
    ///
    /// data.add_entity_to_group(group_name.clone(), entity_name.clone()).unwrap();
    /// data.remove_entity_from_group(group_name.clone(), entity_name.clone()).unwrap();
    ///
    /// let entities = data.group(group_name).unwrap().entities_sorted();
    /// assert!(entities.is_empty());
    /// ```
    #[must_use]
    pub fn remove_entity_from_group<S1, S2>(
        &mut self,
        group_name: S1,
        entity_name: S2,
    ) -> Result<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        // Check if the entity exists & format name
        let entity_name = self.entity(entity_name)?.name();
        let group_name = self.group(group_name)?.name();

        self.groups
            .remove_entity_from_group(&group_name, &entity_name)?;

        // Remove the entity from activities in which it is participating through the group
        let activity_ids_in_which_to_remove_entity = self
            .ids_of_activities_in_which_entity_is_participating_only_through_this_group(
                &entity_name,
                &group_name,
            );
        for id in activity_ids_in_which_to_remove_entity {
            self.remove_entity_from_activity(id, &entity_name)?;
        }
        self.events().emit_entity_removed_from_group(self);
        Ok(())
    }

    /// Renames the group with the given name.
    /// Every name is formatted before use.
    ///
    /// If the group is taking part in activities, it is renamed there as well.
    /// Returns the formatted new name of the group.
    /// # Errors
    ///
    /// Returns Err if the group does not exist, if any formatted name is empty
    /// or if the name is already taken.
    #[must_use]
    pub fn set_group_name<S1, S2>(&mut self, old_name: S1, new_name: S2) -> Result<String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let new_name = clean_string(new_name)?;
        self.check_name_taken_by_entity(&new_name)?;

        // First, rename in entities to check for any error
        let old_name = clean_string(old_name)?;
        self.groups.set_name_of(&old_name, new_name.clone())?;

        // Then, rename in activities
        self.activities
            .rename_group_in_all(&old_name, new_name.clone());
        self.events().emit_group_renamed(self);
        Ok(new_name)
    }
}
