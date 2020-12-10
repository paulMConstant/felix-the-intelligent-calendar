use super::Group;
use crate::errors::{does_not_exist::DoesNotExist, name_taken::NameTaken, Result};
use std::collections::HashMap;

/// Manages groups.
///
/// A group has a unique name and contains entities.
/// A group may not have the same name as an entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Groups {
    groups: HashMap<String, Group>,
}

impl Groups {
    /// Creates the groups collection.
    #[must_use]
    pub fn new() -> Groups {
        Groups {
            groups: HashMap::new(),
        }
    }

    /// Returns immutable references to the groups, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Group> {
        let mut group_vec: Vec<&Group> = self.groups.values().collect();
        group_vec.sort();
        group_vec
    }

    /// Returns immutable reference to the group with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the group does not exist.
    #[must_use]
    pub fn get_by_name(&self, name: &String) -> Result<&Group> {
        match self.groups.get(name) {
            Some(group) => Ok(group),
            None => Err(DoesNotExist::group_does_not_exist(name)),
        }
    }

    /// Adds a new group with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if a group with the same name exists.
    #[must_use]
    pub fn add(&mut self, name: String) -> Result<()> {
        if self.groups.contains_key(&name) {
            Err(NameTaken::name_taken_by_group(name))
        } else {
            self.groups.insert(name.clone(), Group::new(name));
            Ok(())
        }
    }

    /// Removes the group with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not found.
    #[must_use]
    pub fn remove(&mut self, name: &String) -> Result<()> {
        match self.groups.remove(name) {
            Some(_) => Ok(()),
            None => Err(DoesNotExist::group_does_not_exist(name)),
        }
    }

    /// Adds a new entity to the group with given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not found or if the entity is already in the group.
    #[must_use]
    pub fn add_entity_to_group(&mut self, group_name: &String, entity_name: String) -> Result<()> {
        match self.groups.get_mut(group_name) {
            Some(group) => {
                group.inner.add_entity(entity_name)?;
                Ok(())
            }
            None => Err(DoesNotExist::group_does_not_exist(group_name)),
        }
    }

    /// Removes the entity from the group with given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not found or if the entity is already in the group.
    #[must_use]
    pub fn remove_entity_from_group(
        &mut self,
        group_name: &String,
        entity_name: &String,
    ) -> Result<()> {
        match self.groups.get_mut(group_name) {
            Some(group) => {
                group.inner.remove_entity(entity_name)?;
                Ok(())
            }
            None => Err(DoesNotExist::group_does_not_exist(group_name)),
        }
    }

    /// Renames the group with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not found or if a group already has this name.
    #[must_use]
    pub fn set_name_of(&mut self, old_name: &String, new_name: String) -> Result<()> {
        if self.groups.contains_key(&new_name) {
            Err(NameTaken::name_taken_by_group(new_name))
        } else {
            // We have to change the key and the value
            match self.groups.remove(old_name) {
                Some(mut group) => {
                    group.inner.set_name(new_name.clone());
                    self.groups.insert(new_name, group);
                    Ok(())
                }
                None => Err(DoesNotExist::group_does_not_exist(old_name)),
            }
        }
    }

    /// Renames the entity with given name in all groups.
    pub fn rename_entity_in_all(&mut self, old_name: &String, new_name: String) {
        for group in self.groups.values_mut() {
            // We don't care about the result : it is fine if the entity is not
            // taking part in the activity, this will yield no conflict when it is renamed
            let _ = group.inner.rename_entity(old_name, new_name.clone());
        }
    }
}

// No tests, functions are tested in tests directory
