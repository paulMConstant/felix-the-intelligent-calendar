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

    /// Returns a copy of the group with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the group does not exist.
    pub fn group<S>(&self, name: S) -> Result<Group>
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
    pub fn add_group<S>(&mut self, name: S) -> Result<String>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        self.check_name_taken_by_entity(&name)?;

        self.groups.add(name.clone())?;
        let group = self
            .group(&name)
            .expect("Group was just added so it should exist");
        self.events().borrow_mut().emit_group_added(self, &group);
        Ok(name)
    }

    /// Removes a group with the given formatted name.
    ///
    /// If the group is taking part in any activity, it is removed from them.
    ///
    /// # Errors
    ///
    /// Returns Err if the group does not exist or if the formatted name is empty.
    pub fn remove_group<S>(&mut self, name: S) -> Result<()>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        let position_of_removed_group = self
            .groups_sorted()
            .into_iter()
            .position(|group| group.name() == name);

        // Remove group in all activities
        for id in self
            .activities_not_sorted()
            .iter()
            .map(|activity| activity.id())
            .collect::<Vec<_>>()
        {
            // If the group is already out of the activity, ok
            let _ = self.remove_group_from_activity(id, &name);
        }

        self.groups.remove(&name)?;
        let position_of_removed_group =
            position_of_removed_group.expect("Group was removed so it should have existed");

        self.events()
            .borrow_mut()
            .emit_group_removed(self, position_of_removed_group);
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
        let group = self
            .group(&group_name)
            .expect("We just added an entity, therefore the group exists");
        self.events()
            .borrow_mut()
            .emit_entity_added_to_group(self, &group);
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
        let group = self
            .group(&group_name)
            .expect("We just removed an entity, therefore the group exists");
        self.events()
            .borrow_mut()
            .emit_entity_removed_from_group(self, &group);
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
        let group = self
            .group(&new_name)
            .expect("Group was renamed so it should exist");
        self.events().borrow_mut().emit_group_renamed(self, &group);
        Ok(new_name)
    }
}
