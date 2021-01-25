use super::ActivityInsertionUi;

use cairo;
use gtk::prelude::*;

use crate::app::ui::EntityToShow;

use felix_backend::data::{TimeInterval, MIN_TIME_DISCRETIZATION};

const NUM_HOURS_IN_DAY: i32 = 24;

// Cairo calculates from the half of a pixel. This is used as an offset.
const HALF_PIXEL: f64 = 0.5;

const FULL_LINE_RGB: f64 = 0.7;
const DASH_LINE_RGB: f64 = 0.8;
const CORNER_SEPARATOR_LINE_RGB: f64 = 0.89;

const IN_WORK_HOURS_RGB: f64 = 0.99;
const OUTSIDE_WORK_HOURS_RGB: f64 = 0.93;
const HOUR_FONT_RGB: f64 = 0.4;
const SCHEDULE_FONT_RGB: f64 = 0.2;

const DASH_SIZE: f64 = 5.0;
const HOUR_FONT_SIZE: f64 = 14.0;
const SCHEDULE_FONT_SIZE: f64 = 16.0;
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
        fetch_from!(self, schedules_drawing, header_scrolled_window);
        let schedules = self.schedules_to_show.clone();

        schedules_drawing.connect_draw(move |w, c| {
            let header_visible_width = header_scrolled_window.get_allocated_width() as f64;
            let schedules = schedules.lock().unwrap();
            if let Some(width_per_schedule) =
                resize_draw_area(schedules.len(), w, header_visible_width)
            {
                draw_schedules(&c, w, &schedules, width_per_schedule);
            }
            gtk::Inhibit(false)
        });
    }

    fn connect_draw_header(&self) {
        fetch_from!(self, header_drawing, header_scrolled_window);
        let schedules = self.schedules_to_show.clone();
        header_drawing.connect_draw(move |w, c| {
            let header_visible_width = header_scrolled_window.get_allocated_width() as f64;
            let schedules = schedules.lock().unwrap();
            if let Some(width_per_schedule) =
                resize_draw_area(schedules.len(), w, header_visible_width)
            {
                draw_header(&c, w, &schedules, width_per_schedule);
            }
            gtk::Inhibit(false)
        });
    }
}

fn draw_hours(c: &cairo::Context, width: f64, height: f64) {
    paint_background_uniform(c, IN_WORK_HOURS_RGB);
    draw_hour_lines(c, width, height);

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
    paint_background_uniform(c, IN_WORK_HOURS_RGB);

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

fn draw_schedules(
    c: &cairo::Context,
    w: &gtk::DrawingArea,
    schedules: &Vec<EntityToShow>,
    width_per_schedule: f64,
) {
    let width = w.get_allocated_width() as f64;
    let height = w.get_allocated_height() as f64;
    paint_background_uniform(c, OUTSIDE_WORK_HOURS_RGB);

    let work_hours_of_entities = schedules.iter().map(|entity| entity.work_hours());
    draw_inside_work_hours_background(c, height, width_per_schedule, work_hours_of_entities);
    // TODO Draw inserted activities

    draw_hour_lines(c, width, height);
    let nb_schedules = schedules.len();
    // Draw schedule separators
    c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);
    c.set_line_width(LINE_WIDTH);
    c.set_dash(&[], 0.0);
    let mut current_x = 0.0;
    for _i in 0..nb_schedules {
        current_x += width_per_schedule;
        c.move_to(current_x, height);
        c.line_to(current_x, 0.0);
    }
    c.stroke();
}

/// Resizes the draw area if necessary, else returns the width which each schedule should take.
fn resize_draw_area(nb_schedules: usize, w: &gtk::DrawingArea, visible_width: f64) -> Option<f64> {
    let width = w.get_allocated_width() as f64;
    let height = w.get_allocated_height() as f64;

    let width_per_schedule = compute_schedule_width(nb_schedules, visible_width);

    let width_taken_by_schedules = width_per_schedule * nb_schedules as f64;
    match width_per_schedule * nb_schedules as f64 {
        required_width if required_width > width => {
            // Header is too small
            w.set_size_request(width_taken_by_schedules as i32, height as i32);
            w.queue_resize();
            None
        }
        required_width if required_width < width && width > visible_width => {
            // Header is too big
            w.set_size_request(visible_width as i32, height as i32);
            w.queue_resize();
            None
        }
        _ => Some(width_per_schedule),
    }
}

fn compute_schedule_width(nb_schedules: usize, visible_widget_width: f64) -> f64 {
    let width_per_schedule = visible_widget_width / nb_schedules as f64;

    let width_per_schedule = width_per_schedule.max(MIN_SCHEDULE_WIDTH);
    let width_per_schedule = width_per_schedule.min(MAX_SCHEDULE_WIDTH);
    width_per_schedule
}

fn draw_header(
    c: &cairo::Context,
    w: &gtk::DrawingArea,
    schedules: &Vec<EntityToShow>,
    width_per_schedule: f64,
) {
    // TODO resizing will not work when schedules are removed ?
    // 1 - Should first try to resize to "visible size" OK
    // 2 - Calculate the size of schedules outside OK
    // 3 - Draw schedule separations OK
    // 3 - Make schedules and header one single scroll bar OK
    // 4 - Add remaining time, deletion
    // 5 - Draw schedules
    // 6 - Support for drag & drop (via eventBox)

    // Get schedules width
    let height = w.get_allocated_height() as f64;

    let nb_schedules = schedules.len();

    paint_background_uniform(c, IN_WORK_HOURS_RGB);

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
    for entity_name in schedules.iter().map(|entity| entity.name()) {
        let size_of_text = c.text_extents(&entity_name).width;
        // Center the text
        let x_offset = (width_per_schedule - size_of_text) / 2.0;
        c.move_to(current_x + x_offset, height - SCHEDULE_FONT_Y_OFFSET);
        c.show_text(&entity_name);
        current_x += width_per_schedule;
    }
}

fn draw_hour_lines(c: &cairo::Context, width: f64, height: f64) {
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

fn paint_background_uniform(c: &cairo::Context, color: f64) {
    c.set_source_rgb(color, color, color);
    c.paint();
}

fn get_height_for_one_hour(total_height: f64) -> f64 {
    total_height / NUM_HOURS_IN_DAY as f64
}

fn get_height_for_min_discretization(total_height: f64) -> f64 {
    let num_min_discretization_in_hour = 60 / MIN_TIME_DISCRETIZATION.minutes();
    get_height_for_one_hour(total_height) / num_min_discretization_in_hour as f64
}

fn draw_inside_work_hours_background<'a, TimeIntervalIterator>(
    c: &cairo::Context,
    height: f64,
    width_per_schedule: f64,
    work_hours_of_entities: TimeIntervalIterator,
) where
    TimeIntervalIterator: Iterator<Item = &'a Vec<TimeInterval>>,
{
    let min_discretization_height = get_height_for_min_discretization(height);

    c.set_source_rgb(IN_WORK_HOURS_RGB, IN_WORK_HOURS_RGB, IN_WORK_HOURS_RGB);

    let mut current_x = 0.0;
    for work_hours in work_hours_of_entities {
        for interval in work_hours {
            let height_begin = interval.beginning().n_times_min_discretization() as f64
                * min_discretization_height;
            let height_to_paint =
                interval.duration().n_times_min_discretization() as f64 * min_discretization_height;
            c.rectangle(current_x, height_begin, width_per_schedule, height_to_paint);
            c.fill();
        }
        current_x += width_per_schedule;
    }
}
