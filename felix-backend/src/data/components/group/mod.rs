mod group_inner;
pub mod groups;

use group_inner::GroupInner;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

/// A group is an aggregation of entities.
///
/// Groups have unique names. A group may not have the same name as an entity.
///
/// This structure is read-only. If you wish to create or modify a group, use the Data object.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Group {
    inner: GroupInner,
}

impl Group {
    /// Creates a new group with the given name.
    fn new(name: String) -> Group {
        Group {
            inner: GroupInner::new(name),
        }
    }

    // *** Getters ***
    // This is the only public API, no setters. To modify a group, use the inner field.
    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.inner.name().clone()
    }

    /// Simple getter for the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<String> {
        self.inner.entities_sorted().into_iter().cloned().collect()
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
