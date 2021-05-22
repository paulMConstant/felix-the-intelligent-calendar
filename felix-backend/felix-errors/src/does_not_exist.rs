use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

use felix_datatypes::TimeInterval;

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
#[derive(Debug, Clone)]
pub struct DoesNotExist {
    what: ComponentType,
    who: String,
}

impl fmt::Display for DoesNotExist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let does_not_exist = tr("does not exist");
        if self.what == ComponentType::Entity {
            write!(f, "{} {}.", self.who, does_not_exist)
        } else {
            let what = match self.what {
                ComponentType::TimeInterval => tr("The interval"),
                ComponentType::Group => tr("The group"),
                ComponentType::Activity => tr("The activity with id"),
                ComponentType::Entity => panic!("This case should have been treated above"),
            };
            write!(f, "{} '{}' {}.", what, self.who, does_not_exist)
        }
    }
}

impl Error for DoesNotExist {}

impl DoesNotExist {
    // Constructors
    #[must_use]
    pub fn interval_does_not_exist(interval: TimeInterval) -> Box<DoesNotExist> {
        Box::new(DoesNotExist {
            what: ComponentType::TimeInterval,
            who: interval.to_string(),
        })
    }

    #[must_use]
    pub fn entity_does_not_exist<S>(name: S) -> Box<DoesNotExist>
    where
        S: Into<String>,
    {
        Box::new(DoesNotExist {
            what: ComponentType::Entity,
            who: name.into(),
        })
    }

    #[must_use]
    pub fn group_does_not_exist<S>(name: S) -> Box<DoesNotExist>
    where
        S: Into<String>,
    {
        Box::new(DoesNotExist {
            what: ComponentType::Group,
            who: name.into(),
        })
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
