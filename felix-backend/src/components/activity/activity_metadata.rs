use crate::errors::{already_in::AlreadyIn, name_taken::NameTaken, not_in::NotIn, Result};
use crate::ActivityId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

/// Represents a color. Each field should be kept in [0.0; 1.0].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rgba {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl PartialEq for Rgba {
    // Simply round up the floats
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}

impl Hash for Rgba {
    // Simply round up the floats
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in &[self.red, self.green, self.blue, self.alpha] {
            const HASH_PRECISION: f64 = 1_000_000.0;
            ((value * HASH_PRECISION) as usize).hash(state);
        }
    }
}

/// Simple structure holding non-computation related data : id, name, entities.
///
/// We directly store incompatible activities in the ActivityComputationData which is why
/// the entities are not directly computation-related.
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct ActivityMetadata {
    id: ActivityId,
    name: String,
    // BTreeSet implements Hash
    entities: BTreeSet<String>,
    groups: BTreeSet<String>,
    display_color: Rgba,
}

impl ActivityMetadata {
    /// Creates new activity metadata.
    #[must_use]
    pub fn new(id: ActivityId, name: String) -> ActivityMetadata {
        // Default color is blue
        const DEFAULT_COLOR: Rgba = Rgba {
            red: 0.203,
            green: 0.396,
            blue: 0.643,
            alpha: 1.0,
        };

        ActivityMetadata {
            id,
            name,
            entities: BTreeSet::new(),
            groups: BTreeSet::new(),
            display_color: DEFAULT_COLOR,
        }
    }

    // *** Getters ***

    /// Simple getter for the id.
    #[must_use]
    pub fn id(&self) -> ActivityId {
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
    #[must_use]
    pub fn entities_as_set(&self) -> &BTreeSet<String> {
        &self.entities
    }

    /// Getter for the color.
    #[must_use]
    pub fn color(&self) -> Rgba {
        self.display_color
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
    pub fn add_entity(&mut self, entity: String) -> Result<()> {
        if self.entities.insert(entity.clone()) {
            Ok(())
        } else {
            Err(AlreadyIn::entity_already_in_activity(entity, self.name()))
        }
    }

    /// Removes an entity from the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not taking part in the activity.
    pub fn remove_entity(&mut self, entity: &str) -> Result<()> {
        if self.entities.remove(entity) {
            Ok(())
        } else {
            Err(NotIn::entity_not_in_activity(entity, self.name()))
        }
    }

    /// Renames an entity in the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not taking part in the activity or if
    /// an entity with the new name is already taking part in the activity.
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

    /// Add a group to the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is already taking part in the activity.
    pub fn add_group(&mut self, group: String) -> Result<()> {
        if self.groups.contains(&group) {
            Err(AlreadyIn::group_already_in_activity(group, self.name()))
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
    pub fn remove_group(&mut self, group: &str) -> Result<()> {
        if self.groups.contains(group) {
            self.groups.remove(group);
            Ok(())
        } else {
            Err(NotIn::group_not_in_activity(group, self.name()))
        }
    }

    /// Renames the group in the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not taking part in the activity or
    /// if a group with this name is already present in the activity.
    pub fn rename_group(&mut self, old_name: &str, new_name: String) -> Result<()> {
        if self.groups.contains(&new_name) {
            return Err(NameTaken::name_taken_by_group(new_name));
        };

        if self.groups.remove(old_name) {
            self.groups.insert(new_name);
            Ok(())
        } else {
            Err(NotIn::group_not_in_activity(old_name, self.name()))
        }
    }

    /// Sets the color of the activity.
    pub fn set_color(&mut self, color: Rgba) {
        self.display_color = color;
    }
}

impl Eq for ActivityMetadata {}
