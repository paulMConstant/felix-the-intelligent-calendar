use crate::data::{ActivityID, Data};

impl Data {
    pub(in super::super::groups) fn ids_of_activities_in_which_entity_is_participating_only_through_this_group(
        &self,
        entity_name: &String,
        group_name: &String,
    ) -> Vec<ActivityID> {
        let other_groups_of_entity: Vec<String> = self
            .groups_sorted()
            .into_iter()
            .filter_map(|group| {
                if &group.name() != group_name && group.entities_sorted().contains(entity_name) {
                    Some(group.name())
                } else {
                    None
                }
            })
            .collect();

        self.activities_sorted()
            .iter()
            .filter_map(|activity| {
                let entities = activity.entities_sorted();
                let groups = activity.groups_sorted();
                if entities.contains(entity_name)
                    && groups.contains(group_name)
                    && (groups
                        .into_iter()
                        .any(|group_name| other_groups_of_entity.contains(&group_name))
                        == false)
                {
                    Some(activity.id())
                } else {
                    None
                }
            })
            .collect()
    }
}
