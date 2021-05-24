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
    mail: String,
    send_me_a_mail: bool,
}

impl Entity {
    /// Creates a new entity with the given name.
    #[must_use]
    fn new(name: String) -> Entity {
        Entity {
            name,
            mail: String::new(),
            send_me_a_mail: false,
        }
    }

    // *** Getters ***
    // This is the only public API. To modify an entity, users must use the Data API.
    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Simple getter for the mail.
    #[must_use]
    pub fn mail(&self) -> String {
        self.mail.clone()
    }

    /// Simple getter to check if a mail should be sent to the entity.
    #[must_use]
    pub fn send_me_a_mail(&self) -> bool {
        self.send_me_a_mail
    }

    // *** Private Setters ***

    /// Sets the name of the entity.
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the mail of the entity. The given mail is always accepted.
    fn set_mail(&mut self, mail: String) {
        self.mail = mail;
    }

    /// Call with true if a mail should be sent to the entity, else with false.
    /// Never fails.
    fn set_send_me_a_mail(&mut self, send: bool) {
        self.send_me_a_mail = send;
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
