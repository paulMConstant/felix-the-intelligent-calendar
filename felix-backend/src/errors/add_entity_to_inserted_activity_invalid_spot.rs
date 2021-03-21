use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum WhySpotIsInvalid {
    BlockingActivity(String),
    OutsideOfWorkHours,
}

/// Throw this error when the user adds an entity to an activity which is inserted
/// and the entity is not free for the activity's insertion slot.
///
/// # Example
///
/// ```
/// use felix_backend::errors::add_entity_to_inserted_activity_invalid_spot::{
///     AddEntityToInsertedActivityInvalidSpot,
///     WhySpotIsInvalid
///};
///
/// let error = AddEntityToInsertedActivityInvalidSpot::blocking_activity("Entity", "Activity", "BlockingActivity");
///
/// assert_eq!(format!("{}", error),
///     "Entity cannot be added to 'Activity' because it would overlap with 'BlockingActivity'.");
/// assert_eq!(error.who(), "Entity");
/// assert_eq!(error.in_what(), "Activity");
/// assert_eq!(error.why(), WhySpotIsInvalid::BlockingActivity("BlockingActivity".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct AddEntityToInsertedActivityInvalidSpot {
    who: String,
    in_what: String,
    why: WhySpotIsInvalid,
}

impl fmt::Display for AddEntityToInsertedActivityInvalidSpot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self.why.clone() {
            WhySpotIsInvalid::BlockingActivity(name) => format!("it would overlap with '{}'", name),

            WhySpotIsInvalid::OutsideOfWorkHours => {
                "it would be outside of their work hours".to_string()
            }
        };

        write!(
            f,
            "{} {} '{}' {} {}.",
            self.who,
            tr("cannot be added to"),
            self.in_what,
            tr("because"),
            reason
        )
    }
}

impl Error for AddEntityToInsertedActivityInvalidSpot {}

impl AddEntityToInsertedActivityInvalidSpot {
    // Constructors
    #[must_use]
    pub fn blocking_activity<S1, S2, S3>(
        entity_name: S1,
        activity_name: S2,
        blocking_activity: S3,
    ) -> Box<AddEntityToInsertedActivityInvalidSpot>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Box::new(AddEntityToInsertedActivityInvalidSpot {
            who: entity_name.into(),
            in_what: activity_name.into(),
            why: WhySpotIsInvalid::BlockingActivity(blocking_activity.into()),
        })
    }

    #[must_use]
    pub fn outside_of_work_hours<S1, S2>(
        entity_name: S1,
        activity_name: S2,
    ) -> Box<AddEntityToInsertedActivityInvalidSpot>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(AddEntityToInsertedActivityInvalidSpot {
            who: entity_name.into(),
            in_what: activity_name.into(),
            why: WhySpotIsInvalid::OutsideOfWorkHours,
        })
    }

    // Getters
    #[must_use]
    pub fn who(&self) -> String {
        self.who.clone()
    }

    #[must_use]
    pub fn in_what(&self) -> String {
        self.in_what.clone()
    }

    #[must_use]
    pub fn why(&self) -> WhySpotIsInvalid {
        self.why.clone()
    }
}
