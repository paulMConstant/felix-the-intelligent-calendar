use super::super::super::WorkHours;
use crate::data::TimeInterval;

#[derive(Debug, Clone)]
pub struct EntityInner {
    name: String,
    mail: String,
    send_me_a_mail: bool,
    custom_work_hours: WorkHours,
}

impl EntityInner {
    /// Instantiates a new EntityInner.
    pub fn new<S>(name: S) -> EntityInner
    where
        S: Into<String>,
    {
        EntityInner {
            name: name.into(),
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
    ///
    /// The name is not formatted or checked, this is done by the entities collection.
    /// It is easier to have the entities collection do it because names are checked for addition
    /// of entities, renaming, checking for existence, etc. Having the entities collection do it
    /// keeps it it one place.
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.name = name.into();
    }

    /// Sets the mail of the entity. The given mail is always accepted.
    pub fn set_mail<S>(&mut self, mail: S)
    where
        S: Into<String>,
    {
        self.mail = mail.into();
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
    #[must_use]
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        self.custom_work_hours.add_work_interval(interval)
    }

    /// Removes the given custom work hour interval from the entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval is not found.
    #[must_use]
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        self.custom_work_hours.remove_work_interval(interval)
    }

    /// Removes a work interval to the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the work interval is not found or if the new interval overlaps
    /// with existing ones.
    #[must_use]
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<(), String> {
        self.custom_work_hours
            .update_work_interval(old_interval, new_interval)
    }
}

// No tests, functions are tested in tests directory
