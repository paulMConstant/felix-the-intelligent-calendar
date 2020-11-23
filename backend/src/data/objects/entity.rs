pub mod entities;
mod entity_inner;

use super::time::time_interval::TimeInterval;
use entity_inner::EntityInner;
use std::cmp::Ordering;

/// Represents any entity which can be used by an activity : person, tool, room...
///
/// This structure is read-only. If you wish to create or modify an entity, use the Data object.
#[derive(Debug, Clone)]
pub struct Entity {
    inner: EntityInner,
}

impl Entity {
    /// Creates a new entity with the given name.
    #[must_use]
    fn new<S>(name: S) -> Entity
    where
        S: Into<String>,
    {
        Entity {
            inner: EntityInner::new(name),
        }
    }

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

impl Eq for Entity {}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
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
