use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

/// Throw this error when the user tries to modify work hours
/// while at least one activity is inserted.
#[derive(Debug, Clone)]
pub struct ChangeWorkHoursWhileActivityInserted {}

impl fmt::Display for ChangeWorkHoursWhileActivityInserted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.",
            tr("Work hours cannot be modified while an activity is inserted")
        )
    }
}

impl Error for ChangeWorkHoursWhileActivityInserted {}

impl ChangeWorkHoursWhileActivityInserted {
    #[must_use]
    pub fn new() -> Box<ChangeWorkHoursWhileActivityInserted> {
        Box::new(ChangeWorkHoursWhileActivityInserted {})
    }
}
