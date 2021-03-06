use crate::app::ui::helpers::format::format_time_spin_button;

use felix_data::TimeInterval;

use glib::clone;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

// Macro placed first because order matters with macros
macro_rules! start_editing_callback {
    ($work_interval_builders:ident, $position_of_interval: ident, $editing_done_callback: ident) => {
        move |_| {
            for (index, builder) in $work_interval_builders.borrow().iter().enumerate() {
                if index == $position_of_interval {
                    fetch_from_builder!(builder, edit_button=gtk::Button:"TimeIntervalEditButton");
                    // This button ends the editing of current interval
                    edit_button.set_sensitive(true);
                    set_editing_done_icon_for_button(&edit_button);
                    make_spinbuttons_sensitive(&builder, true);

                    let editing_done_callback = $editing_done_callback.clone();
                    edit_button.connect_clicked(clone!(@strong builder => move |_|
                       editing_done_callback($position_of_interval, builder.clone())));
                } else {
                    // All other buttons are not to be touched until editing is done
                    fetch_from_builder!(builder,
                                        edit_button=gtk::Button:"TimeIntervalEditButton",
                                        delete_button=gtk::Button:"TimeIntervalDeleteButton");
                    for button in &[edit_button, delete_button] {
                        button.set_sensitive(false);
                    }
                }
            }
        }
    };
}

pub struct WorkHoursBuilder {
    work_hours_scrolled_window: Option<gtk::ScrolledWindow>,
    work_interval_builders: Rc<RefCell<Vec<gtk::Builder>>>,
    work_interval_editing_done_callback: Rc<dyn Fn(usize, gtk::Builder)>,
    work_interval_remove_callback: Rc<dyn Fn(usize)>,
}

impl WorkHoursBuilder {
    pub fn new() -> WorkHoursBuilder {
        WorkHoursBuilder {
            work_hours_scrolled_window: None,
            work_interval_builders: Rc::new(RefCell::new(Vec::new())),
            work_interval_editing_done_callback: Rc::new(Box::new(|_, _| {
                panic!("Work interval editing done callback was called before being set")
            })),
            work_interval_remove_callback: Rc::new(Box::new(|_| {
                panic!("Work interval remove callback was called before being set")
            })),
        }
    }

    pub fn set_work_hours_scrolled_window(
        &mut self,
        work_hours_scrolled_window: gtk::ScrolledWindow,
    ) {
        self.work_hours_scrolled_window = Some(work_hours_scrolled_window);
    }

    pub fn set_work_interval_editing_done_callback(
        &mut self,
        work_interval_editing_done_callback: Rc<dyn Fn(usize, gtk::Builder)>,
    ) {
        self.work_interval_editing_done_callback = work_interval_editing_done_callback;
    }

    pub fn set_work_interval_remove_callback(
        &mut self,
        work_interval_remove_callback: Rc<dyn Fn(usize)>,
    ) {
        self.work_interval_remove_callback = work_interval_remove_callback;
    }

    pub fn on_add_work_hour(&self, current_work_hours: Vec<TimeInterval>) {
        self.remove_work_hours_if_any();
        self.add_new_work_hours(current_work_hours, true);
    }

    pub fn on_work_hours_changed(&self, work_hours: Vec<TimeInterval>) {
        self.remove_work_hours_if_any();
        self.add_new_work_hours(work_hours, false);
    }

    fn remove_work_hours_if_any(&self) {
        let work_hours_scrolled_window = self.work_hours_scrolled_window.as_ref()
        .expect("WorkHoursBuilder should be initialized with a scrolled window before events are triggered.");
        for child in work_hours_scrolled_window.get_children() {
            work_hours_scrolled_window.remove(&child);
        }
    }

    fn add_new_work_hours(&self, current_work_hours: Vec<TimeInterval>, add_work_hour: bool) {
        let work_hours_scrolled_window = self.work_hours_scrolled_window.as_ref()
        .expect("WorkHoursBuilder should be initialized with a scrolled window before events are triggered.");

        let work_intervals_box = self.create_work_intervals_box(current_work_hours, add_work_hour);
        work_hours_scrolled_window.add(&work_intervals_box);
        work_intervals_box.show();
    }

    fn create_work_intervals_box(
        &self,
        current_work_hours: Vec<TimeInterval>,
        add_work_hour: bool,
    ) -> gtk::Box {
        let work_intervals_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let mut work_intervals: Vec<gtk::Box> = Vec::with_capacity(current_work_hours.len() + 1);
        self.work_interval_builders.borrow_mut().clear();

        // Add the current work hours and a new one
        for (index, interval) in current_work_hours.iter().enumerate() {
            work_intervals.push(self.new_registered_work_interval(Some(*interval), index));
        }
        if add_work_hour {
            let position_of_interval = current_work_hours.len();
            work_intervals.push(self.new_registered_work_interval(None, position_of_interval));
            // The new work hour is being edited - prevent other intervals from being edited
            self.make_other_buttons_insensitive(position_of_interval);
        }

        // Pack the work interval boxes into the main box
        for interval in work_intervals {
            work_intervals_box.pack_start(&interval, true, true, 0);
        }

        work_intervals_box
    }

    /// Creates a new work interval and stores its builder in self.work_interval_builders.
    fn new_registered_work_interval(
        &self,
        interval: Option<TimeInterval>,
        position_of_interval: usize,
    ) -> gtk::Box {
        let builder = new_time_interval_builder();

        format_time_interval_spinbuttons(&builder);

        if let Some(interval) = interval {
            self.init_time_interval_builder_with_given_interval(
                &builder,
                interval,
                position_of_interval,
            );
        } else {
            self.init_time_interval_builder_without_interval(&builder, position_of_interval);
        };

        let time_interval: gtk::Box = builder
            .get_object("TimeIntervalBox")
            .expect("Could not load TimeIntervalBox");

        self.work_interval_builders.borrow_mut().push(builder);

        time_interval
    }

    fn init_time_interval_builder_with_given_interval(
        &self,
        builder: &gtk::Builder,
        interval: TimeInterval,
        position_of_interval: usize,
    ) {
        fetch_from_builder!(builder,
                            edit_button=gtk::Button:"TimeIntervalEditButton",
                            delete_button=gtk::Button:"TimeIntervalDeleteButton");
        set_start_editing_icon_for_button(&edit_button);

        let work_interval_builders = self.work_interval_builders.clone();
        let editing_done_callback = self.work_interval_editing_done_callback.clone();

        edit_button.connect_clicked(start_editing_callback!(
            work_interval_builders,
            position_of_interval,
            editing_done_callback
        ));

        self.init_delete_button_callback(&delete_button, position_of_interval);

        make_spinbuttons_sensitive(&builder, false);
        update_interval_spinbuttons(&builder, interval);
    }

    fn init_time_interval_builder_without_interval(
        &self,
        builder: &gtk::Builder,
        position_of_interval: usize,
    ) {
        fetch_from_builder!(builder,
                            edit_button=gtk::Button:"TimeIntervalEditButton",
                            delete_button=gtk::Button:"TimeIntervalDeleteButton");
        set_editing_done_icon_for_button(&edit_button);

        let editing_done_callback = self.work_interval_editing_done_callback.clone();
        edit_button.connect_clicked(clone!(@strong builder => move |_|
            editing_done_callback(position_of_interval, builder.clone())));

        init_spinbuttons_to_default_value(&builder);
        self.init_delete_button_callback(&delete_button, position_of_interval);
    }

    fn make_other_buttons_insensitive(&self, position_of_sensitive_button: usize) {
        for (index, builder) in self.work_interval_builders.borrow().iter().enumerate() {
            if index == position_of_sensitive_button {
                continue;
            }
            fetch_from_builder!(builder,
                                edit_button=gtk::Button:"TimeIntervalEditButton",
                                delete_button=gtk::Button:"TimeIntervalDeleteButton");
            for button in &[edit_button, delete_button] {
                button.set_sensitive(false);
            }
        }
    }

    fn init_delete_button_callback(
        &self,
        delete_button: &gtk::Button,
        position_of_interval: usize,
    ) {
        let remove_work_interval_callback = self.work_interval_remove_callback.clone();
        delete_button.connect_clicked(move |_| remove_work_interval_callback(position_of_interval));
    }
}

fn new_time_interval_builder() -> gtk::Builder {
    let builder = gtk::Builder::new();
    builder
        .add_from_resource("/com/github/paulmconstant/felix/ui/time_interval.ui")
        .expect("Could not load ui file: time_interval.ui");
    builder
}

fn set_editing_done_icon_for_button(button: &gtk::Button) {
    set_button_icon(&button, "object-select-symbolic");
}

fn set_start_editing_icon_for_button(button: &gtk::Button) {
    set_button_icon(&button, "document-edit-symbolic");
}

fn set_button_icon(button: &gtk::Button, icon_name: &str) {
    button.set_label("");
    let image = gtk::Image::from_icon_name(Some(icon_name), gtk::IconSize::Button);
    button.set_image(Some(&image));
    button.set_always_show_image(true);
}

fn update_interval_spinbuttons(builder: &gtk::Builder, interval: TimeInterval) {
    fetch_from_builder!(builder,
     interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
     interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
     interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
     interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
    );
    interval_begin_hours.set_value(interval.beginning().hours() as f64);
    interval_begin_minutes.set_value(interval.beginning().minutes() as f64);
    interval_end_hours.set_value(interval.end().hours() as f64);
    interval_end_minutes.set_value(interval.end().minutes() as f64);
}

fn make_spinbuttons_sensitive(builder: &gtk::Builder, sensitive: bool) {
    fetch_from_builder!(builder,
        interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
        interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
        interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
        interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
    );

    for spinbutton in &[
        interval_begin_hours,
        interval_begin_minutes,
        interval_end_hours,
        interval_end_minutes,
    ] {
        spinbutton.set_sensitive(sensitive);
    }
}

fn format_time_interval_spinbuttons(builder: &gtk::Builder) {
    fetch_from_builder!(builder,
        interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
        interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
        interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
        interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
    );

    for spinbutton in &[
        interval_begin_hours,
        interval_begin_minutes,
        interval_end_hours,
        interval_end_minutes,
    ] {
        format_time_spin_button(spinbutton);
    }
}

fn init_spinbuttons_to_default_value(builder: &gtk::Builder) {
    fetch_from_builder!(builder,
        interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
        interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
        interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
        interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
    );
    let buttons = [
        interval_begin_hours,
        interval_begin_minutes,
        interval_end_hours,
        interval_end_minutes,
    ];
    let values = [8.0, 0.0, 12.0, 15.0];
    for (spinbutton, default_value) in buttons.iter().zip(values.iter()) {
        spinbutton.set_value(*default_value);
    }
}
