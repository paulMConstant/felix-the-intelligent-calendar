use crate::app::App;

use gtk::prelude::*;

use std::convert::TryFrom;
use std::rc::Rc;

use felix_backend::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION};
use felix_backend::errors::invalid_interval::InvalidInterval;
use felix_backend::errors::Result;

macro_rules! reset_custom_work_hours_if_err {
    ($ui:ident, $data:ident, $operation:expr) => {
        if let Err(e) = $operation {
            $data.events().borrow_mut().emit_work_hours_changed(&$data);
            $ui.borrow().notify_err(e);
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
        fetch_from!(self.ui.borrow(), custom_work_hours_scrolled_window);

        self.ui
            .borrow_mut()
            .custom_work_hours_builder()
            .set_work_hours_scrolled_window(custom_work_hours_scrolled_window);
    }

    fn connect_add_custom_work_hour(&self) {
        fetch_from!(self.ui.borrow(), custom_work_hours_add_button);

        let ui = self.ui.clone();
        let data = self.data.clone();
        app_register_signal!(
            self,
            custom_work_hours_add_button,
            custom_work_hours_add_button.connect_clicked(move |_| {
                let current_entity = ui
                    .borrow()
                    .current_entity()
                    .expect("Current entity should be set before adding custom work hours");
                ui.borrow_mut().on_add_custom_work_hour(
                    data.borrow()
                        .custom_work_hours_of(current_entity.name())
                        .unwrap_or_else(|_| {
                            panic!(
                                "Could not get custom work hours of {}",
                                current_entity.name()
                            )
                        }),
                );
            })
        );
    }

    fn set_custom_work_hour_editing_done_callback(&self) {
        let data = self.data.clone();
        let ui = self.ui.clone();

        let work_hour_editing_done_callback = Rc::new(move |position, builder: gtk::Builder| {
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

            let mut data = data.borrow_mut();

            let beginning = Time::new(begin_hours, begin_minutes);
            let end = Time::new(end_hours, end_minutes);

            if beginning > end || end - beginning < MIN_TIME_DISCRETIZATION {
                let error: Result<()> = Err(InvalidInterval::new());
                reset_custom_work_hours_if_err!(ui, data, error);
            }

            let current_entity = ui
                .borrow()
                .current_entity()
                .expect("Current entity should be set before adding custom work hours");
            let work_hours = data
                .custom_work_hours_of(current_entity.name())
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not get the custom work hours of {}",
                        current_entity.name()
                    )
                });
            let interval = TimeInterval::new(beginning, end);

            if position < work_hours.len() {
                reset_custom_work_hours_if_err!(
                    ui,
                    data,
                    data.update_custom_work_interval_for(
                        current_entity.name(),
                        work_hours[position],
                        interval
                    )
                );
            } else {
                reset_custom_work_hours_if_err!(
                    ui,
                    data,
                    data.add_custom_work_interval_for(current_entity.name(), interval)
                );
            }
        });

        self.ui
            .borrow_mut()
            .custom_work_hours_builder()
            .set_work_interval_editing_done_callback(work_hour_editing_done_callback);
    }

    fn set_custom_work_hour_remove_callback(&self) {
        let ui = self.ui.clone();
        let data = self.data.clone();
        let work_hour_editing_done_callback = Rc::new(move |position| {
            let mut data = data.borrow_mut();
            let current_entity = ui
                .borrow()
                .current_entity()
                .expect("Current entity should be set before adding custom work hours");
            let work_hours = data
                .custom_work_hours_of(current_entity.name())
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not get the custom work hours of {}",
                        current_entity.name()
                    )
                });

            if position < work_hours.len() {
                reset_custom_work_hours_if_err!(
                    ui,
                    data,
                    data.remove_custom_work_interval_for(
                        current_entity.name(),
                        work_hours[position]
                    )
                );
            } else {
                data.events().borrow_mut().emit_work_hours_changed(&data);
            }
        });

        self.ui
            .borrow_mut()
            .custom_work_hours_builder()
            .set_work_interval_remove_callback(work_hour_editing_done_callback);
    }
}
