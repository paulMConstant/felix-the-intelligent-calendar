use crate::app::ui::Ui;
use plan_backend::data::{Activity, Group};

use gtk::prelude::*;

// Sorted map
use std::collections::BTreeMap;

type EntityName = String;
type EntityNotInActivityButInGroup = Option<String>;
type EntityInActivity = BTreeMap<EntityName, EntityNotInActivityButInGroup>;

impl Ui {
    pub(super) fn update_current_activity_entities(&self, groups: &Vec<&Group>) {
        fetch_from!(
            self,
            activity_entities_list_store,
            activity_entities_tree_view
        );

        if let Some(activity) = &self.current_activity {
            let entities = create_entity_list(activity, groups);

            with_blocked_signals!(
                self,
                {
                    activity_entities_list_store.clear();
                    // Iteration over a BTreeMap is sorted by key.
                    for (entity_name, not_in_activity_but_in_group) in entities.iter() {
                        let (icon, strikethrough, color) = match not_in_activity_but_in_group {
                            Some(_group) => ("list-add-symbolic", true, "grey"), // TODO add group as tooltip
                            None => ("user-trash-symbolic", false, "black"),
                        };
                        activity_entities_list_store.insert_with_values(
                            None,
                            &[0, 1, 2, 3],
                            &[&entity_name, &icon, &strikethrough, &color],
                        );
                    }
                },
                activity_entities_tree_view
            );
        }
    }
}

/// Creates the list of entities which should be added to the list store.
/// If any entity is present in an activity's group but not in the activity,
/// the group in which they are present is returned as well.
fn create_entity_list(activity: &Activity, groups: &Vec<&Group>) -> EntityInActivity {
    let activity_entities = activity.entities_sorted();
    let mut entities: EntityInActivity = EntityInActivity::new();

    let activity_group_names = activity.groups_sorted();
    let activity_groups = groups
        .iter()
        .filter(|group| activity_group_names.contains(&group.name()));

    for group in activity_groups {
        for entity in group.entities_sorted() {
            if entities.contains_key(&entity) {
                continue;
            }

            let group: EntityNotInActivityButInGroup = if activity_entities.contains(&entity) {
                None
            } else {
                // The entity is in one of the activity's groups but not in the activity.
                let group: EntityNotInActivityButInGroup = Some(group.name());
                group
            };
            entities.insert(entity, group);
        }
    }

    for entity in activity_entities {
        if entities.contains_key(&entity) == false {
            entities.insert(entity, None);
        }
    }
    entities
}
