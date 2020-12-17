use glib::clone;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_activities_tab(&self) {
        self.connect_add_activity();
        self.connect_activity_selected();
        self.connect_remove_activity();
        self.connect_rename_activity();
        self.connect_set_activity_duration();
        self.connect_add_to_activity();

        self.connect_clean_add_activity_entry();
        self.connect_clean_activity_name_entry();
        self.connect_clean_activity_add_to_entry();
    }

    fn connect_add_activity(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            activity_add_button,
            activity_add_entry
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_add_button,
            activity_add_button.connect_clicked(clone!(@strong app_data =>
                                                                    move |_| {
            app_data.lock().unwrap().event_add_activity()
                                                                    }))
        );

        app_register_signal!(
            self,
            activity_add_entry,
            activity_add_entry.connect_activate(clone!(@strong app_data => move |_| {


            app_data.lock().unwrap().event_add_activity()
                             }))
        );
    }

    fn connect_activity_selected(&self) {
        fetch_from!(self.app_data.lock().unwrap(), activities_tree_view);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activities_tree_view,
            activities_tree_view.connect_cursor_changed(clone!(@strong app_data => move |_| {
               app_data.lock().unwrap().event_activity_selected();
            }),)
        );
    }

    fn connect_remove_activity(&self) {
        fetch_from!(self.app_data.lock().unwrap(), activity_remove_button);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_remove_button,
            activity_remove_button.connect_clicked(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_remove_activity();
            }))
        );
    }

    fn connect_rename_activity(&self) {
        fetch_from!(self.app_data.lock().unwrap(), activity_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_name_entry,
            activity_name_entry.connect_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_rename_activity();
            }))
        );
    }

    fn connect_clean_add_activity_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), activity_add_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_add_entry,
            activity_add_entry.connect_changed(
                clone!(@strong app_data, @weak activity_add_entry => move |_| {

                app_data.lock().unwrap().event_clean_entry_content(activity_add_entry);
                                         })
            )
        );
    }

    fn connect_clean_activity_name_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), activity_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_name_entry,
            activity_name_entry.connect_changed(
                clone!(@strong app_data, @weak activity_name_entry => move |_| {

                app_data.lock().unwrap().event_clean_entry_content(activity_name_entry);
                                         })
            )
        );
    }

    fn connect_clean_activity_add_to_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), activity_add_to_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_add_to_entry,
            activity_add_to_entry.connect_changed(
                clone!(@strong app_data, @weak activity_add_to_entry => move |_| {
                    app_data.lock().unwrap().event_clean_entry_content(activity_add_to_entry);
                })
            )
        );
    }

    fn connect_set_activity_duration(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            activity_duration_minute_spin,
            activity_duration_hour_spin
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_duration_minute_spin,
            activity_duration_minute_spin.connect_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_set_activity_duration();
            }))
        );

        app_register_signal!(
            self,
            activity_duration_hour_spin,
            activity_duration_hour_spin.connect_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_set_activity_duration();
            }))
        );
    }

    fn connect_add_to_activity(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            activity_add_to_entry,
            activity_add_to_button
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            activity_add_to_entry,
            activity_add_to_entry.connect_activate(clone!(@strong app_data => move |_| {
            app_data.lock().unwrap().event_add_to_activity();
                                 }))
        );
        app_register_signal!(
            self,
            activity_add_to_button,
            activity_add_to_button.connect_clicked(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_to_activity();
            }))
        );
    }
}
