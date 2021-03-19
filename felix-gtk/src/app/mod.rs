#[macro_use]
pub mod macros;

pub mod app_builder;
pub mod connect;
pub mod notify;
pub mod ui;

use crate::config::APP_NAME;
use felix_backend::data::Data;
use ui::Ui;

use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct App {
    data: Rc<RefCell<Data>>,
    ui: Rc<RefCell<Ui>>,
}

impl App {
    /// Loads UI files in UI builder, binds mainwindow to application and sets title.
    pub fn new(application: &gtk::Application) -> App {
        let data = init_data();
        let ui = init_ui(&application);

        App { data, ui }
    }

    fn on_activity_duration_changed_start_polling_to_insert_it_again(
        &self,
        data: &Data,
        polling_duration_counter: Rc<RefCell<u32>>,
    ) {
        if data.activities_were_uninserted_and_can_maybe_be_inserted_back() {
            const FREQUENCY_CHECK_COMPUTATION_RESULT_DONE_MS: u32 = 5;
            const TIMEOUT_CHECK_COMPUTATION_RESULT_DONE_MS: u32 = 1000;
            const TIMEOUT_MAX_COUNTER_VALUE: u32 = TIMEOUT_CHECK_COMPUTATION_RESULT_DONE_MS
                / FREQUENCY_CHECK_COMPUTATION_RESULT_DONE_MS;

            let mut counter = polling_duration_counter.borrow_mut();
            if *counter == 0 {
                // The polling function is not currently running
                // Add one preemptively so that the function is never called twice
                *counter += 1;

                let data = self.data.clone();
                let counter = polling_duration_counter.clone();
                // Launch polling function
                glib::timeout_add_local(FREQUENCY_CHECK_COMPUTATION_RESULT_DONE_MS, move || {
                    let mut counter = counter.borrow_mut();
                    if *counter > TIMEOUT_MAX_COUNTER_VALUE {
                        *counter = 0;
                        data.borrow_mut()
                            .clear_list_activities_removed_because_duration_increased();
                        glib::Continue(false)
                    } else {
                        *counter += 1;
                        data.borrow_mut()
                            .insert_activities_removed_because_duration_increased_in_closest_spot();
                        glib::Continue(true)
                    }
                });
            } else {
                //Extend polling function duration
                *counter = 0;
            }
        }
    }
}

fn init_data() -> Rc<RefCell<Data>> {
    Rc::new(RefCell::new(Data::new()))
}

fn init_ui(application: &gtk::Application) -> Rc<RefCell<Ui>> {
    // Initialize UI
    let builder = gtk::Builder::new();
    builder
        .add_from_resource("/com/github/paulmconstant/felix/ui/main_window.ui")
        .expect("Could not load ui file: main_window.ui");

    builder
        .add_from_resource("/com/github/paulmconstant/felix/ui/data_window.ui")
        .expect("Could not load ui file: data_window.ui");

    let ui = Rc::new(RefCell::new(Ui::new(builder)));

    fetch_from!(ui.borrow(), main_window);
    main_window.set_application(Some(application));
    main_window.set_title(APP_NAME);
    ui
}
