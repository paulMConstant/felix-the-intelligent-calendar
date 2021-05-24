use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityInner {
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
}

// No tests, functions are tested in tests directory
