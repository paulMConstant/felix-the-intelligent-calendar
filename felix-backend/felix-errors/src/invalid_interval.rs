use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

/// Throw this error when the user adds an invalid time interval.
#[derive(Debug, Clone)]
pub struct InvalidInterval {}

impl fmt::Display for InvalidInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.",
            tr("This interval is not valid. The end must be greater than the beginning")
        )
    }
}

impl Error for InvalidInterval {}

impl InvalidInterval {
    #[must_use]
    pub fn new() -> Box<InvalidInterval> {
        Box::new(InvalidInterval {})
    }
}
