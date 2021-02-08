use crate::data::{Entity, TimeInterval};
use crate::errors::{does_not_exist::DoesNotExist, name_taken::NameTaken, Result};
use std::collections::HashMap;

/// Manages the entities. Makes sure there are no duplicates.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    #[must_use]
    pub fn get_by_name(&self, name: &String) -> Result<Entity> {
        match self.entities.get(name) {
            Some(entity) => Ok(entity.clone()),
            None => Err(DoesNotExist::entity_does_not_exist(name)),
        }
    }

    /// Returns a mutable reference to the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    ///
    /// Keep this function private !
    /// No mutable access to elements of the collection should be granted.
    #[must_use]
    fn get_mut_by_name(&mut self, name: &String) -> Result<&mut Entity> {
        match self.entities.get_mut(name) {
            Some(entity) => Ok(entity),
            None => Err(DoesNotExist::entity_does_not_exist(name)),
        }
    }

    /// Adds an entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the name is already taken.
    #[must_use]
    pub fn add(&mut self, name: String) -> Result<()> {
        if self.entities.contains_key(&name) {
            Err(NameTaken::name_taken_by_entity(name))
        } else {
            self.entities.insert(name.clone(), Entity::new(name));
            Ok(())
        }
    }

    /// Removes the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    #[must_use]
    pub fn remove(&mut self, name: &String) -> Result<()> {
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
    #[must_use]
    pub fn set_name_of(&mut self, old_name: &String, new_name: String) -> Result<()> {
        if self.entities.contains_key(&new_name) {
            Err(NameTaken::name_taken_by_entity(new_name))
        } else {
            // We have to change the key and the value
            match self.entities.remove(old_name) {
                Some(mut entity) => {
                    entity.inner.set_name(new_name.clone());
                    self.entities.insert(new_name, entity);
                    Ok(())
                }
                None => Err(DoesNotExist::entity_does_not_exist(old_name)),
            }
        }
    }

    /// Sets the mail of the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    #[must_use]
    pub fn set_mail_of(&mut self, entity_name: &String, mail: String) -> Result<()> {
        self.get_mut_by_name(entity_name)?.inner.set_mail(mail);
        Ok(())
    }

    /// Sets 'send_me_a_mail' for the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    #[must_use]
    pub fn set_send_mail_to(&mut self, entity_name: &String, send: bool) -> Result<()> {
        self.get_mut_by_name(entity_name)?
            .inner
            .set_send_me_a_mail(send);
        Ok(())
    }

    /// Adds a work interval to the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the work interval overlaps with another.
    #[must_use]
    pub fn add_custom_work_interval_for(
        &mut self,
        entity_name: &String,
        interval: TimeInterval,
    ) -> Result<()> {
        self.get_mut_by_name(entity_name)?
            .inner
            .add_work_interval(interval)
    }

    /// Removes a work interval from the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the work interval is not found.
    #[must_use]
    pub fn remove_custom_work_interval_for(
        &mut self,
        entity_name: &String,
        interval: TimeInterval,
    ) -> Result<()> {
        self.get_mut_by_name(entity_name)?
            .inner
            .remove_work_interval(interval)
    }

    /// Updates the given interval for the given entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist ,if the work interval is not found
    /// or if the new work interval overlaps with others.
    #[must_use]
    pub fn update_custom_work_interval_for(
        &mut self,
        entity_name: &String,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<()> {
        self.get_mut_by_name(entity_name)?
            .inner
            .update_work_interval(old_interval, new_interval)
    }
}

// No tests, functions are tested in tests directory