use gettextrs::gettext as tr;
use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntityOrGroup {
    Entity,
    Group,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActivityOrGroup {
    Activity,
    Group,
}

/// Throw this error when the user adds a component into a component which already contains it.
///
/// # Example
///
/// ```
/// use felix_backend::errors::already_in::{AlreadyIn, ActivityOrGroup};
///
/// let error = AlreadyIn::entity_already_in_group("Entity Name", "Group Name");
///
/// assert_eq!(format!("{}", error), "Entity Name is already in the group 'Group Name'.");
/// assert_eq!(error.who(), "Entity Name");
/// assert_eq!(error.in_what(), ActivityOrGroup::Group);
/// assert_eq!(error.in_who(), "Group Name");
/// ```
#[derive(Debug, Clone)]
pub struct AlreadyIn {
    what: EntityOrGroup,
    who: String,
    in_what: ActivityOrGroup,
    in_who: String,
}

impl fmt::Display for AlreadyIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let in_what = match self.in_what {
            ActivityOrGroup::Activity => tr("the activity"),
            ActivityOrGroup::Group => tr("the group"),
        };
        let is_in = tr("is already in");
        match self.what {
            EntityOrGroup::Entity => {
                write!(f, "{} {} {} '{}'.", self.who, is_in, in_what, self.in_who)
            }
            EntityOrGroup::Group => {
                let what = tr("The group");
                write!(
                    f,
                    "{} '{}' {} {} '{}'.",
                    what, self.who, is_in, in_what, self.in_who
                )
            }
        }
    }
}

impl Error for AlreadyIn {}

impl AlreadyIn {
    // Constructors
    #[must_use]
    pub fn entity_already_in_group<S1, S2>(entity_name: S1, group_name: S2) -> Box<AlreadyIn>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(AlreadyIn {
            what: EntityOrGroup::Entity,
            who: entity_name.into(),
            in_what: ActivityOrGroup::Group,
            in_who: group_name.into(),
        })
    }

    #[must_use]
    pub fn entity_already_in_activity<S1, S2>(entity_name: S1, activity_name: S2) -> Box<AlreadyIn>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(AlreadyIn {
            what: EntityOrGroup::Entity,
            who: entity_name.into(),
            in_what: ActivityOrGroup::Activity,
            in_who: activity_name.into(),
        })
    }

    #[must_use]
    pub fn group_already_in_activity<S1, S2>(group_name: S1, activity_name: S2) -> Box<AlreadyIn>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Box::new(AlreadyIn {
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
