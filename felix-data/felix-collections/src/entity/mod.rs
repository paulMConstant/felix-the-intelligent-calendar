mod entities;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub type EntityName = String;

pub use entities::Entities;

/// Represents any entity which can be used by an activity.
///
/// Entities include people, of course, but also rooms, which have a schedule like any human
/// (free or not free at any point during the day). This extends to any tool or thing which
/// does not have the gift of ubiquity.
///
/// Entities have unique names. An entity may not have the same name as a group.
///
/// This structure is read-only. If you wish to create or modify an entity, use the Data object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    name: EntityName,
}

impl Entity {
    /// Creates a new entity with the given name.
    #[must_use]
    fn new(name: String) -> Entity {
        Entity { name }
    }

    // *** Getters ***
    // This is the only public API. To modify an entity, users must use the Data API.
    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    // *** Private Setters ***

    /// Sets the name of the entity.
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().cmp(&other.name())
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
