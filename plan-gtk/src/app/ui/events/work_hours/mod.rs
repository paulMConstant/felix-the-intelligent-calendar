use crate::app::ui::Ui;

use plan_backend::data::{Data, TimeInterval};

use glib::clone;
use gtk::prelude::*;

impl Ui {
    pub fn on_add_work_hour(&self, current_work_hours: Vec<TimeInterval>) {
        self.remove_work_hours_if_any();
        self.add_new_work_hours(current_work_hours, true);
    }

    pub fn on_work_hours_changed(&self, data: &Data) {
        self.remove_work_hours_if_any();
        self.add_new_work_hours(data.work_hours(), false);
    }

    fn remove_work_hours_if_any(&self) {
        fetch_from!(self, work_hours_scrolled_window);

        // One spot is occupied by AddWorkHourButton. Button + work hours > 1
        for child in work_hours_scrolled_window.get_children() {
            work_hours_scrolled_window.remove(&child);
        }
    }

    fn add_new_work_hours(&self, current_work_hours: Vec<TimeInterval>, add_work_hour: bool) {
        fetch_from!(self, work_hours_scrolled_window);
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
        self.work_interval_builders.lock().unwrap().clear();

        // Add the current work hours and a new one
        for (index, interval) in current_work_hours.iter().enumerate() {
            work_intervals.push(self.new_work_interval(Some(*interval), index));
        }
        if add_work_hour {
            work_intervals.push(self.new_work_interval(None, current_work_hours.len()));
        }

        // Pack the work interval boxes into the main box
        for interval in work_intervals {
            work_intervals_box.pack_start(&interval, true, true, 0);
        }

        work_intervals_box
    }

    fn new_work_interval(
        &self,
        interval: Option<TimeInterval>,
        position_of_interval: usize,
    ) -> gtk::Box {
        let builder = new_time_interval_builder();

        fetch_from_builder!(builder,
         edit_button=gtk::Button:"TimeIntervalEditButton",
         interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
         interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
         interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
         interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
        );

        if let Some(interval) = interval {
            edit_button.set_label("Edit");
            let work_interval_builders = self.work_interval_builders.clone();
            let editing_done_callback = self.work_interval_editing_done_callback.clone();

            edit_button.connect_clicked(move |_| {
                for (index, builder) in work_interval_builders.lock().unwrap().iter().enumerate() {
        fetch_from_builder!(builder,
                            edit_button=gtk::Button:"TimeIntervalEditButton",
                            interval_begin_hours=gtk::SpinButton:"IntervalBeginHourSpin",
                            interval_begin_minutes=gtk::SpinButton:"IntervalBeginMinuteSpin",
                            interval_end_hours=gtk::SpinButton:"IntervalEndHourSpin",
                            interval_end_minutes=gtk::SpinButton:"IntervalEndMinuteSpin"
                           );
                    if index == position_of_interval {
                        edit_button.set_visible(true);
                        edit_button.set_label("Ok");
                        interval_begin_hours.set_editable(true);
                        interval_begin_minutes.set_editable(true);
                        interval_end_hours.set_editable(true);
                        interval_end_minutes.set_editable(true);
                        let editing_done_callback = editing_done_callback.clone();
                        edit_button.connect_clicked(clone!(@weak builder => move |_| editing_done_callback(position_of_interval, builder.clone())));
                        edit_button.connect_clicked(clone!(@weak edit_button => move |_| edit_button.set_label("Edit")));
                    } else {
                        edit_button.set_visible(false);
                    }
                }
            });

            interval_begin_hours.set_editable(false);
            interval_begin_minutes.set_editable(false);
            interval_end_hours.set_editable(false);
            interval_end_minutes.set_editable(false);

            interval_begin_hours.set_value(interval.beginning().hours() as f64);
            interval_begin_minutes.set_value(interval.beginning().minutes() as f64);
            interval_end_hours.set_value(interval.end().hours() as f64);
            interval_end_minutes.set_value(interval.end().minutes() as f64);
        } else {
            edit_button.set_label("Ok");
            let editing_done_callback = self.work_interval_editing_done_callback.clone();
            edit_button.connect_clicked(clone!(@weak builder => move |_| editing_done_callback(position_of_interval, builder.clone())));
            edit_button.connect_clicked(
                clone!(@weak edit_button => move |_| edit_button.set_label("Edit")),
            );
        };

        let time_interval: gtk::Box = builder
            .get_object("TimeIntervalBox")
            .expect("Could not load TimeIntervalBox");

        self.work_interval_builders.lock().unwrap().push(builder);
        time_interval
    }
}

fn new_time_interval_builder() -> gtk::Builder {
    let builder = gtk::Builder::new();
    builder
        .add_from_resource("/com/github/paulmconstant/plan/ui/time_interval.ui")
        .expect("Could not load ui file: time_interval.ui");
    builder
}
