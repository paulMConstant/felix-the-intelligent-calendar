use std::error::Error;
use std::fmt;
use gettextrs::gettext as tr;

/// Throw this error when the user creates a time interval which overlaps with others.
/// 
///
/// # Example
///
/// ```
/// use plan_backend::errors::interval_overlaps::IntervalOverlaps;
///
/// let error = IntervalOverlaps::new();
///
/// assert_eq!(format!("{}", error), "The given interval overlaps with others.");
/// ```
#[derive(Debug, Clone)]
pub struct IntervalOverlaps { }

impl fmt::Display for IntervalOverlaps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.", tr("The given interval overlaps with others"))
    }
}

impl Error for IntervalOverlaps {}

impl IntervalOverlaps {
    // Constructors
    #[must_use]
    pub fn new() -> IntervalOverlaps {
        IntervalOverlaps { }
    }
}

