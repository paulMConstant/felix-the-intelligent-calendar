mod objects {
    pub mod activity;
    pub mod entity;
    pub mod time;
}
mod helpers {
    pub mod clean_string;
}

use objects::{
    activity::activities::Activities, entity::entities::Entities, time::work_hours::WorkHours,
};
pub use objects::{
    activity::Activity,
    entity::Entity,
    time::{time_interval::TimeInterval, Time, MIN_TIME_DISCRETIZATION},
};

/// Stores, calculates and maintains coherency between entities, work hours and activities.
///
/// This is the only mutable object in the data module.
///
/// # Examples
///
/// Add and remove work intervals :
///
/// ```
/// # use backend::data::{Data, Time, TimeInterval};
/// let mut data = Data::new();
///
/// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
/// let afternoon_shift = TimeInterval::new(Time::new(14, 0), Time::new(18, 0));
/// data.add_work_interval(afternoon_shift).unwrap();
/// data.add_work_interval(morning_shift).unwrap();
///
/// let work_hours = data.work_hours();
/// assert_eq!(work_hours[0], morning_shift);
/// assert_eq!(work_hours[1], afternoon_shift);
///
/// data.remove_work_interval(morning_shift).unwrap();
/// let work_hours = data.work_hours();
/// assert_eq!(work_hours.len(), 1);
/// ```
///
/// Add, remove and modify entities :
///
/// ```
/// # use backend::data::{Data, Time, TimeInterval};
/// let mut data = Data::new();
///
/// let entity_name = data.add_entity("Bernard").unwrap().name();
///
/// let mail = "bernard@xyz.com";
/// data.set_entity_mail(entity_name.clone(), mail.clone()).unwrap();
///
/// let custom_morning_shift = TimeInterval::new(Time::new(10, 0), Time::new(12, 0));
/// data.add_custom_work_interval_for(entity_name.clone(), custom_morning_shift);
///
/// let new_name = "Jean";
/// data.set_entity_name(entity_name, new_name);
///
/// let send_mail = true;
/// data.set_send_mail_to(new_name, send_mail).unwrap();
///
/// let entity = data.entity(new_name).unwrap();
///
/// assert!(entity.send_me_a_mail(), send_mail);
/// assert_eq!(entity.mail(), mail);
/// assert_eq!(entity.custom_work_hours()[0], custom_morning_shift);
///
/// data.remove_entity(new_name).unwrap();
/// assert!(data.entities_sorted().is_empty());
/// ```
///
/// Add, remove and modify activities :
///
/// ```
/// # use backend::data::{Data, Time, TimeInterval};
/// let mut data = Data::new();
///
/// let activity_id = data.add_activity("My Activity").unwrap().id();
/// let entity_name = data.add_entity("My Entity").unwrap().name();
///
/// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
/// data.add_work_interval(morning_shift).unwrap();
///
/// data.set_activity_duration(activity_id, Time::new(1, 0));
/// data.add_participant_to_activity(activity_id, entity_name);
/// ```
pub struct Data {
    work_hours: WorkHours,
    entities: Entities,
    activities: Activities,
}

impl Data {
    /// Creates a new data object.
    pub fn new() -> Data {
        Data {
            work_hours: WorkHours::new(),
            entities: Entities::new(),
            activities: Activities::new(),
        }
    }

    // *** Code Organization ***
    // - Getter for collection
    // - Individual getter
    // - Add
    // - Remove
    // - Modify
    // - Tests are in tests folder next to src (public API)
    //
    // Collections organized in the following order :
    // - Work Hours
    // - Entities
    // - Entities (Custom Work Hours)
    // - Activities

    // *** Work Hours ***
    // - Getter for collection
    // - Add
    // - Remove

    /// Returns an immutable reference to the work hours.
    #[must_use]
    pub fn work_hours(&self) -> Vec<TimeInterval> {
        self.work_hours.work_intervals().clone()
    }

    /// Adds the given time interval to the work hours.
    ///
    /// Work hours are always sorted.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps with the existing work intervals.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let (beginning, end) = (Time::new(8, 0), Time::new(12, 0));
    /// let interval = TimeInterval::new(beginning, end);
    /// assert!(data.add_work_interval(interval).is_ok());
    ///
    /// let overlapping_interval = TimeInterval::new(beginning, Time::new(9, 0));
    /// assert!(data.add_work_interval(overlapping_interval).is_err());
    /// ```
    #[must_use]
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        self.work_hours.add_work_interval(interval)
        // TODO update possible insertion times
    }

    /// Removes the time interval with given beginning from the work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the time interval is not found or if the time interval can't be removed
    /// because an entity no longer has any time left.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let (beginning, end) = (Time::new(8, 0), Time::new(12, 0));
    /// let interval = TimeInterval::new(beginning, end);
    /// data.add_work_interval(interval).unwrap();
    ///
    /// assert!(data.remove_work_interval(interval).is_ok());
    /// assert!(data.work_hours().is_empty());
    ///
    /// let nonexistent_interval = interval;
    /// assert!(data.remove_work_interval(interval).is_err());
    /// ```
    #[must_use]
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<(), String> {
        self.work_hours.remove_work_interval(interval)
        // TODO check if every entity has enough time left
        // TODO update possible insertion times
    }

    // *** Entities ***
    // - Getter for collection
    // - Individual getter
    // - Add
    // - Remove
    // - Modify

    /// Returns vector of immutable references to the entities, sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<&Entity> {
        self.entities.sorted_by_name()
    }

    /// Gets an immutable reference to the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = "Jeanne";
    /// data.add_entity(name).unwrap();
    /// assert!(data.entity(name).is_ok());
    ///
    /// let invalid_name = "Jean";
    /// assert!(data.entity(invalid_name).is_err());
    /// ```
    #[must_use]
    pub fn entity<S>(&self, name: S) -> Result<&Entity, String>
    where
        S: Into<String>,
    {
        self.entities.get_by_name(name)
    }

    /// Adds an entity with the formatted given name.
    ///
    /// Returns an immutable reference to the added entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty or if the name is already taken.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = "Jeanne";
    /// assert!(data.add_entity(name).is_ok());
    /// // Name already taken
    /// assert!(data.add_entity(name).is_err());
    /// ```
    #[must_use]
    pub fn add_entity<S>(&mut self, name: S) -> Result<&Entity, String>
    where
        S: Into<String>,
    {
        self.entities.add(name)
    }

    /// Removes the entity with the formatted given name.
    ///
    /// If the entity is taking pat in any activity, it is removed from them.
    /// Returns the name of the removed entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = "Jeanne";
    /// assert!(data.add_entity(name).is_ok());
    /// assert!(data.remove_entity(name).is_ok());
    /// // Entity does not exist
    /// assert!(data.remove_entity(name).is_err());
    /// ```
    #[must_use]
    pub fn remove_entity<S>(&mut self, name: S) -> Result<String, String>
    where
        S: Into<String>,
    {
        let name = name.into();
        // First, remove in entities to check for any error
        self.entities.remove(name.clone())?;
        // If the entity was successfuly removed in entities, remove it
        // in all activities
        Ok(self.activities.remove_participant_from_all(name)?)
    }

    /// Renames the entity with the formatted given name.
    ///
    /// If the entity is taking part in any activity, it is renamed there as well.
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or if the name is already taken.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let name = "Jeanne";
    /// let new_name = "Jean";
    ///
    /// assert!(data.add_entity(name).is_ok());
    /// assert!(data.set_entity_name(name, new_name).is_ok());
    ///
    /// let invalid_name = name;
    /// assert!(data.set_entity_name(invalid_name, "other name").is_err());
    /// ```
    #[must_use]
    pub fn set_entity_name<S1, S2>(&mut self, old_name: S1, new_name: S2) -> Result<String, String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        // First, rename in entities to check for any error
        let old_name = old_name.into();
        let new_name = self.entities.set_name_of(old_name.clone(), new_name)?;
        // Then, rename in activities
        Ok(self
            .activities
            .rename_participant_in_all(old_name, new_name.clone())?)
    }

    /// Sets the mail of the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    /// let (name, mail) = ("Jeanne", "jeanne@xyz.com");
    /// assert!(data.add_entity(name).is_ok());
    /// assert!(data.set_entity_mail(name, mail).is_ok());
    /// assert_eq!(data.entity(name).unwrap().mail(), mail);
    /// ```
    #[must_use]
    pub fn set_entity_mail<S1, S2>(&mut self, entity_name: S1, mail: S2) -> Result<(), String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.entities.set_mail_of(entity_name, mail)
    }

    /// Set to true to send mails to the entity with formatted given name.
    ///
    /// # Errors
    ///
    /// Returs Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    /// let (name, send) = ("Jeanne", true);
    /// data.add_entity(name).unwrap();
    /// assert_eq!(data.entity(name).unwrap().send_me_a_mail(), false);
    /// assert!(data.set_send_mail_to(name, send).is_ok());
    /// assert!(data.entity(name).unwrap().send_me_a_mail());
    /// ```
    #[must_use]
    pub fn set_send_mail_to<S>(&mut self, entity_name: S, send: bool) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.entities.set_send_mail_to(entity_name, send)
    }

    // *** Custom Work Hours ***
    // - Individual getter
    // - Add / Remove

    /// Returns the free time of an entity (total time in work hours - time taken by activities).
    ///
    /// The activities should never take more time than the total time ; should that happen,
    /// Time::new(0, 0) is returned.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12,0));
    /// data.add_work_interval(morning_shift).unwrap();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    /// let activity_id = data.add_activity("Activity").unwrap().id();
    /// let activity_duration = Time::new(1, 0);
    /// data.set_activity_duration(activity_id, activity_duration).unwrap();
    /// data.add_participant_to_activity(activity_id, name.clone());
    ///
    /// // Total time is 4 hours, time taken by activity is 1 hour.
    /// assert_eq!(data.free_time_of(name).unwrap(), Time::new(3, 0));
    /// ```
    #[must_use]
    pub fn free_time_of<S>(&self, entity_name: S) -> Result<Time, String>
    where
        S: Into<String>,
    {
        let entity_name = entity_name.into();
        let total_duration: Time = self
            .work_hours_of(entity_name.clone())?
            .iter()
            .map(|interval| interval.duration())
            .sum();
        let activity_duration: Time = self
            .activities
            .sorted_by_name()
            .iter()
            .filter_map(|activity| {
                if activity.participants_sorted().contains(&entity_name) {
                    Some(activity.duration())
                } else {
                    None
                }
            })
            .sum();
        Ok(if total_duration < activity_duration {
            Time::new(0, 0)
        } else {
            total_duration - activity_duration
        })
    }

    /// Returns the work hours of the entity with the formatted given name.
    ///
    /// If the entity has custom work hours, returns them, else returns the global work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity with given name is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = data.add_entity("Jeanne").unwrap().name();
    ///
    /// let regular_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(regular_work_interval);
    /// assert_eq!(data.work_hours_of(name.clone()).unwrap(), data.work_hours());
    ///
    /// let custom_work_interval = TimeInterval::new(Time::new(10, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(name.clone(), custom_work_interval);
    /// let expected = data.entity(name.clone()).unwrap().custom_work_hours();
    /// assert_eq!(data.work_hours_of(name).unwrap(), expected);
    /// ```
    #[must_use]
    pub fn work_hours_of<S>(&self, entity_name: S) -> Result<Vec<TimeInterval>, String>
    where
        S: Into<String>,
    {
        let custom_work_hours = self.entity(entity_name)?.custom_work_hours();
        Ok(if custom_work_hours.len() == 0 {
            self.work_hours().clone()
        } else {
            custom_work_hours
        })
    }

    /// Adds a custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval overlaps with others.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = "Jeanne";
    /// data.add_entity(name);
    ///
    /// let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// let overlapping_interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    ///
    /// assert!(data.add_custom_work_interval_for(name, custom_work_interval).is_ok());
    /// assert!(data.add_custom_work_interval_for(name, overlapping_interval).is_err());
    /// assert_eq!(data.entity(name).unwrap().custom_work_hours()[0], custom_work_interval);
    /// ```
    #[must_use]
    pub fn add_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.entities
            .add_custom_work_interval_for(entity_name, interval)
        // TODO update possible insertion times
    }

    /// Removes the given custom work interval for the entity with the formatted given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found or the work interval is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let name = "Jeanne";
    /// data.add_entity(name);
    ///
    /// let custom_work_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(name, custom_work_interval).unwrap();
    ///
    /// assert_eq!(data.work_hours_of(name).unwrap().len(), 1);
    /// assert!(data.remove_custom_work_interval_for(name, custom_work_interval).is_ok());
    /// assert!(data.entity(name).unwrap().custom_work_hours().is_empty());
    ///
    /// let nonexistent_interval = custom_work_interval;
    /// assert!(data.remove_custom_work_interval_for(name, nonexistent_interval).is_err());
    /// ```
    #[must_use]
    pub fn remove_custom_work_interval_for<S>(
        &mut self,
        entity_name: S,
        interval: TimeInterval,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        self.entities
            .remove_custom_work_interval_for(entity_name, interval)
        // TODO check if the entity has enough free time
        // TODO update possible insertion times
    }

    // *** Activities ***
    // - Getter for collection
    // - Individual getter
    // - Add
    // - Remove
    // - Modify

    /// Returns the activities, sorted by name.
    #[must_use]
    pub fn activities_sorted(&self) -> Vec<&Activity> {
        self.activities.sorted_by_name()
    }

    /// Returns an immutable reference to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    #[must_use]
    pub fn activity(&self, id: u16) -> Result<&Activity, String> {
        self.activities.get_by_id(id)
    }

    /// Adds an activity with the formatted given name.
    ///
    /// Automatically assigns a unique id.
    /// Returns an immutable reference to the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_name = "Activity";
    /// let activity = data.add_activity(activity_name);
    /// assert!(activity.is_ok());
    /// let activity_id = activity.unwrap().id();
    ///
    /// assert_eq!(data.activities_sorted().len(), 1);
    /// assert!(data.activity(activity_id).is_ok());
    /// ```
    #[must_use]
    pub fn add_activity<S>(&mut self, name: S) -> Result<&Activity, String>
    where
        S: Into<String>,
    {
        self.activities.add(name)
    }

    /// Removes the activity with the given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let invalid_id = activity_id + 1;
    ///
    /// assert!(data.remove_activity(invalid_id).is_err());
    /// assert_eq!(data.activities_sorted().len(), 1);
    /// assert!(data.remove_activity(activity_id).is_ok());
    /// assert!(data.activities_sorted().is_empty());
    /// ```
    #[must_use]
    pub fn remove_activity(&mut self, id: u16) -> Result<(), String> {
        self.activities.remove(id)
    }

    /// Adds the participant with given name to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the participant is not found,
    /// if the participant does not have enough time left
    /// or the participant is already taking part in the activity.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity_name = data.add_entity("Bernard").unwrap().name();
    ///
    /// // Make sure the participant has enough time !
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_work_interval(morning_shift).unwrap();
    ///
    /// assert!(data.add_participant_to_activity(activity_id, entity_name.clone()).is_ok());
    ///
    /// let participants = data.activity(activity_id).unwrap().participants_sorted();
    /// assert_eq!(participants.len(), 1);
    /// assert_eq!(participants[0], entity_name);
    /// ```
    #[must_use]
    pub fn add_participant_to_activity<S>(
        &mut self,
        id: u16,
        participant_name: S,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the entity exists and formats name
        let participant_name = self.entity(participant_name)?.name();
        // Check its free time
        let free_time = self.free_time_of(participant_name.clone())?;
        if free_time < self.activity(id)?.duration() {
            Err(format!(
                "{} does not have enough time left for this activity.",
                participant_name
            ))
        } else {
            // Add the entity to the activity
            self.activities.add_participant(id, participant_name)
        }
    }

    /// Removes the participant with given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found, if the participant is not found
    /// or the participant is not taking part in the activity.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, TimeInterval};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let entity_name = data.add_entity("Bernard").unwrap().name();
    ///
    /// // Make sure the participant has enough time before adding him to an activity
    /// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    /// data.add_custom_work_interval_for(entity_name.clone(), morning_shift).unwrap();
    ///
    /// data.add_participant_to_activity(activity_id, entity_name.clone()).unwrap();
    /// assert!(data.remove_participant_from_activity(activity_id, entity_name).is_ok());
    /// assert!(data.activity(activity_id).unwrap().participants_sorted().is_empty());
    /// ```
    #[must_use]
    pub fn remove_participant_from_activity<S>(
        &mut self,
        id: u16,
        participant_name: S,
    ) -> Result<(), String>
    where
        S: Into<String>,
    {
        // Check that the entity exists
        let participant_name = self.entity(participant_name)?.name();
        // Remove the entity from the activity
        self.activities.remove_participant(id, participant_name)
    }

    /// Sets the name of the activity with given id with the formatted given name.
    ///
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or the formatted name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::Data;
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    ///
    /// let new_name = "New Name";
    /// assert!(data.set_activity_name(activity_id, new_name).is_ok());
    /// assert_eq!(data.activity(activity_id).unwrap().name(), new_name);
    /// ```
    #[must_use]
    pub fn set_activity_name<S>(&mut self, id: u16, name: S) -> Result<String, String>
    where
        S: Into<String>,
    {
        self.activities.set_name(id, name)
    }

    /// Sets the duration of the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the activity is not found or the duration is too short
    /// (< MIN\_TIME\_DISCRETIZATION) or an entity does not have enough time left.
    ///
    /// # Example
    ///
    /// ```
    /// # use backend::data::{Data, Time, MIN_TIME_DISCRETIZATION};
    /// let mut data = Data::new();
    ///
    /// let activity_id = data.add_activity("Test").unwrap().id();
    /// let min_valid_duration = MIN_TIME_DISCRETIZATION;
    ///
    /// assert!(data.set_activity_duration(activity_id, min_valid_duration).is_ok());
    /// assert_eq!(data.activity(activity_id).unwrap().duration(), min_valid_duration);
    /// ```
    #[must_use]
    pub fn set_activity_duration(&mut self, id: u16, new_duration: Time) -> Result<(), String> {
        // If the duration is longer than the previous one, check for conflicts
        let activity = self.activity(id)?;
        let current_duration = activity.duration();
        if new_duration > current_duration {
            // Duration is longer - check if it conflicts with participant's schedule
            let required_free_time = new_duration - current_duration; // > 0
            for entity_name in activity.participants_sorted() {
                if self.free_time_of(entity_name.clone())? < required_free_time {
                    return Err(format!(
                        "{} does not have enough time for the new duration.",
                        entity_name
                    ));
                };
            }
        };
        self.activities.set_duration(id, new_duration)
    }
}
