use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

/// Defines the component type which is not found.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntityOrGroup {
    Group,
    Entity,
}

/// Defines the container in which the component is not found.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActivityOrGroup {
    Group,
    Activity,
}

/// Throw this error when the user asks for a component which is not present in the activity/group.
///
/// # Example
///
/// ```
/// use felix_backend::errors::not_in::{NotIn, EntityOrGroup, ActivityOrGroup};
///
/// let error = NotIn::entity_not_in_group("Entity Name", "Group Name");
///
/// assert_eq!(format!("{}", error), "Entity Name is not in the group 'Group Name'.");
/// assert_eq!(error.what(), EntityOrGroup::Entity);
/// assert_eq!(error.who(), "Entity Name");
/// assert_eq!(error.in_what(), ActivityOrGroup::Group);
/// assert_eq!(error.in_who(), "Group Name");
/// ```
#[derive(Debug, Clone)]
pub struct NotIn {
    what: EntityOrGroup,
    who: String,
    in_what: ActivityOrGroup,
    in_who: String,
}

impl fmt::Display for NotIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let in_what = match self.in_what {
            ActivityOrGroup::Activity => tr("the activity"),
            ActivityOrGroup::Group => tr("the group"),
        };
        let is_not_in = tr("is not in");
        match self.what {
            EntityOrGroup::Entity => write!(
                f,
                "{} {} {} '{}'.",
                self.who, is_not_in, in_what, self.in_who
            ),
            EntityOrGroup::Group => {
                let what = tr("The group");
                write!(
                    f,
                    "{} '{}' {} {} '{}'.",
                    what,
                    self.who,
                    tr("is not in"),
                    in_what,
                    self.in_who
                )
            }
        }
    }
}

impl Error for NotIn {}

impl NotIn {
    // Constructors
    #[must_use]
    pub fn entity_not_in_group<S1, S2>(entity_name: S1, group_name: S2) -> Box<NotIn>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(NotIn {
            what: EntityOrGroup::Entity,
            who: entity_name.into(),
            in_what: ActivityOrGroup::Group,
            in_who: group_name.into(),
        })
    }

    #[must_use]
    pub fn entity_not_in_activity<S1, S2>(entity_name: S1, activity_name: S2) -> Box<NotIn>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(NotIn {
            what: EntityOrGroup::Entity,
            who: entity_name.into(),
            in_what: ActivityOrGroup::Activity,
            in_who: activity_name.into(),
        })
    }

    #[must_use]
    pub fn group_not_in_activity<S1, S2>(group_name: S1, activity_name: S2) -> Box<NotIn>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(NotIn {
            what: EntityOrGroup::Group,
            who: group_name.into(),
            in_what: ActivityOrGroup::Activity,
            in_who: activity_name.into(),
        })
    }

    // Getters
    #[must_use]
    pub fn what(&self) -> EntityOrGroup {
        self.what
    }

    #[must_use]
    pub fn who(&self) -> String {
        self.who.clone()
    }
    #[must_use]
    pub fn in_what(&self) -> ActivityOrGroup {
        self.in_what
    }
    #[must_use]
    pub fn in_who(&self) -> String {
        self.in_who.clone()
    }
}
