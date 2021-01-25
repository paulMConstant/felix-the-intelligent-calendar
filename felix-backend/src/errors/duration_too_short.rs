use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

/// Throw this error when the user sets the duration of an activity to 0.
///
/// # Example
///
/// ```
/// use felix_backend::errors::duration_too_short::DurationTooShort;
///
/// let error = DurationTooShort::new();
///
/// assert_eq!(format!("{}", error), "The given duration is too short.");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct DurationTooShort;

impl fmt::Display for DurationTooShort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.", tr("The given duration is too short"))
    }
}

impl Error for DurationTooShort {}

impl DurationTooShort {
    // Constructors
    #[must_use]
    pub fn new() -> Box<DurationTooShort> {
        Box::new(DurationTooShort {})
    }
}
