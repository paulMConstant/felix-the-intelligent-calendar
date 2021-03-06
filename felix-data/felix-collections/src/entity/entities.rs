use crate::Entity;
use felix_errors::{does_not_exist::DoesNotExist, name_taken::NameTaken, Result};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, HashMap};

/// Manages the entities. Makes sure there are no duplicates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entities {
    entities: HashMap<String, Entity>,
}

impl Entities {
    /// Creates the Entities collection.
    #[must_use]
    pub fn new() -> Entities {
        Entities {
            entities: HashMap::new(),
        }
    }

    /// Returns immutable references to the entities, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Entity> {
        let mut entity_vec: Vec<&Entity> = self.entities.values().collect();
        entity_vec.sort();
        entity_vec
    }

    /// Returns a copy of the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    pub fn get_by_name(&self, name: &str) -> Result<Entity> {
        match self.entities.get(name) {
            Some(entity) => Ok(entity.clone()),
            None => Err(DoesNotExist::entity_does_not_exist(name)),
        }
    }

    // This function may be useful when entities hold more data than their name (e.g. a count)

    // Returns a mutable reference to the entity with the given name.
    //
    // # Errors
    //
    // Returns Err if the entity does not exist.
    //
    // Keep this function private !
    // No mutable access to elements of the collection should be granted.
    //fn get_mut_by_name(&mut self, name: &str) -> Result<&mut Entity> {
    //    match self.entities.get_mut(name) {
    //        Some(entity) => Ok(entity),
    //        None => Err(DoesNotExist::entity_does_not_exist(name)),
    //    }
    //}

    /// Adds an entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the name is already taken.
    pub fn add(&mut self, name: String) -> Result<()> {
        match self.entities.entry(name.clone()) {
            Entry::Occupied(_) => Err(NameTaken::name_taken_by_entity(name)),
            Entry::Vacant(v) => {
                v.insert(Entity::new(name));
                Ok(())
            }
        }
    }

    /// Removes the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    pub fn remove(&mut self, name: &str) -> Result<()> {
        match self.entities.remove(name) {
            Some(_) => Ok(()),
            None => Err(DoesNotExist::entity_does_not_exist(name)),
        }
    }

    /// Renames the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the new name is already taken.
    pub fn set_name_of(&mut self, old_name: &str, new_name: String) -> Result<()> {
        match self.entities.entry(new_name.clone()) {
            Entry::Occupied(_) => Err(NameTaken::name_taken_by_entity(new_name)),
            Entry::Vacant(_) => {
                // We have to change the key and the value
                match self.entities.remove(old_name) {
                    Some(mut entity) => {
                        entity.set_name(new_name.clone());
                        self.entities.insert(new_name, entity);
                        Ok(())
                    }
                    None => Err(DoesNotExist::entity_does_not_exist(old_name)),
                }
            }
        }
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}

// No tests, functions are tested in felix_data
