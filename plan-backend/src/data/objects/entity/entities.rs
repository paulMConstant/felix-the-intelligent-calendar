use super::super::helpers::clean_string::clean;
use crate::data::{Entity, TimeInterval};
use std::collections::HashMap;

/// Manages the entities. Makes sure there are no duplicates.
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

    // Organization
    // - Collection getter
    // - Immutable individual getter
    // - Add
    // - Remove
    // - Modify

    /// Returns immutable references to the entities, sorted by name.
    #[must_use]
    pub fn sorted_by_name(&self) -> Vec<&Entity> {
        let mut entity_vec: Vec<&Entity> = self.entities.values().collect();
        entity_vec.sort();
        entity_vec
    }

    /// Returns immutable reference to the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or the formatted name is empty.
    #[must_use]
    pub fn get_by_name<S>(&self, name: S) -> Result<&Entity, String>
    where
        S: Into<String>,
    {
        let name = clean(name)?;
        match self.entities.get(&name) {
            Some(entity) => Ok(entity),
            None => Err(format!("{} does not exist !", name)),
        }
    }

    /// Returns a mutable reference to the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or the formatted name is empty.
    ///
    /// Keep this function private !
    /// No mutable access to elements of the collection should be granted.
    #[must_use]
    fn get_mut_by_name<S>(&mut self, name: S) -> Result<&mut Entity, String>
    where
        S: Into<String>,
    {
        let name = clean(name)?;
        match self.entities.get_mut(&name) {
            Some(entity) => Ok(entity),
            None => Err(format!("{} does not exist !", name)),
        }
    }

    /// Adds an entity with the formatted given name.
    /// Returns an immutable reference to the created entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the name is already taken.
    #[must_use]
    pub fn add<S>(&mut self, name: S) -> Result<&Entity, String>
    where
        S: Into<String>,
    {
        let name = clean(name)?;
        if self.entities.contains_key(&name) {
            Err(format!("{} already exists !", name))
        } else {
            self.entities
                .insert(name.clone(), Entity::new(name.clone()));
            Ok(&self.entities.get(&name).unwrap())
        }
    }

    /// Removes the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the formatted given name is empty.
    #[must_use]
    pub fn remove<S>(&mut self, name: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        let name = clean(name)?;
        match self.entities.remove(&name) {
            Some(_) => Ok(()),
            None => Err(format!("{} does not exist !", name)),
        }
    }

    /// Renames the entity with the formatted given name.
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the formatted new name is empty or already taken.
    #[must_use]
    pub fn set_name_of<S1, S2>(&mut self, old_name: S1, new_name: S2) -> Result<String, String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let new_name = clean(new_name)?;
        if self.entities.contains_key(&new_name) {
            Err(format!("The name '{}' is already taken !", new_name))
        } else {
            let old_name = clean(old_name)?;
            // We have to change the key and the value
            match self.entities.remove(&old_name) {
                Some(mut entity) => {
                    entity.inner.set_name(new_name.clone());
                    self.entities.insert(new_name.clone(), entity);
                    Ok(new_name)
                }
                None => Err(format!("'{}' does not exist !", old_name)),
            }
        }
    }

    /// Sets the mail of the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    #[must_use]
    pub fn set_mail_of<S1, S2>(&mut self, entity_name: S1, mail: S2) -> Result<(), String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.get_mut_by_name(entity_name)?.inner.set_mail(mail);
        Ok(())
    }

    /// Sets 'send_me_a_mail' for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    #[must_use]
    pub fn set_send_mail_to<S>(&mut self, entity_name: S, send: bool) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.get_mut_by_name(entity_name)?
            .inner
            .set_send_me_a_mail(send);
        Ok(())
    }

    /// Adds a work interval to the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the work interval overlaps with another.
    #[must_use]
    pub fn add_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.get_mut_by_name(entity_name)?
            .inner
            .add_work_interval(interval)
    }

    /// Removes a work interval from the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the work interval is not found.
    #[must_use]
    pub fn remove_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
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
    pub fn update_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.get_mut_by_name(entity_name)?
            .inner
            .update_work_interval(old_interval, new_interval)
    }
}

// No tests, functions are tested in tests directory
