pub mod entities;
mod entity_inner;

use crate::data::TimeInterval;
use entity_inner::EntityInner;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

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
    inner: EntityInner,
}

impl Entity {
    /// Creates a new entity with the given name.
    #[must_use]
    fn new(name: String) -> Entity {
        Entity {
            inner: EntityInner::new(name),
        }
    }

    // *** Getters ***
    // This is the only public API. To modify an entity, use the inner field.
    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.inner.name().clone()
    }

    /// Simple getter for the mail.
    #[must_use]
    pub fn mail(&self) -> String {
        self.inner.mail().clone()
    }

    /// Simple getter to check if a mail should be sent to the entity.
    #[must_use]
    pub fn send_me_a_mail(&self) -> bool {
        self.inner.send_me_a_mail()
    }

    /// Simple getter for the custom work hours of the entity.
    /// If you wish to fetch the work hours of the entity, use Data::work_hours_of().
    #[must_use]
    pub fn custom_work_hours(&self) -> Vec<TimeInterval> {
        self.inner.custom_work_hours().clone()
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
