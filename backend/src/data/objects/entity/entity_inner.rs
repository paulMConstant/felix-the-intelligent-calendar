use super::super::super::{TimeInterval, WorkHours};

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
}

#[cfg(test)]
mod tests {
    use super::super::super::time::Time;
    use super::super::Entity;
    use super::*;

    #[test]
    fn modify_metadata() {
        let mut entity = Entity::new("Pascal");

        // Set name
        let new_name = "Marie";
        entity.inner.set_name(new_name);
        assert_eq!(entity.name(), new_name.to_owned());

        // Set mail
        let mail = "marie@xyz.com";
        entity.inner.set_mail(mail);
        assert_eq!(entity.mail(), mail.to_owned());

        // Set send_me_a_mail
        entity.inner.set_send_me_a_mail(true);
        assert_eq!(entity.send_me_a_mail(), true);
        entity.inner.set_send_me_a_mail(false);
        assert_eq!(entity.send_me_a_mail(), false);
    }

    #[test]
    fn modify_custom_work_hours() {
        let mut entity = Entity::new("Test");
        // Add work interval
        let interval = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
        assert!(entity.inner.add_work_interval(interval).is_ok());
        assert_eq!(entity.custom_work_hours().len(), 1);
        assert_eq!(entity.custom_work_hours()[0], interval);

        // Add overlapping interval
        let overlapping_interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
        assert!(entity
            .inner
            .add_work_interval(overlapping_interval)
            .is_err());

        // Try removing wrong time interval
        let nonexistent_interval = TimeInterval::new(Time::new(7, 0), Time::new(10, 0));
        assert!(entity
            .inner
            .remove_work_interval(nonexistent_interval)
            .is_err());
        // Remove time interval
        assert!(entity.inner.remove_work_interval(interval).is_ok());
        assert!(entity.custom_work_hours().is_empty());
    }
}
