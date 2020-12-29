use super::ActivityInsertionUi;

use cairo;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

const NUM_HOURS_IN_DAY: i32 = 24;

// Cairo calculates from the half of a pixel. This is used as an offset.
const HALF_PIXEL: f64 = 0.5;

const FULL_LINE_RGB: f64 = 0.7;
const DASH_LINE_RGB: f64 = 0.8;
const CORNER_SEPARATOR_LINE_RGB: f64 = 0.89;
const BACKGROUND_RGB: f64 = 0.99;
const HOUR_FONT_RGB: f64 = 0.4;
const SCHEDULE_FONT_RGB: f64 = 0.2;

const DASH_SIZE: f64 = 5.0;
const HOUR_FONT_SIZE: f64 = 14.0;
const SCHEDULE_FONT_SIZE: f64 = 20.0;
const LINE_WIDTH: f64 = 0.5;

const HEADER_SEPARATOR_HEIGHT_PROPORTION: f64 = 0.33;
const SCHEDULE_FONT_Y_OFFSET: f64 = 12.0;

const MIN_SCHEDULE_WIDTH: f64 = 200.0;
const MAX_SCHEDULE_WIDTH: f64 = 450.0;

impl ActivityInsertionUi {
    pub(super) fn connect_draw(&self) {
        self.connect_draw_hours();
        self.connect_draw_corner();
        self.connect_draw_schedules();
        self.connect_draw_header();
    }

    fn connect_draw_hours(&self) {
        fetch_from!(self, hours_drawing);

        hours_drawing.connect_draw(move |w, c| {
            draw_hours(
                &c,
                w.get_allocated_width() as f64,
                w.get_allocated_height() as f64,
            );
            gtk::Inhibit(false)
        });
    }

    fn connect_draw_corner(&self) {
        fetch_from!(self, corner_drawing);

        corner_drawing.connect_draw(move |w, c| {
            draw_corner(
                &c,
                w.get_allocated_width() as f64,
                w.get_allocated_height() as f64,
            );
            gtk::Inhibit(false)
        });
    }

    fn connect_draw_schedules(&self) {
        fetch_from!(self, schedules_drawing);

        schedules_drawing.connect_draw(move |w, c| {
            draw_schedules(
                &c,
                w.get_allocated_width() as f64,
                w.get_allocated_height() as f64,
            );
            gtk::Inhibit(false)
        });
    }

    fn connect_draw_header(&self) {
        fetch_from!(self, header_drawing);
        let schedules = self.schedules_to_show.clone();
        header_drawing.connect_draw(move |w, c| {
            draw_header(schedules.clone(), &c, w);
            gtk::Inhibit(false)
        });
    }
}

fn draw_hours(c: &cairo::Context, width: f64, height: f64) {
    draw_background_and_lines(c, width, height);

    // Draw the hour numbers
    c.set_source_rgb(HOUR_FONT_RGB, HOUR_FONT_RGB, HOUR_FONT_RGB);
    c.set_font_size(HOUR_FONT_SIZE);
    let y_step = get_height_for_one_hour(height);
    let mut current_y = y_step / 5.0 + HALF_PIXEL;
    for hour in 0..NUM_HOURS_IN_DAY {
        let text_to_display = &format!("{:02}:00", hour);
        let size_of_text = c.text_extents(text_to_display).width;
        // Center the text
        let x_offset = (width - size_of_text) / 2.0;
        c.move_to(x_offset, current_y);
        c.show_text(&format!("{:02}:00", hour));
        current_y += y_step;
    }

    // Draw the separation line between hours and schedules
    c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);
    c.set_dash(&[], 0.0);
    c.move_to(width, height);
    c.line_to(width, 0.0);
    c.stroke();
}

fn draw_corner(c: &cairo::Context, width: f64, height: f64) {
    paint_background_uniform(c);

    // Draw the hour-schedule separation line
    c.set_source_rgb(
        CORNER_SEPARATOR_LINE_RGB,
        CORNER_SEPARATOR_LINE_RGB,
        CORNER_SEPARATOR_LINE_RGB,
    );
    c.set_dash(&[], 0.0);
    c.move_to(width, height);
    c.line_to(width, HEADER_SEPARATOR_HEIGHT_PROPORTION * height);
    c.stroke();
}

fn draw_schedules(c: &cairo::Context, width: f64, height: f64) {
    draw_background_and_lines(c, width, height);
}

fn draw_header(schedules: Arc<Mutex<Vec<String>>>, c: &cairo::Context, w: &gtk::DrawingArea) {
    // TODO resizing will not work when schedules are removed.
    // 1 - Should first try to resize to "visible size"
    // 2 - Calculate the size of schedules outside
    // 3 - Draw schedule separations
    // 3 - Make schedules and header one single scroll bar
    // 4 - Add remaining time, deletion
    // 5 - Draw schedules
    // 6 - Support for drag & drop (via eventBox)

    // Get schedules width
    let width = w.get_allocated_width() as f64;
    let height = w.get_allocated_height() as f64;

    no_notify_assign_or_return!(schedules, schedules.try_lock());
    let nb_schedules = schedules.len();
    let width_per_schedule = width / nb_schedules as f64;

    let width_per_schedule = width_per_schedule.max(MIN_SCHEDULE_WIDTH);
    let width_per_schedule = width_per_schedule.min(MAX_SCHEDULE_WIDTH);

    // Resize drawing area
    let width_taken_by_schedules = width_per_schedule * nb_schedules as f64;
    if width_per_schedule * nb_schedules as f64 > width {
        w.set_size_request(width_taken_by_schedules as i32, height as i32);
        w.queue_resize();
        return;
    }

    paint_background_uniform(c);

    // Draw schedule separators
    c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);
    c.set_line_width(LINE_WIDTH);
    c.set_dash(&[], 0.0);
    let mut current_x = 0.0;
    for _i in 0..nb_schedules {
        current_x += width_per_schedule;
        c.move_to(current_x, height);
        c.line_to(current_x, HEADER_SEPARATOR_HEIGHT_PROPORTION * height);
    }
    c.stroke();

    // Draw schedule names
    c.set_source_rgb(SCHEDULE_FONT_RGB, SCHEDULE_FONT_RGB, SCHEDULE_FONT_RGB);
    c.set_font_size(SCHEDULE_FONT_SIZE);
    let mut current_x = 0.0;
    for entity_name in schedules.iter() {
        let size_of_text = c.text_extents(&entity_name).width;
        // Center the text
        let x_offset = (width_per_schedule - size_of_text) / 2.0;
        c.move_to(current_x + x_offset, height - SCHEDULE_FONT_Y_OFFSET);
        c.show_text(&entity_name);
        current_x += width_per_schedule;
    }
}

fn draw_background_and_lines(c: &cairo::Context, width: f64, height: f64) {
    paint_background_uniform(c);

    // Draw hour lines
    c.set_line_width(LINE_WIDTH);
    c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);

    let y_step = get_height_for_one_hour(height);
    // Half pixel offset because cairo calculates from the half of a pixel
    let mut current = HALF_PIXEL;
    for _hour in 0..NUM_HOURS_IN_DAY {
        c.move_to(0.0, current);
        c.line_to(width, current);
        current += y_step;
    }
    c.stroke();

    // Draw half hour lines
    c.set_dash(&[DASH_SIZE], 0.0);
    c.set_source_rgb(DASH_LINE_RGB, DASH_LINE_RGB, DASH_LINE_RGB);
    let mut current = y_step / 2.0 + HALF_PIXEL;
    for _half_hour in 0..NUM_HOURS_IN_DAY {
        c.move_to(0.0, current);
        c.line_to(width, current);
        current += y_step;
    }
    c.stroke();
}

fn paint_background_uniform(c: &cairo::Context) {
    c.set_source_rgb(BACKGROUND_RGB, BACKGROUND_RGB, BACKGROUND_RGB);
    c.paint();
}

fn get_height_for_one_hour(total_height: f64) -> f64 {
    total_height / NUM_HOURS_IN_DAY as f64
}
