use crate::app::App;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_groups_tab(&self) {
        self.connect_add_group();
        self.connect_group_selected();
        self.connect_add_entity_to_group();
        self.connect_remove_group();
        self.connect_rename_group();
        self.connect_remove_entity_from_group();

        self.connect_clean_add_entity_to_group_entry();
        self.connect_clean_add_group_entry();
        self.connect_clean_group_name_entry();
    }

    fn connect_add_group(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            group_add_button,
            group_add_entry
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            group_add_button,
            group_add_button.connect_clicked(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_group();
            }))
        );

        app_register_signal!(
            self,
            group_add_entry,
            group_add_entry.connect_activate(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_group();
            }))
        );
    }

    fn connect_group_selected(&self) {
        fetch_from!(self.app_data.lock().unwrap(), groups_tree_view);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            groups_tree_view,
            groups_tree_view.connect_cursor_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_group_selected();
            }),)
        );
    }

    fn connect_remove_group(&self) {
        fetch_from!(self.app_data.lock().unwrap(), group_remove_button);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            group_remove_button,
            group_remove_button.connect_clicked(clone!(@strong app_data => move |_| {
               app_data.lock().unwrap().event_remove_group();
            }))
        );
    }

    fn connect_rename_group(&self) {
        fetch_from!(self.app_data.lock().unwrap(), group_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            group_name_entry,
            group_name_entry.connect_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_rename_group();
            }))
        );
    }

    fn connect_add_entity_to_group(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            entity_into_group_name_entry,
            add_to_group_button
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_into_group_name_entry,
            entity_into_group_name_entry.connect_activate(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_entity_to_group();
            }))
        );
        app_register_signal!(
            self,
            add_to_group_button,
            add_to_group_button.connect_clicked(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_entity_to_group();
            }))
        );
    }

    fn connect_remove_entity_from_group(&self) {
        fetch_from!(self.app_data.lock().unwrap(), group_members_tree_view);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            group_members_tree_view,
            group_members_tree_view.connect_row_activated(clone!(@strong app_data => move |_self, treepath, treeview_column| {
                app_data.lock().unwrap().event_remove_entity_from_group(treepath, treeview_column);
            })));
    }

    fn connect_clean_group_name_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), group_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            group_name_entry,
            group_name_entry.connect_changed(
                clone!(@strong app_data, @weak group_name_entry => move |_| {
                            app_data.lock().unwrap().event_clean_entry_content(group_name_entry);
                })
            )
        );
    }

    fn connect_clean_add_group_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), group_add_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            group_add_entry,
            group_add_entry.connect_changed(
                clone!(@strong app_data, @weak group_add_entry => move |_| {
                    app_data.lock().unwrap().event_clean_entry_content(group_add_entry);
                })
            )
        );
    }

    fn connect_clean_add_entity_to_group_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_into_group_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_into_group_name_entry,
            entity_into_group_name_entry.connect_changed(clone!(@strong app_data, @weak entity_into_group_name_entry => move |_| {
                app_data.lock().unwrap().event_clean_entry_content(entity_into_group_name_entry);
            })));
    }
}
