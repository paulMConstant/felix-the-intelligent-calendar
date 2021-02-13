use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

use crate::data::{Time, MIN_TIME_DISCRETIZATION};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InvalidOrNotComputed {
    Invalid,
    NotComputed,
}

/// Throw this error when the user tries to insert an activity where it cannot be inserted.
///
/// # Example
///
/// ```
/// use felix_backend::errors::invalid_insertion::InvalidInsertion;
/// use felix_backend::data::Time;
///
/// let error = InvalidInsertion::insertion_not_in_computed_insertions("Activity", Time::new(10, 0));
///
/// assert_eq!(format!("{}", error),
///     "The activity 'Activity' cannot be inserted with beginning 10:00.");
/// ```
#[derive(Debug, Clone)]
pub struct InvalidInsertion {
    who: String,
    in_who: Time,
    reason: InvalidOrNotComputed,
}

impl fmt::Display for InvalidInsertion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.reason {
            InvalidOrNotComputed::NotComputed => write!(
                f,
                "{} '{}' {}.",
                tr("The possible beginnings of the activity"),
                self.who,
                tr("have not been computed yet")
            ),
            InvalidOrNotComputed::Invalid => write!(
                f,
                "{} '{}' {} {}.",
                tr("The activity"),
                self.who,
                tr("cannot be inserted with beginning"),
                self.in_who
            ),
        }
    }
}

impl Error for InvalidInsertion {}

impl InvalidInsertion {
    #[must_use]
    pub fn insertion_not_in_computed_insertions<S1>(
        activity_name: S1,
        invalid_insertion_time: Time,
    ) -> Box<InvalidInsertion>
    where
        S1: Into<String>,
    {
        Box::new(InvalidInsertion {
            who: activity_name.into(),
            reason: InvalidOrNotComputed::Invalid,
            in_who: invalid_insertion_time,
        })
    }

    #[must_use]
    pub fn insertions_not_computed_yet<S1>(activity_name: S1) -> Box<InvalidInsertion>
    where
        S1: Into<String>,
    {
        Box::new(InvalidInsertion {
            who: activity_name.into(),
            reason: InvalidOrNotComputed::NotComputed,
            in_who: MIN_TIME_DISCRETIZATION, // We don't care about the time
        })
    }

    // Getters
    #[must_use]
    pub fn who(&self) -> String {
        self.who.clone()
    }

    #[must_use]
    pub fn in_who(&self) -> Time {
        self.in_who
    }

    #[must_use]
    pub fn reason(&self) -> InvalidOrNotComputed {
        self.reason
    }
}
