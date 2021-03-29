use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

use crate::data::{Time, MIN_TIME_DISCRETIZATION};

type ActivityName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyInvalid {
    OverlappingWithOtherInsertedActivity(ActivityName),
    CannotFitOrWouldBlockOtherActivities,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidOrNotComputed {
    Invalid(WhyInvalid),
    NotComputed,
}

/// Throw this error when the user tries to insert an activity where it cannot be inserted,
/// or when the possible insertion times are queried but not availabe.
///
/// # Example
///
/// ```
/// use felix_backend::errors::invalid_insertion::InvalidInsertion;
/// use felix_backend::data::Time;
///
/// let error = InvalidInsertion::would_overlap_with_activity("Activity", Time::new(10, 0),
///     "Other Activity");
///
/// assert_eq!(format!("{}", error),
///     "Activity cannot be inserted with beginning 10:00 because it would overlap with 'Other Activity'.");
/// ```
#[derive(Debug, Clone)]
pub struct InvalidInsertion {
    who: String,
    in_who: Time,
    reason: InvalidOrNotComputed,
}

impl fmt::Display for InvalidInsertion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.reason {
            InvalidOrNotComputed::NotComputed => write!(
                f,
                "{} '{}' {}.",
                tr("The possible beginnings of the activity"),
                self.who,
                tr("have not been computed yet")
            ),
            InvalidOrNotComputed::Invalid(why) => {
                let reason = match why {
                    WhyInvalid::CannotFitOrWouldBlockOtherActivities => {
                        tr("this beginning is invalid or will cause problems in the future")
                    }
                    WhyInvalid::OverlappingWithOtherInsertedActivity(activity) => {
                        format!("{} '{}'", tr("it would overlap with"), activity)
                    }
                };

                write!(
                    f,
                    "{} {} {} {} {}.",
                    self.who,
                    tr("cannot be inserted with beginning"),
                    self.in_who,
                    tr("because"),
                    reason
                )
            }
        }
    }
}

impl Error for InvalidInsertion {}

impl InvalidInsertion {
    #[must_use]
    pub fn cannot_fit_or_would_block_other_activities<S>(
        activity_name: S,
        invalid_insertion_time: Time,
    ) -> Box<InvalidInsertion>
    where
        S: Into<String>,
    {
        Self::insertion_not_in_computed_insertions(
            activity_name,
            invalid_insertion_time,
            WhyInvalid::CannotFitOrWouldBlockOtherActivities,
        )
    }

    #[must_use]
    pub fn would_overlap_with_activity<S1, S2>(
        activity_name: S1,
        invalid_insertion_time: Time,
        blocking_activity: S2,
    ) -> Box<InvalidInsertion>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::insertion_not_in_computed_insertions(
            activity_name,
            invalid_insertion_time,
            WhyInvalid::OverlappingWithOtherInsertedActivity(blocking_activity.into()),
        )
    }

    #[must_use]
    fn insertion_not_in_computed_insertions<S>(
        activity_name: S,
        invalid_insertion_time: Time,
        reason: WhyInvalid,
    ) -> Box<InvalidInsertion>
    where
        S: Into<String>,
    {
        Box::new(InvalidInsertion {
            who: activity_name.into(),
            reason: InvalidOrNotComputed::Invalid(reason),
            in_who: invalid_insertion_time,
        })
    }

    #[must_use]
    pub fn insertions_not_computed_yet<S>(activity_name: S) -> Box<InvalidInsertion>
    where
        S: Into<String>,
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
        self.reason.clone()
    }
}
