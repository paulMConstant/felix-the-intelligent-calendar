use crate::app::ui::helpers::tree::tree_path_from_selection_index;
use crate::app::ui::Ui;

use gtk::prelude::*;

use felix_data::{Data, Entity};

impl Ui {
    pub(super) fn update_current_entity_without_ui(&mut self, entity: Option<Entity>) {
        self.current_entity = entity;
    }

    pub(super) fn update_current_entity(&mut self, entity: Option<Entity>, data: &Data) {
        self.update_current_entity_without_ui(entity);

        if self.current_entity.is_some() {
            self.update_current_entity_view(data);
        } else {
            self.hide_current_entity_view();
        };
    }

    pub(super) fn hide_current_entity_view(&self) {
        fetch_from!(self, entity_specific_box);
        entity_specific_box.hide();
    }

    fn update_current_entity_view(&self, data: &Data) {
        fetch_from!(
            self,
            entity_specific_box,
            entity_name_entry,
            entity_send_mail_switch,
            entity_mail_entry
        );

        let current_entity = self
            .current_entity
            .as_ref()
            .expect("Current entity should be Some when the view is updated");

        entity_specific_box.show();

        with_blocked_signals!(
            self,
            {
                entity_name_entry.set_text(&current_entity.name());
                entity_mail_entry.set_text(&current_entity.mail());
                entity_send_mail_switch.set_active(current_entity.send_me_a_mail());
            },
            entity_name_entry,
            entity_mail_entry,
            entity_send_mail_switch
        );

        self.custom_work_hours_builder.on_work_hours_changed(
            data.custom_work_hours_of(current_entity.name())
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not fetch custom work hours of {}",
                        current_entity.name()
                    )
                }),
        );
    }

    /// Updates the treeview of entities and selects the given row if not None.
    /// If the given row is None, keeps the originally selected row.
    pub(super) fn update_entities_treeview(&self, entities: &[&Entity]) {
        self.update_entities_list_store(entities);
        self.update_entities_treeview_selection();
    }

    fn update_entities_list_store(&self, entities: &[&Entity]) {
        fetch_from!(self, entities_list_store, entities_tree_view);

        with_blocked_signals!(
            self,
            {
                entities_list_store.clear();
                for entity_name in entities.iter().map(|entity| entity.name()) {
                    entities_list_store.insert_with_values(None, &[0], &[&entity_name]);
                }
            },
            entities_tree_view
        );
    }

    /// Selects the row with given index.
    fn update_entities_treeview_selection(&self) {
        fetch_from!(self, entities_tree_view, entities_list_store);

        let current_entity = self.current_entity.as_ref();
        if let Some(entity) = current_entity {
            let selection_tree_path =
                tree_path_from_selection_index(None, entities_list_store, Some(entity.name()));
            let focus_column = None::<&gtk::TreeViewColumn>;
            with_blocked_signals!(
                self,
                entities_tree_view.set_cursor(&selection_tree_path, focus_column, false),
                entities_tree_view
            );
        }
    }
}
