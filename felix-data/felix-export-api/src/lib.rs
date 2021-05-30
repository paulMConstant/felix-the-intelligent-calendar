mod lines_from_activities;
mod line;
mod pdf_sizes;

use line::{Line, LineSplits};
use lines_from_activities::extract_lines_from_activities;
use pdf_sizes::{PdfSize, PdfSizes};

use felix_collections::Activity;
use std::path::PathBuf;

const FONT_RGB: f64 = 0.0;
const LINE_WIDTH: f64 = 2.0;

const INCH_TO_POINT_MULTIPLIER: f64 = 72.0;
const A4_HEIGHT_IN_POINTS: f64 = 11.693 * INCH_TO_POINT_MULTIPLIER;
const A4_WIDTH_IN_POINTS: f64 = 8.268 * INCH_TO_POINT_MULTIPLIER;

pub fn generate_pdf(entity: String, activities: Vec<Activity>, output_dir: PathBuf) {
    // 1. Format data
    let pdf = Pdf::new(entity, activities);

    // 2. Print data
    pdf.render(output_dir);
}

struct Pdf {
    title: String,
    title_x_offset: f64,
    title_height: f64,

    lines: Vec<Line>,
    /// For each line, contains the string index of the line breaks.
    line_breaks: Vec<LineSplits>,

    size: PdfSize,
}

impl Pdf {
    pub fn new(entity: String, activities: Vec<Activity>) -> Pdf {
        let mut pdf = Pdf {
            title: entity,
            lines: extract_lines_from_activities(activities),

            title_x_offset: 0.0,
            title_height: 0.0,

            line_breaks: Vec::new(),
            size: PdfSizes::Tiny.value(),
        };

        pdf.select_biggest_size_which_fits_on_page();

        pdf
    }

    fn select_biggest_size_which_fits_on_page(&mut self) {
        let mut sizes = PdfSizes::new();

        // Find the first size which fits on the page
        while let Some(next_size) = sizes.next() {
            sizes = next_size;
            self.compute_dimensions_based_on_size(sizes.value());

            if self.compute_total_height() <= A4_HEIGHT_IN_POINTS {
                break;
            }
        }
        // TODO_LATER if no size fits, add on page and repeat ?
    }

    fn compute_total_height(&self) -> f64 {
        let tmp_dir = tempfile::tempdir().expect("Could not create tempdir");

        self.render(tmp_dir.path().to_path_buf()) + self.size.top_bottom_margin
    }

    /// Computes data such as title height, line height, line breaks before rendering.
    fn compute_dimensions_based_on_size(&mut self, size: PdfSize) {
        self.size = size;
        self.compute_line_breaks();
        self.compute_title_position();
    }

    /// For each line, adds the string indexes of where the line should break.
    fn compute_line_breaks(&mut self) {
        self.line_breaks = vec![Vec::new(); self.lines.len()];

        let tmp_dir = tempfile::tempdir().expect("Could not create tempdir");
        let tmp_file = tmp_dir.path().join("tmp.pdf");

        let surface = cairo::PdfSurface::new(
            A4_WIDTH_IN_POINTS,
            A4_HEIGHT_IN_POINTS,
            tmp_file
        )
        .expect("Could not create pdf surface");
        let c = cairo::Context::new(&surface);
        c.set_font_size(self.size.line_font_size);
        c.set_line_width(LINE_WIDTH);

        // Break lines which are too long
        let margins = self.size.left_right_margin * 2.0;
        for (line_index, line) in self.lines.iter().enumerate().filter(|(_index, line)| {
            let line_length = c.text_extents(&line.print()).width;
            // Line too long if it exceeds the width of the PDF
            line_length + margins > A4_WIDTH_IN_POINTS
        }) {
            let length_of_time_interval = c.text_extents(&line.timestamp).width;
            let width_per_line = A4_WIDTH_IN_POINTS - margins - length_of_time_interval;

            let mut words_fit_in_line = Vec::new();

            let mut line_length = 0;
            for word in line.activity_name_and_participants.split(' ') {
                words_fit_in_line.push(word);

                let length_of_line = c.text_extents(&words_fit_in_line.join(" ")).width;
                if length_of_line > width_per_line {
                    // Mark a break (last word excluded)
                    self.line_breaks[line_index].push(line_length);
                    // Start a new line with the last word
                    // Keep only the last word
                    // Swap_remove puts the last element in first place
                    words_fit_in_line.swap_remove(0);
                    words_fit_in_line.truncate(1);
                }
                line_length += word.len() + 1;
            }
        }
    }

    /// Computes :
    /// * title x offset
    /// * title height
    fn compute_title_position(&mut self) {
        let tmp_dir = tempfile::tempdir().expect("Could not create tempdir");
        let tmp_file = tmp_dir.path().join("tmp.pdf");

        let surface = cairo::PdfSurface::new(
            A4_WIDTH_IN_POINTS,
            A4_HEIGHT_IN_POINTS,
            tmp_file
        )
        .expect("Could not create pdf surface");
        let c = cairo::Context::new(&surface);
        c.set_font_size(self.size.title_font_size);
        c.set_line_width(LINE_WIDTH);

        let title_extents = c.text_extents(&self.title);
        self.title_x_offset = A4_WIDTH_IN_POINTS / 2.0 - title_extents.width / 2.0;
        self.title_height = title_extents.height;
    }

    /// Generates the pdf file with the parameters which have been calculated when creating the
    /// object.
    ///
    /// # Returns
    ///
    /// The total height of the surface.
    pub fn render(&self, mut output_dir: PathBuf) -> f64 {
        let filename = self.title.replace(' ', "_");
        output_dir.push(filename + ".pdf");

        let surface = cairo::PdfSurface::new(
            A4_WIDTH_IN_POINTS,
            A4_HEIGHT_IN_POINTS,
            output_dir
        )
        .expect("Could not create pdf surface");

        let c = cairo::Context::new(&surface);

        c.set_source_rgb(FONT_RGB, FONT_RGB, FONT_RGB); // Black
        c.set_font_size(self.size.title_font_size);
        c.set_line_width(LINE_WIDTH);
        let mut current_y = self.size.top_bottom_margin;

        c.move_to(self.title_x_offset, current_y);
        c.show_text(&self.title);

        c.set_font_size(self.size.line_font_size);
        current_y += self.title_height;
        for (line_index, line) in self.lines.iter().enumerate() {
            current_y += self.size.line_spacing;
            c.move_to(self.size.left_right_margin, current_y);
            c.show_text(&line.timestamp);

            let next_line_start_x = c.get_current_point().0;

            let mut last_line_break = 0;
            for &next_line_break in &self.line_breaks[line_index] {
                c.show_text(&line.activity_name_and_participants[last_line_break..next_line_break]);

                last_line_break = next_line_break;
                current_y += self.size.inter_line_break_spacing;

                c.move_to(next_line_start_x, current_y);
            }

            c.show_text(&line.activity_name_and_participants[last_line_break..]);
        }
        current_y
    }
}

#[cfg(test)]
mod tests;
