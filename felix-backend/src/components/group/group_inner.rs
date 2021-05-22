use crate::errors::{already_in::AlreadyIn, name_taken::NameTaken, not_in::NotIn, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GroupInner {
    name: String,
    entities: HashSet<String>,
}

impl GroupInner {
    /// Creates a new GroupInner with the given name.
    pub fn new(name: String) -> GroupInner {
        GroupInner {
            name,
            entities: HashSet::new(),
        }
    }

    // *** Getters ***

    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Getter for the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<&String> {
        let mut entity_vec: Vec<&String> = self.entities.iter().collect();
        entity_vec.sort();
        entity_vec
    }

    // *** Setters ***
    /// Adds an entity to the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is already in the group.
    pub fn add_entity(&mut self, entity_name: String) -> Result<()> {
        if self.entities.insert(entity_name.clone()) {
            Ok(())
        } else {
            Err(AlreadyIn::entity_already_in_group(entity_name, self.name()))
        }
    }

    /// Removes an entity from the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not in the group.
    pub fn remove_entity(&mut self, entity_name: &str) -> Result<()> {
        if self.entities.remove(entity_name) {
            Ok(())
        } else {
            Err(NotIn::entity_not_in_group(entity_name, self.name()))
        }
    }

    /// Sets the name of the group.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Renames an entity in the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not part of the group or if
    /// an entity with the new name is already in the group.
    pub fn rename_entity(&mut self, old_name: &str, new_name: String) -> Result<()> {
        if self.entities.contains(&new_name) {
            return Err(NameTaken::name_taken_by_entity(new_name));
        };

        if self.entities.remove(old_name) {
            self.entities.insert(new_name);
            Ok(())
        } else {
            Err(NotIn::entity_not_in_activity(old_name, self.name()))
        }
    }
}
