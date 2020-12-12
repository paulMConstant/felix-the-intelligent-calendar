use crate::app::appdata::AppData;
use plan_backend::data::Entity;

use gtk::prelude::*;

impl AppData {
    /// Updates the state of AppData and Entity-specific UI.
    pub(super) fn update_current_entity(&mut self, entity: &Option<Entity>) {
        match entity {
            Some(entity) => {
                self.update_current_entity_view(entity);
                self.state.current_entity = Some(entity.name());
            }
            None => {
                self.hide_current_entity_view();
                self.state.current_entity = None;
            }
        };
    }

    /// Updates the treeview of entities and selects the given row if not None.
    /// If the given row is None, keeps the original row.
    pub(super) fn update_entities_treeview(&self, selection_row: Option<i32>) {
        self.update_entities_list_store();
        self.update_treeview_selection(selection_row);
    }

    fn update_entities_list_store(&self) {
        fetch_from!(self, entities_list_store, entities_tree_view);

        with_blocked_signals!(
            self,
            {
                entities_list_store.clear();
                for entity_name in self
                    .data
                    .entities_sorted()
                    .into_iter()
                    .map(|entity| entity.name())
                {
                    entities_list_store.insert_with_values(None, &[0], &[&entity_name]);
                }
            },
            entities_tree_view
        );
    }

    /// Selects the row with given index.
    fn update_treeview_selection(&self, selection_index: Option<i32>) {
        // Select the corresponding entity in tree view
        fetch_from!(self, entities_tree_view);

        let selection_index = selection_index.or_else(|| {
            if let Some(current_entity_name) = self.state.current_entity.as_ref() {
                self.index_of_row_containing(&current_entity_name)
            } else {
                None
            }
        });

        let selection_tree_path = match selection_index {
            Some(index) => gtk::TreePath::from_indicesv(&[index]),
            None => gtk::TreePath::new(),
        };
        let focus_column = None::<&gtk::TreeViewColumn>;
        with_blocked_signals!(
            self,
            entities_tree_view.set_cursor(&selection_tree_path, focus_column, false),
            entities_tree_view
        );
    }

    fn index_of_row_containing(&self, text: &String) -> Option<i32> {
        fetch_from!(self, entities_list_store);
        let iter = entities_list_store.get_iter_first();
        let mut index = 0;
        if let Some(iter) = iter {
            loop {
                let text_entities_list_store = entities_list_store
                    .get_value(&iter, 0)
                    .get::<String>()
                    .expect("Iter should be valid; if it is not, we break out of the loop")
                    .expect("Value should be of type gchararray, no problem to convert to string");
                if text_entities_list_store == *text {
                    return Some(index);
                }
                if entities_list_store.iter_next(&iter) == false {
                    return None;
                }
                index += 1;
            }
        };
        // We should never reach this point. This is here for the compiler.
        return None;
    }

    fn update_current_entity_view(&mut self, entity: &Entity) {
        fetch_from!(
            self,
            entity_specific_box,
            entity_name_entry,
            entity_send_mail_switch,
            entity_mail_entry,
            entity_custom_work_hours_switch
        );

        entity_specific_box.show();

        with_blocked_signals!(
            self,
            {
                entity_name_entry.set_text(&entity.name());
                entity_mail_entry.set_text(&entity.mail());
                entity_custom_work_hours_switch
                    .set_active(entity.custom_work_hours().is_empty() == false);
                entity_send_mail_switch.set_active(entity.send_me_a_mail());
            },
            entity_name_entry,
            entity_mail_entry,
            entity_custom_work_hours_switch,
            entity_send_mail_switch
        );
    }

    fn hide_current_entity_view(&mut self) {
        fetch_from!(self, entity_specific_box);
        entity_specific_box.hide();
    }
}
