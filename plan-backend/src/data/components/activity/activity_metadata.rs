use std::collections::HashSet;

/// Simple structure holding non-computation related data : id, name, entities.
///
/// We directly store incompatible activities in the ActivityComputationData which is why
/// the entities are not directly computation-related.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityMetadata {
    id: u16,
    name: String,
    entities: HashSet<String>,
    groups: HashSet<String>,
}

impl ActivityMetadata {
    /// Creates new activity metadata.
    #[must_use]
    pub fn new(id: u16, name: String) -> ActivityMetadata {
        ActivityMetadata {
            id,
            name,
            entities: HashSet::new(),
            groups: HashSet::new(),
        }
    }

    // *** Getters ***

    /// Simple getter for the id.
    #[must_use]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Simple getter for the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<String> {
        let mut entities_vec: Vec<String> = self.entities.iter().cloned().collect();
        entities_vec.sort();
        entities_vec
    }

    /// Simple getter for the entities, sorted by name.
    #[must_use]
    pub fn groups_sorted(&self) -> Vec<String> {
        let mut groups_vec: Vec<String> = self.groups.iter().cloned().collect();
        groups_vec.sort();
        groups_vec
    }

    /// Getter for the entities, not sorted.
    pub fn entities_as_set(&self) -> &HashSet<String> {
        &self.entities
    }

    // *** Setters ***

    // No setter for the id. The id should be unique and never change.

    /// Simple setter for the name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Adds an entity to the activity.
    /// The entities are always sorted.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is already taking part in the activity.
    #[must_use]
    pub fn add_entity(&mut self, entity: String) -> Result<(), String> {
        if self.entities.insert(entity.clone()) {
            Ok(())
        } else {
            Err(format!(
                "{} is already taking part in the activity '{}'.",
                entity,
                self.name()
            ))
        }
    }

    /// Removes an entity from the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not taking part in the activity.
    #[must_use]
    pub fn remove_entity(&mut self, entity: &String) -> Result<(), String> {
        if self.entities.remove(entity) {
            Ok(())
        } else {
            Err(format!(
                "{} is not taking part in the activity '{}'.",
                entity,
                self.name()
            ))
        }
    }

    /// Renames an entity in the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not taking part in the activity or if
    /// an entity with the new name is already taking part in the activity.
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

    /// Add a group to the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is already taking part in the activity.
    #[must_use]
    pub fn add_group(&mut self, group: String) -> Result<(), String> {
        if self.groups.contains(&group) {
            Err(format!(
                "The group '{}' is already in the activity '{}'.",
                group,
                self.name()
            ))
        } else {
            self.groups.insert(group);
            Ok(())
        }
    }

    /// Removes a group from the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is already taking part in the activity.
    #[must_use]
    pub fn remove_group(&mut self, group: &String) -> Result<(), String> {
        if self.groups.contains(group) {
            self.groups.remove(group);
            Ok(())
        } else {
            Err(format!(
                "The group '{}' is not in the activity '{}'.",
                group,
                self.name()
            ))
        }
    }
}

// No tests, functions are tested in tests directory
