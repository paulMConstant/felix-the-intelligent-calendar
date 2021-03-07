use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

/// Throw this error when the user adds an entity to an activity which is inserted
/// and the entity is not free for the activity's insertion slot.
///
/// # Example
///
/// ```
/// use felix_backend::errors::add_entity_to_inserted_activity_spot_taken
///     ::AddEntityToInsertedActivitySpotTaken;
///
/// let error = AddEntityToInsertedActivitySpotTaken::new("Entity", "Activity", "BlockingActivity");
///
/// assert_eq!(format!("{}", error),
///     "Entity cannot be added to 'Activity' because it would overlap with 'BlockingActivity'.");
/// assert_eq!(error.who(), "Entity");
/// assert_eq!(error.in_what(), "Activity");
/// assert_eq!(error.blocking_activity(), "BlockingActivity");
/// ```
#[derive(Debug, Clone)]
pub struct AddEntityToInsertedActivitySpotTaken {
    who: String,
    in_what: String,
    blocking_activity: String,
}

impl fmt::Display for AddEntityToInsertedActivitySpotTaken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} '{}' {} '{}'.",
            self.who,
            tr("cannot be added to"),
            self.in_what,
            tr("because it would overlap with"),
            self.blocking_activity
        )
    }
}

impl Error for AddEntityToInsertedActivitySpotTaken {}

impl AddEntityToInsertedActivitySpotTaken {
    // Constructors
    #[must_use]
    pub fn new<S1, S2, S3>(
        entity_name: S1,
        activity_name: S2,
        blocking_activity: S3,
    ) -> Box<AddEntityToInsertedActivitySpotTaken>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Box::new(AddEntityToInsertedActivitySpotTaken {
            who: entity_name.into(),
            in_what: activity_name.into(),
            blocking_activity: blocking_activity.into(),
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
    pub fn blocking_activity(&self) -> String {
        self.blocking_activity.clone()
    }
}
