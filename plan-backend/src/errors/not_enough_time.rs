use std::error::Error;
use std::fmt;
use gettextrs::gettext as tr;

/// Defines the reason why the entity will not have enough time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WhyNotEnoughTime {
    WorkHoursShortened,
    ActivityAdded,
    ActivityDurationIncreased,
    AddedToGroup,
}

/// Throw this error when the requested operation leaves an entity with not enough time.
///
/// The error is built from functions in the form 'reason\_for(entity_name)'. 
///
/// # Example
///
/// ```
/// use plan_backend::errors::not_enough_time::{NotEnoughTime, WhyNotEnoughTime};
///
/// let error = NotEnoughTime::activity_added_for("Entity Name", "Activity");
///
/// assert_eq!(format!("{}", error),
/// "Entity Name will not have enough time if they are added to 'Activity'.");
/// assert_eq!(error.entity_name(), "Entity Name");
/// assert_eq!(error.why(), WhyNotEnoughTime::ActivityAdded);
/// ```
#[derive(Debug, Clone)]
pub struct NotEnoughTime {
    reason: WhyNotEnoughTime,
    entity_name: String,
    // Activity or group name
    associated_name: Option<String>,
}

impl fmt::Display for NotEnoughTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self.reason {
            WhyNotEnoughTime::WorkHoursShortened => tr("if their work hours are shortened"),

            WhyNotEnoughTime::ActivityAdded => {
                let activity_name = self.associated_name.as_ref().expect("Error when adding activity but activity name was not supplied");
                format!("{} '{}'", tr("if they are added to"), activity_name)
            }

            WhyNotEnoughTime::ActivityDurationIncreased => {
                let activity_name = self.associated_name.as_ref().expect("Error when setting activity duration but activity name was not supplied");
                    format!("{} '{}' {}", tr("if the duration of"), activity_name, tr("is increased"))
                    }

            WhyNotEnoughTime::AddedToGroup => {
                let group_name = self.associated_name.as_ref().expect("Error when adding to group but group name was not supplied");
                format!("{} '{}'", tr("if they take part in the activities of the group"), group_name)
            }
        };

        write!(f, "{} {} {}.", self.entity_name, tr("will not have enough time"), reason)
    }
}

impl Error for NotEnoughTime {}

impl NotEnoughTime {
    // Constructors
    pub fn activity_added_for<S1, S2>(entity_name: S1, activity_name: S2) -> Box<NotEnoughTime> where S1: Into<String>, S2: Into<String> {
        Box::new(NotEnoughTime { reason: WhyNotEnoughTime::ActivityAdded, entity_name: entity_name.into(), associated_name: Some(activity_name.into()) })
    }

    pub fn activity_duration_too_long_for<S1, S2>(entity_name: S1, activity_name: S2) -> Box<NotEnoughTime> where S1: Into<String>, S2: Into<String> {
        Box::new(NotEnoughTime { reason: WhyNotEnoughTime::ActivityDurationIncreased, entity_name: entity_name.into(), associated_name: Some(activity_name.into())})
    }

    pub fn work_hours_shortened_for<S>(entity_name: S) -> Box<NotEnoughTime> where S: Into<String> {
        Box::new(NotEnoughTime { reason: WhyNotEnoughTime::WorkHoursShortened, entity_name: entity_name.into(), associated_name: None })
    }

    pub fn added_to_group<S1, S2>(entity_name: S1, group_name: S2) -> Box<NotEnoughTime> where S1: Into<String>, S2: Into<String> {
        Box::new(NotEnoughTime { reason: WhyNotEnoughTime::AddedToGroup, entity_name: entity_name.into(), associated_name: Some(group_name.into()) })
    }

    // Getters
    pub fn entity_name(&self) -> String {
        self.entity_name.clone()
    }

    pub fn why(&self) -> WhyNotEnoughTime {
        self.reason
    }
}

