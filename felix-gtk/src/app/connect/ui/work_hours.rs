use crate::app::notify::notify_err;
use crate::app::App;

use gtk::prelude::*;

use std::convert::TryFrom;
use std::sync::Arc;

use felix_backend::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION};
use felix_backend::errors::invalid_interval::InvalidInterval;
use felix_backend::errors::Result;

macro_rules! reset_work_hours_if_err {
    ($data:ident, $operation:expr) => {
        if let Err(e) = $operation {
            $data.events().borrow_mut().emit_work_hours_changed(&$data);
            notify_err(e);
            return;
        }
    };
}

impl App {
    pub fn connect_work_hours_tab(&self) {
        self.init_work_hours_builder();
        self.connect_add_work_hour();
        self.set_work_hour_editing_done_callback();
        self.set_work_hour_remove_callback();
    }

    fn init_work_hours_builder(&self) {
        fetch_from!(self.ui(), work_hours_scrolled_window);

        self.ui
            .lock()
            .unwrap()
            .work_hours_builder()
            .set_work_hours_scrolled_window(work_hours_scrolled_window);
    }

    fn connect_add_work_hour(&self) {
        fetch_from!(self.ui(), work_hour_add_button);

        let ui = self.ui.clone();
        let data = self.data.clone();
        app_register_signal!(
            self,
            work_hour_add_button,
            work_hour_add_button.connect_clicked(move |_| {
                let current_work_hours = data.lock().unwrap().work_hours();
                ui.lock().unwrap().on_add_work_hour(current_work_hours);
            })
        );
    }

    fn set_work_hour_editing_done_callback(&self) {
        let data = self.data.clone();
        let work_hour_editing_done_callback = Arc::new(move |position, builder: gtk::Builder| {
            let mut data = data.lock().unwrap();
            let work_hours = data.work_hours();

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

            let beginning = Time::new(begin_hours, begin_minutes);
            let end = Time::new(end_hours, end_minutes);
            if beginning > end || end - beginning < MIN_TIME_DISCRETIZATION {
                let error: Result<()> = Err(InvalidInterval::new());
                reset_work_hours_if_err!(data, error);
            }

            let interval = TimeInterval::new(beginning, end);

            if position < work_hours.len() {
                reset_work_hours_if_err!(
                    data,
                    data.update_work_interval(work_hours[position], interval)
                );
            } else {
                reset_work_hours_if_err!(data, data.add_work_interval(interval));
            }
        });

        self.ui
            .lock()
            .unwrap()
            .work_hours_builder()
            .set_work_interval_editing_done_callback(work_hour_editing_done_callback);
    }

    fn set_work_hour_remove_callback(&self) {
        let data = self.data.clone();
        let work_hour_editing_done_callback = Arc::new(move |position| {
            let mut data = data.lock().unwrap();
            let work_hours = data.work_hours();

            if position < work_hours.len() {
                reset_work_hours_if_err!(data, data.remove_work_interval(work_hours[position]));
            } else {
                data.events().borrow_mut().emit_work_hours_changed(&data);
            }
        });

        self.ui
            .lock()
            .unwrap()
            .work_hours_builder()
            .set_work_interval_remove_callback(work_hour_editing_done_callback);
    }
}
