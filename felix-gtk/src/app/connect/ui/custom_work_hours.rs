use crate::app::notify::notify_err;
use crate::app::App;

use glib::clone;
use gtk::prelude::*;

use std::convert::TryFrom;
use std::sync::Arc;

use felix_backend::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION};
use felix_backend::errors::invalid_interval::InvalidInterval;
use felix_backend::errors::Result;

macro_rules! reset_custom_work_hours_if_err {
    ($data:ident, $operation:expr) => {
        if let Err(e) = $operation {
            $data
                .events()
                .borrow_mut()
                .emit_custom_work_hours_changed(&$data);
            notify_err(e);
            return;
        }
    };
}

impl App {
    pub fn connect_custom_work_hours(&self) {
        self.init_custom_work_hours_builder();

        self.connect_add_custom_work_hour();
        self.set_custom_work_hour_editing_done_callback();
        self.set_custom_work_hour_remove_callback();
    }

    fn init_custom_work_hours_builder(&self) {
        fetch_from!(self.ui(), custom_work_hours_scrolled_window);

        self.ui
            .lock()
            .unwrap()
            .custom_work_hours_builder()
            .set_work_hours_scrolled_window(custom_work_hours_scrolled_window);
    }

    fn connect_add_custom_work_hour(&self) {
        fetch_from!(self.ui(), custom_work_hours_add_button);

        let ui = &self.ui;
        app_register_signal!(
            self,
            custom_work_hours_add_button,
            custom_work_hours_add_button.connect_clicked(clone!(@strong ui => move |_| {
                let ui = ui.lock().unwrap();
                let current_entity = ui.current_entity()
                    .expect("Current entity should be set before adding custom work hours");
                ui.on_add_custom_work_hour(current_entity.custom_work_hours());
            }))
        );
    }

    fn set_custom_work_hour_editing_done_callback(&self) {
        let data = &self.data;
        let ui = &self.ui;

        let work_hour_editing_done_callback = Arc::new(
            clone!(@strong ui, @strong data => move |position, builder: gtk::Builder| {

                fetch_from_builder!(builder,
                                interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
                                interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
                                interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
                                interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
                               );

                safe_spinbutton_to_i8!(interval_begin_hours => begin_hours,
                                       interval_begin_minutes => begin_minutes,
                                       interval_end_hours => end_hours,
                                       interval_end_minutes => end_minutes);

                let mut data = data.lock().unwrap();

                let beginning = Time::new(begin_hours, begin_minutes);
                let end = Time::new(end_hours, end_minutes);

                if beginning > end || end - beginning < MIN_TIME_DISCRETIZATION {
                    let error: Result<()> = Err(InvalidInterval::new());
                    reset_custom_work_hours_if_err!(data, error);
                }

                let current_entity= ui.lock().unwrap().current_entity()
                    .expect("Current entity should be set before adding custom work hours");
                let work_hours = current_entity.custom_work_hours();
                let interval = TimeInterval::new(beginning, end);

                if position < work_hours.len() {
                    reset_custom_work_hours_if_err!(data,
                        data.update_custom_work_interval_for(current_entity.name(), work_hours[position], interval));
                } else {
                    reset_custom_work_hours_if_err!(data, data.add_custom_work_interval_for(current_entity.name(), interval));
                }
            }),
        );

        self.ui
            .lock()
            .unwrap()
            .custom_work_hours_builder()
            .set_work_interval_editing_done_callback(work_hour_editing_done_callback);
    }

    fn set_custom_work_hour_remove_callback(&self) {
        let ui = &self.ui;
        let data = &self.data;
        let work_hour_editing_done_callback = Arc::new(
            clone!(@strong ui, @strong data => move |position| {
                let mut data = data.lock().unwrap();
                let current_entity = ui.lock().unwrap().current_entity()
                    .expect("Current entity should be set before adding custom work hours");
                let work_hours = current_entity.custom_work_hours();

                if position < work_hours.len() {
                    reset_custom_work_hours_if_err!(data,
                                                    data.remove_custom_work_interval_for(current_entity.name(), work_hours[position]));
                } else {
                    data.events().borrow_mut().emit_custom_work_hours_changed(&data);
                }
            }),
        );

        self.ui
            .lock()
            .unwrap()
            .custom_work_hours_builder()
            .set_work_interval_remove_callback(work_hour_editing_done_callback);
    }
}
