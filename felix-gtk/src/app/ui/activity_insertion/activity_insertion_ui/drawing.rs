use super::{costs_to_rgb::costs_to_rgb, ActivityInsertionUi, Schedules, NUM_HOURS_IN_DAY};

use felix_backend::{Rgba, Time};

use gtk::prelude::*;

use std::cell::RefCell;
use std::f64::consts::{FRAC_PI_4, PI};
use std::rc::Rc;

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
const SCHEDULE_FONT_SIZE: f64 = 14.0;
const LINE_WIDTH: f64 = 0.5;

const HEADER_SEPARATOR_HEIGHT_PROPORTION: f64 = 0.33;
const SCHEDULE_FONT_Y_OFFSET: f64 = 12.0;

const TIME_TOOLTIP_HEIGHT: i32 = 25;
const TIME_TOOLTIP_WIDTH: i32 = 100;
const TIME_TOOLTIP_FONT_SIZE: f64 = 14.0;
const TIME_TOOLTIP_FONT_RGB: f64 = 0.0;
const TIME_TOOLTIP_BACKGROUND_RGB: f64 = 0.9;
const TIME_TOOLTIP_Y_OFFSET: f64 = 18.0;
const TIME_TOOLTIP_X_OFFSET: f64 = 30.0;

pub const REMOVE_BUTTON_RADIUS: f64 = 9.0;
const REMOVE_BUTTON_Y_OFFSET: f64 = 17.0;
const REMOVE_BUTTON_X_OFFSET: f64 = 20.0;
const REMOVE_BUTTON_LINE_WIDTH: f64 = 1.5;

const ACTIVITY_NAME_FONT_SIZE: f64 = 14.0;

impl ActivityInsertionUi {
    pub(super) fn connect_draw(&self) {
        self.connect_draw_hours();
        self.connect_draw_corner();
        self.connect_draw_schedules();
        self.connect_draw_header();
        self.connect_draw_time_tooltip();
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
            if schedule_size_ok_or_resize(schedules.clone(), w, header_visible_width) {
                draw_schedules(&c, w, schedules.clone());
            }
            gtk::Inhibit(false)
        });
    }

    fn connect_draw_header(&self) {
        fetch_from!(self, header_drawing, header_scrolled_window);
        let schedules = self.schedules_to_show.clone();

        header_drawing.connect_draw(move |w, c| {
            let header_visible_width = header_scrolled_window.get_allocated_width() as f64;
            if schedule_size_ok_or_resize(schedules.clone(), w, header_visible_width) {
                draw_header(&c, w, schedules.clone());
            }
            gtk::Inhibit(false)
        });
    }

    fn connect_draw_time_tooltip(&self) {
        fetch_from!(self, schedules_drawing);

        let schedules = self.schedules_to_show.clone();

        schedules_drawing.connect_draw(move |_w, c| {
            draw_time_tooltip(&c, schedules.clone());
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

fn draw_schedules(c: &cairo::Context, w: &gtk::DrawingArea, schedules: Rc<RefCell<Schedules>>) {
    let width = w.get_allocated_width() as f64;
    let height = w.get_allocated_height() as f64;
    paint_background_uniform(c, OUTSIDE_WORK_HOURS_RGB);

    draw_inside_work_hours_background(c, height, &schedules);
    draw_inserted_activities(c, height, &schedules);
    // If we see that possible insertions overlap inserted activities, it's a bug.
    // Hence the order 1. Draw inserted activities 2. draw possible insertions.
    draw_possible_insertions_background(c, height, &schedules);

    let schedules = schedules.borrow();
    draw_hour_lines(c, width, height);
    let nb_schedules = schedules.entities_to_show.len();
    // Draw schedule separators
    c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);
    c.set_line_width(LINE_WIDTH);
    c.set_dash(&[], 0.0);
    let mut current_x = 0.0;
    for _i in 0..nb_schedules {
        current_x += schedules.width_per_schedule;
        c.move_to(current_x, height);
        c.line_to(current_x, 0.0);
    }
    c.stroke();
}

/// If the schedule size is good, returns true, else resizes the draw area and returns false
fn schedule_size_ok_or_resize(
    schedules: Rc<RefCell<Schedules>>,
    w: &gtk::DrawingArea,
    visible_width: f64,
) -> bool {
    let width = w.get_allocated_width() as f64;
    let height = w.get_allocated_height() as f64;

    let mut schedules = schedules.borrow_mut();
    let nb_schedules = schedules.entities_to_show.len();
    schedules.compute_schedule_width(visible_width);

    let width_taken_by_schedules = schedules.width_per_schedule * nb_schedules as f64;
    match width_taken_by_schedules {
        required_width if required_width > width => {
            // Header is too small
            w.set_size_request(width_taken_by_schedules as i32, height as i32);
            w.queue_resize();
            false
        }
        required_width if required_width < width && width > visible_width => {
            // Header is too big
            w.set_size_request(visible_width as i32, height as i32);
            w.queue_resize();
            false
        }
        _ => true,
    }
}

fn draw_header(c: &cairo::Context, w: &gtk::DrawingArea, schedules: Rc<RefCell<Schedules>>) {
    // Get schedules width
    let height = w.get_allocated_height() as f64;

    let schedules = schedules.borrow();
    let nb_schedules = schedules.entities_to_show.len();

    paint_background_uniform(c, IN_WORK_HOURS_RGB);

    // Draw schedule separators
    c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);
    c.set_line_width(LINE_WIDTH);
    c.set_dash(&[], 0.0);
    let mut current_x = 0.0;
    for _i in 0..nb_schedules {
        current_x += schedules.width_per_schedule;
        c.move_to(current_x, height);
        c.line_to(current_x, HEADER_SEPARATOR_HEIGHT_PROPORTION * height);
    }
    c.stroke();

    // Draw schedule names
    c.set_source_rgb(SCHEDULE_FONT_RGB, SCHEDULE_FONT_RGB, SCHEDULE_FONT_RGB);
    c.set_font_size(SCHEDULE_FONT_SIZE);
    let mut current_x = 0.0;
    for entity in &schedules.entities_to_show {
        let total_time: Time = entity
            .work_hours()
            .iter()
            .map(|interval| interval.duration())
            .sum();
        let text = format!(
            "{} - {} / {}",
            entity.name(),
            entity.free_time(),
            total_time
        );

        c.set_line_width(LINE_WIDTH);
        c.set_source_rgb(SCHEDULE_FONT_RGB, SCHEDULE_FONT_RGB, SCHEDULE_FONT_RGB);
        let size_of_text = c.text_extents(&text).width;
        // Center the text
        let x_offset = (schedules.width_per_schedule - size_of_text) / 2.0;
        c.move_to(current_x + x_offset, height - SCHEDULE_FONT_Y_OFFSET);
        c.show_text(&text);

        // Draw remove button
        c.set_line_width(REMOVE_BUTTON_LINE_WIDTH);
        c.set_source_rgb(FULL_LINE_RGB, FULL_LINE_RGB, FULL_LINE_RGB);
        let (x_remove_button, y_remove_button) = get_center_of_remove_button(&schedules, current_x);

        // Coordinates are reversed in cairo - height at the top, 0 at the bottom
        // Countrary to the application: 0 at the top, height at the bottom
        let y_remove_button = height - y_remove_button;
        // Move to where the button (circle) will start to be drawn:
        // not in the middle but at 0 radian
        c.move_to(x_remove_button + REMOVE_BUTTON_RADIUS, y_remove_button);
        c.arc(
            x_remove_button,
            y_remove_button,
            REMOVE_BUTTON_RADIUS,
            0.0,
            2.0 * PI,
        );

        // Draw line across the circle
        c.move_to(
            x_remove_button - FRAC_PI_4.cos() * REMOVE_BUTTON_RADIUS,
            y_remove_button - FRAC_PI_4.sin() * REMOVE_BUTTON_RADIUS,
        );
        c.line_to(
            x_remove_button + FRAC_PI_4.cos() * REMOVE_BUTTON_RADIUS,
            y_remove_button + FRAC_PI_4.sin() * REMOVE_BUTTON_RADIUS,
        );
        c.stroke();

        current_x += schedules.width_per_schedule;
    }
}

/// Returns the (x, y) position of the remove button based on the left edge of the schedule.
pub fn get_center_of_remove_button(schedules: &Schedules, schedule_left_edge_x: f64) -> (f64, f64) {
    (
        schedule_left_edge_x + schedules.width_per_schedule - REMOVE_BUTTON_X_OFFSET,
        REMOVE_BUTTON_Y_OFFSET,
    )
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

pub fn get_height_for_one_hour(total_height: f64) -> f64 {
    total_height / NUM_HOURS_IN_DAY as f64
}

fn draw_inside_work_hours_background(
    c: &cairo::Context,
    height: f64,
    schedules: &Rc<RefCell<Schedules>>,
) {
    let mut schedules = schedules.borrow_mut();
    schedules.compute_height_for_min_discretization(height);

    c.set_source_rgb(IN_WORK_HOURS_RGB, IN_WORK_HOURS_RGB, IN_WORK_HOURS_RGB);

    for (index, work_hours) in schedules
        .entities_to_show
        .iter()
        .map(|entity| entity.work_hours())
        .enumerate()
    {
        for interval in work_hours {
            let height_begin = interval.beginning().n_times_min_discretization() as f64
                * schedules.height_per_min_discretization;
            let height_to_paint = interval.duration().n_times_min_discretization() as f64
                * schedules.height_per_min_discretization;
            c.rectangle(
                index as f64 * schedules.width_per_schedule,
                height_begin,
                schedules.width_per_schedule,
                height_to_paint,
            );
            c.fill();
        }
    }
}

fn draw_inserted_activities(c: &cairo::Context, height: f64, schedules: &Rc<RefCell<Schedules>>) {
    let mut schedules = schedules.borrow_mut();

    // This may be called in a function above. It does not matter as the calculation is not heavy.
    // Calculating again here is safer.
    schedules.compute_height_for_min_discretization(height);

    for (index, activities) in schedules
        .entities_to_show
        .iter()
        .enumerate()
        .map(|(index, entity)| (index, entity.activities()))
    {
        for activity in activities
            .iter()
            .filter(|activity| activity.insertion_interval().is_some())
        {
            let insertion_interval = activity
                .insertion_interval()
                .expect("Invalid insertion interval ! No filtering was done or it did not work.");
            let height_begin = insertion_interval.beginning().n_times_min_discretization() as f64
                * schedules.height_per_min_discretization;
            let heigh_to_paint = insertion_interval.duration().n_times_min_discretization() as f64
                * schedules.height_per_min_discretization;

            let width_begin = index as f64 * schedules.width_per_schedule;

            let color = activity.color();
            c.set_source_rgb(color.red, color.green, color.blue);

            const SPACE_BELOW_ACTIVITY: f64 = 1.5;
            c.rectangle(
                width_begin,
                height_begin,
                schedules.width_per_schedule,
                heigh_to_paint - SPACE_BELOW_ACTIVITY,
            );
            c.fill();

            // Compute offset to place text
            let size_of_text = c.text_extents(activity.name());
            let x_offset = (schedules.width_per_schedule - size_of_text.width) / 2.0;
            let y_offset = (heigh_to_paint + size_of_text.height) / 2.0;
            c.move_to(width_begin + x_offset, height_begin + y_offset);

            c.set_font_size(ACTIVITY_NAME_FONT_SIZE);
            let foreground = foreground_color_depending_on_background_color(*activity.color());
            c.set_source_rgb(foreground.red, foreground.green, foreground.blue);
            c.show_text(activity.name());
        }
    }
}

fn foreground_color_depending_on_background_color(bg: Rgba) -> Rgba {
    // Magic values from
    // https://stackoverflow.com/questions/3942878/how-to-decide-font-color-in-white-or-black-depending-on-background-color
    if (bg.red * 0.299 + bg.green * 0.587 + bg.blue * 0.114) > 0.73 {
        const WHITE: Rgba = Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        };
        WHITE
    } else {
        const BLACK: Rgba = Rgba {
            red: 0.9,
            green: 0.9,
            blue: 0.9,
            alpha: 1.0,
        };
        BLACK
    }
}

fn draw_possible_insertions_background(
    c: &cairo::Context,
    height: f64,
    schedules: &Rc<RefCell<Schedules>>,
) {
    let mut schedules = schedules.borrow_mut();

    // This may be called in a function above. It does not matter as the calculation is not heavy.
    // Calculating again here is safer.
    schedules.compute_height_for_min_discretization(height);
    if let Some(possible_insertion_times) = &schedules.possible_activity_insertion_times {
        let colors_associated_to_beginnings = costs_to_rgb(&possible_insertion_times);

        for (index, _entity) in
            schedules
                .entities_to_show
                .iter()
                .enumerate()
                .filter(|(_index, entity)| {
                    schedules
                        .activity_insertion_concerned_entities
                        .contains(entity.name())
                })
        {
            for (beginning, color) in &colors_associated_to_beginnings {
                let height_begin = beginning.n_times_min_discretization() as f64
                    * schedules.height_per_min_discretization;
                let heigh_to_paint = schedules.height_per_min_discretization;
                c.set_source_rgb(color.red, color.green, color.blue);
                c.rectangle(
                    index as f64 * schedules.width_per_schedule,
                    height_begin,
                    schedules.width_per_schedule,
                    heigh_to_paint,
                );
                c.fill();
            }
        }
    }
}

fn draw_time_tooltip(c: &cairo::Context, schedules: Rc<RefCell<Schedules>>) {
    let mut schedules = schedules.borrow_mut();
    if let Some(time_tooltip_to_draw) = &schedules.time_tooltip_to_draw {
        let x = time_tooltip_to_draw.x_cursor;
        let y = time_tooltip_to_draw.y_cursor - TIME_TOOLTIP_HEIGHT as f64;

        // Draw rectangle
        c.set_source_rgb(
            TIME_TOOLTIP_BACKGROUND_RGB,
            TIME_TOOLTIP_BACKGROUND_RGB,
            TIME_TOOLTIP_BACKGROUND_RGB,
        );
        c.rectangle(x, y, TIME_TOOLTIP_WIDTH as f64, TIME_TOOLTIP_HEIGHT as f64);
        c.fill();

        // Write time
        c.move_to(x + TIME_TOOLTIP_X_OFFSET, y + TIME_TOOLTIP_Y_OFFSET);
        c.set_font_size(TIME_TOOLTIP_FONT_SIZE);
        c.set_source_rgb(
            TIME_TOOLTIP_FONT_RGB,
            TIME_TOOLTIP_FONT_RGB,
            TIME_TOOLTIP_FONT_RGB,
        );
        c.show_text(&time_tooltip_to_draw.time.to_string());

        // Reset
        schedules.time_tooltip_to_draw = None;
    }
}
