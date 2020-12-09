use std::error::Error;
use std::fmt;
use gettextrs::gettext as tr;

/// Throw this error when a given name is empty.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EmptyName;

impl Error for EmptyName {}

impl fmt::Display for EmptyName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.", tr("The given name is empty"))
    }
}

impl EmptyName {
    pub fn new() -> Box<EmptyName> {
        Box::new(EmptyName { } )
    }
}
