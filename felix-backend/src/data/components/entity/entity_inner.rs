use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityInner {
    name: String,
    mail: String,
    send_me_a_mail: bool,
}

impl EntityInner {
    /// Instantiates a new EntityInner.
    pub fn new(name: String) -> EntityInner {
        EntityInner {
            name,
            mail: String::new(),
            send_me_a_mail: false,
        }
    }

    // *** Getters ***

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn mail(&self) -> &String {
        &self.mail
    }

    #[must_use]
    pub fn send_me_a_mail(&self) -> bool {
        self.send_me_a_mail
    }

    // *** Setters ***

    /// Sets the name of the entity.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the mail of the entity. The given mail is always accepted.
    pub fn set_mail(&mut self, mail: String) {
        self.mail = mail;
    }

    /// Call with true if a mail should be sent to the entity, else with false.
    /// Never fails.
    pub fn set_send_me_a_mail(&mut self, send: bool) {
        self.send_me_a_mail = send;
    }
}

// No tests, functions are tested in tests directory
