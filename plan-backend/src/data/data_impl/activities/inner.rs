//! Helper functions for activity implementation of data.

use crate::data::Data;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Data {
    /// Returns all entities which participate in the given activity in more than one group.
    ///
    /// # Errors
    ///
    /// Returns Err if the given activity id is not valid or the group does not exist.
    #[must_use]
    pub(in super::super::activities) fn entities_participating_through_this_group_only(
        &self,
        activity_id: u16,
        group_name: &String,
    ) -> Result<HashSet<String>, String> {
        let all_participating_groups = self.activity(activity_id)?.groups_sorted();
        let entities_of_group =
            HashSet::from_iter(self.group(group_name)?.entities_sorted().into_iter());

        let entities_in_other_groups = all_participating_groups
            .iter()
            .filter(|&other_group_name| other_group_name != group_name)
            .flat_map(|group_name|
                // Expect is safe to use here: we are sure that the activtiy contains valid groups
                self.group(group_name).expect("Could not get group which is in an activity").entities_sorted()
                )
            .collect::<HashSet<&String>>();

        Ok(entities_of_group
            .difference(&entities_in_other_groups)
            .map(|&entity| entity.clone())
            .collect())
    }
}
