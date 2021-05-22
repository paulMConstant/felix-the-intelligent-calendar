use crate::data::Data;
use crate::errors::{name_taken::NameTaken, Result, };

impl Data {
    /// Checks if the given name is taken by a group.
    ///
    /// # Errors
    ///
    /// Returns Err if the group exists.
    pub(super) fn check_name_taken_by_group(&self, name: &str) -> Result<()> {
        if let Some(group_name) = self
            .groups_sorted()
            .iter()
            .map(|group| group.name())
            .find(|group_name| group_name == name)
        {
            Err(NameTaken::name_taken_by_group(group_name))
        } else {
            Ok(())
        }
    }
}
