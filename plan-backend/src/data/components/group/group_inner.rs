use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    #[must_use]
    pub fn add_entity(&mut self, entity_name: String) -> Result<(), String> {
        if self.entities.insert(entity_name.clone()) {
            Ok(())
        } else {
            Err(format!(
                "{} is already a member of the group '{}'.",
                entity_name,
                self.name.clone()
            ))
        }
    }

    /// Removes an entity from the group.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not in the group.
    #[must_use]
    pub fn remove_entity(&mut self, entity_name: &String) -> Result<(), String> {
        if self.entities.remove(entity_name) {
            Ok(())
        } else {
            Err(format!(
                "{} is not a member of the group '{}'.",
                entity_name,
                self.name.clone()
            ))
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
    #[must_use]
    pub fn rename_entity(&mut self, old_name: &String, new_name: String) -> Result<(), String> {
        if self.entities.contains(&new_name) {
            return Err(format!("The entity '{}' already exists.", new_name));
        };

        if self.entities.remove(old_name) {
            self.entities.insert(new_name);
            Ok(())
        } else {
            Err(format!(
                "{} is not taking part in the activity '{}'.",
                old_name,
                self.name()
            ))
        }
    }
}
