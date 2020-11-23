use super::super::super::helpers::clean_string::clean;
use super::{Entity, TimeInterval};
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

    /// Removes a work interval to the entity with the formatted given name.
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
}

#[cfg(test)]
mod tests {
    use super::super::super::time::Time;
    use super::*;

    #[test]
    fn add_remove_entities() {
        let mut entities = Entities::new();
        let jean = "Jean Lasalle";
        let aaa = "Aaa";
        let zzz = "Zzz";
        entities.add(jean).unwrap();
        assert_eq!(entities.sorted_by_name()[0].name(), jean);

        // Cannot add entity with the same name
        assert!(entities.add(jean).is_err());
        // Cannot add entity with empty name
        assert!(entities.add("").is_err());

        // Check that entities are sorted
        entities.add(aaa).unwrap();
        entities.add(zzz).unwrap();
        let all_entities = entities.sorted_by_name();
        assert_eq!(all_entities[0].name(), aaa);
        assert_eq!(all_entities[1].name(), jean);
        assert_eq!(all_entities[2].name(), zzz);

        // Remove entity with wrong name
        assert!(entities.remove("wrong name").is_err());
        // Remove entity with right name
        entities.remove(aaa).unwrap();
        let all_entities = entities.sorted_by_name();
        assert_eq!(all_entities.len(), 2);
        assert_eq!(all_entities[0].name(), jean);
        assert_eq!(all_entities[1].name(), zzz);
    }

    #[test]
    fn get_entity_by_name() {
        let mut entities = Entities::new();
        assert!(entities.add("Léo paar").is_ok());
        assert!(entities.get_by_name("I dont exist").is_err());
        assert_eq!(
            entities.get_by_name("Léo paar").unwrap().name(),
            clean("Léo paar").unwrap()
        );
    }

    #[test]
    fn set_entity_name() {
        let mut entities = Entities::new();
        let old_name = "Léo";
        let new_name = "Léa";
        assert!(entities.add(old_name).is_ok());

        // Rename existing entity
        assert!(entities.set_name_of(old_name, new_name).is_ok());
        assert!(entities.get_by_name(old_name).is_err());

        // Rename non-existing entity
        assert!(entities
            .set_name_of(old_name, "old name no longer exists")
            .is_err());
    }

    #[test]
    fn work_intervals() {
        let mut entities = Entities::new();
        let name = "Père Du";
        assert!(entities.add(name).is_ok());

        // Add custom work hour
        let interval = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
        assert!(entities
            .add_custom_work_interval_for(name, interval)
            .is_ok());
        let custom_work_hours = entities.get_by_name(name).unwrap().custom_work_hours();
        assert_eq!(custom_work_hours.len(), 1);
        assert_eq!(custom_work_hours[0], interval);

        // Remove custom work hour
        assert!(entities
            .remove_custom_work_interval_for(name, interval)
            .is_ok());
        let custom_work_hours = entities.get_by_name(name).unwrap().custom_work_hours();
        assert!(custom_work_hours.is_empty());
    }
}
