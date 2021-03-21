use super::super::super::WorkHours;
use crate::data::TimeInterval;
use crate::errors::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityInner {
    name: String,
    mail: String,
    send_me_a_mail: bool,
    custom_work_hours: WorkHours,
}

impl EntityInner {
    /// Instantiates a new EntityInner.
    pub fn new(name: String) -> EntityInner {
        EntityInner {
            name,
            mail: String::new(),
            send_me_a_mail: false,
            custom_work_hours: WorkHours::new(),
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

    #[must_use]
    pub fn custom_work_hours(&self) -> &Vec<TimeInterval> {
        self.custom_work_hours.work_intervals()
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

    /// Adds a custom work hour interval to the entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval is overlapping with an existing one.
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.custom_work_hours.add_work_interval(interval)
    }

    /// Removes the given custom work hour interval from the entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval is not found.
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.custom_work_hours.remove_work_interval(interval)
    }

    /// Removes a work interval to the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the work interval is not found or if the new interval overlaps
    /// with existing ones.
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<()> {
        self.custom_work_hours
            .update_work_interval(old_interval, new_interval)
    }
}

// No tests, functions are tested in tests directory
