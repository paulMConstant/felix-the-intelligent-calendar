use std::error::Error;
use std::fmt;
use gettextrs::gettext as tr;

use crate::data::TimeInterval;

/// Defines the component type which does not exist.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ComponentType {
    Entity,
    TimeInterval,
    Group,
    Activity,
}

/// Throw this error when the user asked for a component which does not exist.
/// 
/// The error is built from a constructor function in the form 'xxx\_does\_not\_exist'.
///
/// # Example
///
/// ```
/// use plan_backend::errors::does_not_exist::{DoesNotExist, ComponentType};
///
/// let error = DoesNotExist::entity_does_not_exist("Entity Name");
///
/// assert_eq!(format!("{}", error), "The entity 'Entity Name' does not exist.");
/// assert_eq!(error.what(), ComponentType::Entity);
/// assert_eq!(error.who(), "Entity Name");
/// ```
#[derive(Debug, Clone)]
pub struct DoesNotExist {
    what: ComponentType,
    who: String,
}

impl fmt::Display for DoesNotExist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let what = match self.what {
            ComponentType::Entity => tr("The entity"),
            ComponentType::TimeInterval => tr("The interval"),
            ComponentType::Group => tr("The group"),
            ComponentType::Activity => tr("The activity with id"),
        };
        write!(f, "{} '{}' {}.", what, self.who, tr("does not exist"))
    }
}

impl Error for DoesNotExist {}

impl DoesNotExist {
    // Constructors
    #[must_use]
    pub fn interval_does_not_exist(interval: TimeInterval) -> DoesNotExist {
        DoesNotExist { what: ComponentType::TimeInterval, who: interval.to_string() }
    }

    #[must_use]
    pub fn entity_does_not_exist<S>(name: S) -> DoesNotExist where S: Into<String>, {
        DoesNotExist { what: ComponentType::Entity, who: name.into() }
    }

    #[must_use]
    pub fn group_does_not_exist<S>(name: S) -> DoesNotExist where S: Into<String>, {
        DoesNotExist { what: ComponentType::Group, who: name.into() }
    }
    
    #[must_use]
    pub fn activity_does_not_exist(id: u16) -> DoesNotExist {
        DoesNotExist { what: ComponentType::Activity, who: id.to_string() }
    }

    // Getters
    #[must_use]
    pub fn what(&self) -> ComponentType {
        self.what
    }

    #[must_use]
    pub fn who(&self) -> String {
        self.who.clone()
    }
}
