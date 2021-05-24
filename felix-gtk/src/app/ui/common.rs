use crate::app::ui::Ui;
use felix_data::Data;

use gtk::prelude::*;

impl Ui {
    pub(super) fn update_entity_group_completion_list_store(&self, data: &Data) {
        fetch_from!(self, entity_and_group_completion_list_store);
        entity_and_group_completion_list_store.clear();
        for entity_name in data
            .entities_sorted()
            .into_iter()
            .map(|entity| entity.name())
        {
            entity_and_group_completion_list_store.insert_with_values(
                None,
                &[0, 1],
                &[&entity_name, &"avatar-default-symbolic".to_string()],
            );
        }
        for group_name in data.groups_sorted().into_iter().map(|group| group.name()) {
            entity_and_group_completion_list_store.insert_with_values(
                None,
                &[0, 1],
                &[&group_name, &"system-users-symbolic".to_string()],
            );
        }
    }
}
