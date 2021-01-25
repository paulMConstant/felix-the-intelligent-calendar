use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

/// Defines the component type by which the name is taken.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GroupOrEntity {
    Entity,
    Group,
}

/// Throw this error when the new name of a component given by a user is already taken.
///
/// The error is built from functions in the form 'name\_taken\_by\_xxx' where xxx is either a name
/// or a group.
///
/// # Example
///
/// ```
/// use felix_backend::errors::name_taken::{NameTaken, GroupOrEntity};
///
/// let error = NameTaken::name_taken_by_entity("New Name");
///
/// assert_eq!(format!("{}", error), "The name 'New Name' is already taken by an entity.");
/// assert_eq!(error.by(), GroupOrEntity::Entity);
/// assert_eq!(error.name(), "New Name");
/// ```
#[derive(Debug, Clone)]
pub struct NameTaken {
    by: GroupOrEntity,
    name: String,
}

impl fmt::Display for NameTaken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let by = match self.by {
            GroupOrEntity::Entity => tr("an entity"),
            GroupOrEntity::Group => tr("a group"),
        };
        write!(
            f,
            "{} '{}' {} {}.",
            tr("The name"),
            self.name,
            tr("is already taken by"),
            by
        )
    }
}

impl Error for NameTaken {}

impl NameTaken {
    // Constructors
    pub fn name_taken_by_entity<S>(name: S) -> Box<NameTaken>
    where
        S: Into<String>,
    {
        Box::new(NameTaken {
            by: GroupOrEntity::Entity,
            name: name.into(),
        })
    }

    pub fn name_taken_by_group<S>(name: S) -> Box<NameTaken>
    where
        S: Into<String>,
    {
        Box::new(NameTaken {
            by: GroupOrEntity::Group,
            name: name.into(),
        })
    }

    // Getters
    pub fn by(&self) -> GroupOrEntity {
        self.by
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
