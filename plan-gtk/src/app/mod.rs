#[macro_use]
pub mod macros;

pub mod app_builder;
//pub mod app_data;
pub mod connect;
pub mod notify;
pub mod ui;

use glib::clone;
use gtk::prelude::*;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::app::notify::notify_err;
use crate::config::APP_NAME;
use plan_backend::data::{Data, Time, TimeInterval};
use ui::Ui;

pub struct App {
    data: Arc<Mutex<Data>>,
    ui: Arc<Mutex<Ui>>,
}

impl App {
    /// Loads UI files in UI builder, binds mainwindow to application and sets title.
    pub fn new(application: &gtk::Application) -> App {
        let builder = gtk::Builder::new();
        builder
            .add_from_resource("/com/github/paulmconstant/plan/ui/main_window.ui")
            .expect("Could not load ui file: main_window.ui");

        builder
            .add_from_resource("/com/github/paulmconstant/plan/ui/data_window.ui")
            .expect("Could not load ui file: data_window.ui");

        let data = Arc::new(Mutex::new(Data::new()));
        let ui = Arc::new(Mutex::new(Ui::new(builder)));

        macro_rules! reset_work_hours_if_err {
            ($data:ident, $operation:expr) => {
                if let Err(e) = $operation {
                    $data.events().borrow_mut().emit_work_hours_changed(&$data);
                    notify_err(e);
                }
            };
        }

        let work_hour_editing_done_callback = Arc::new(
            clone!(@strong data => move |position, builder: gtk::Builder| {
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
                let interval = TimeInterval::new(beginning, end);

                if position < work_hours.len() {
                    reset_work_hours_if_err!(data, data.update_work_interval(work_hours[position], interval));
                } else {
                    reset_work_hours_if_err!(data, data.add_work_interval(interval));
                }
            }),
        );

        ui.lock()
            .unwrap()
            .set_work_interval_editing_done_callback(work_hour_editing_done_callback);
        fetch_from!(ui.lock().unwrap(), main_window);
        main_window.set_application(Some(application));
        main_window.set_title(APP_NAME);

        App { data, ui }
    }

    pub fn ui(&self) -> MutexGuard<Ui> {
        self.ui.lock().unwrap()
    }

    pub fn data(&self) -> MutexGuard<Data> {
        self.data.lock().unwrap()
    }
}
