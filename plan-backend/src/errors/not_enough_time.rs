use std::error::Error;
use std::fmt;
use gettextrs::gettext as tr;

/// Defines the reason why the entity will not have enough time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WhyNotEnoughTime {
    WorkHoursShortened,
    ActivityAdded,
    ActivityDurationIncreased,
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
/// let error = NotEnoughTime::activity_added_for("Entity Name");
///
/// assert_eq!(format!("{}", error),
/// "'Entity Name' will not have enough time if they are added to this activity.");
/// assert_eq!(error.entity_name(), "Entity Name");
/// assert_eq!(error.why(), WhyNotEnoughTime::ActivityAdded);
/// ```
#[derive(Debug, Clone)]
pub struct NotEnoughTime {
    reason: WhyNotEnoughTime,
    entity_name: String,
}

impl fmt::Display for NotEnoughTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self.reason {
            WhyNotEnoughTime::WorkHoursShortened => tr("if their work hours are shortened"),
            WhyNotEnoughTime::ActivityAdded => tr("if they are added to this activity"),
            WhyNotEnoughTime::ActivityDurationIncreased => tr("if the duration of this activity is increased"),
        };

        write!(f, "'{}' {} {}.", self.entity_name, tr("will not have enough time"), reason)
    }
}

impl Error for NotEnoughTime {}

impl NotEnoughTime {
    // Constructors
    pub fn activity_added_for<S>(entity_name: S) -> NotEnoughTime where S: Into<String> {
        NotEnoughTime { reason: WhyNotEnoughTime::ActivityAdded, entity_name: entity_name.into() }
    }

    pub fn activity_duration_too_long_for<S>(entity_name: S) -> NotEnoughTime where S: Into<String> {
        NotEnoughTime { reason: WhyNotEnoughTime::ActivityDurationIncreased, entity_name: entity_name.into() }
    }

    pub fn work_hours_shortened_for<S>(entity_name: S) -> NotEnoughTime where S: Into<String> {
        NotEnoughTime { reason: WhyNotEnoughTime::WorkHoursShortened, entity_name: entity_name.into() }
    }

    // Getters
    pub fn entity_name(&self) -> String {
        self.entity_name.clone()
    }

    pub fn why(&self) -> WhyNotEnoughTime {
        self.reason
    }
}

