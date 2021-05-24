mod groups;
pub use groups::Groups;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use felix_errors::{already_in::AlreadyIn, name_taken::NameTaken, not_in::NotIn, Result};

/// A group is an aggregation of entities.
///
/// Groups have unique names. A group may not have the same name as an entity.
///
/// This structure is read-only. If you wish to create or modify a group, use the Data object.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Group {
    name: String,
    entities: Vec<String>,
}

impl Group {
    /// Creates a new group with the given name.
    fn new(name: String) -> Group {
        Group {
            name,
            entities: Vec::new(),
        }
    }

    // *** Getters ***
    // This is the only public API, no setters. To modify a group, use the inner field.
    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Getter for the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<String> {
        self.entities.clone()
    }

    // *** Private Setters ***
    /// Adds an entity to the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is already in the group.
    fn add_entity(&mut self, entity_name: String) -> Result<()> {
        if self.entities.contains(&entity_name) {
            Err(AlreadyIn::entity_already_in_group(entity_name, self.name()))
        } else {
            self.entities.push(entity_name);
            self.entities.sort();
            Ok(())
        }
    }

    /// Removes an entity from the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not in the group.
    fn remove_entity(&mut self, entity_name: &str) -> Result<()> {
        if let Some(position) = self.entities.iter().position(|name| name == entity_name) {
            self.entities.remove(position);
            self.entities.sort();
            Ok(())
        } else {
            Err(NotIn::entity_not_in_group(entity_name, self.name()))
        }
    }

    /// Sets the name of the group.
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Renames an entity in the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not part of the group or if
    /// an entity with the new name is already in the group.
    fn rename_entity(&mut self, old_name: &str, new_name: String) -> Result<()> {
        if self.entities.contains(&new_name) {
            return Err(NameTaken::name_taken_by_entity(new_name));
        };

        self.remove_entity(old_name)?;

        self.entities.push(new_name);
        self.entities.sort();
        Ok(())
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().cmp(&other.name())
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
