use felix_collections::Activity;

const FONT_RGB: f64 = 0.0;
const LINE_WIDTH: f64 = 2.0;

const MAX_TITLE_FONT_SIZE: f64 = 32.0;
const MIN_TITLE_FONT_SIZE: f64 = 18.0;

const MAX_LINE_FONT_SIZE: f64 = 16.0;
const MIN_LINE_FONT_SIZE: f64 = 10.0;

const MAX_SPACING_BETWEEN_LINES: f64 = 15.0;
const MIN_SPACING_BETWEEN_LINES: f64 = 5.0;

const MAX_MARGIN: f64 = 50.0;
const MIN_MARGIN: f64 = 10.0;

const INCH_TO_POINT_MULTIPLIER: f64 = 72.0;
const A4_HEIGHT_IN_POINTS: f64 = 11.693 * INCH_TO_POINT_MULTIPLIER;
const A4_WIDTH_IN_POINTS: f64 = 8.268 * INCH_TO_POINT_MULTIPLIER;

pub struct Pdf {
    pub title: String,
    pub title_x_offset: f64,
    pub title_y_offset: f64,
    pub title_font_size: f64,

    pub lines: Vec<String>,
    pub line_spacing: f64,
    pub line_font_size: f64,

    pub left_right_margin: f64,
    pub bottom_margin: f64,
}

impl Pdf {
    pub fn new(entity: String, activities: &[Activity]) -> Pdf {
        let title = entity;
        let title_x_offset = MAX_MARGIN;
        let title_y_offset = MAX_MARGIN;
        let title_font_size = MAX_TITLE_FONT_SIZE;

        // TODO init lines
        let lines = vec!["Activity1".to_string(), "Activity2".to_string()];
        let line_spacing = MAX_SPACING_BETWEEN_LINES + 30.0;
        let line_font_size = MAX_LINE_FONT_SIZE;

        let left_right_margin = MAX_MARGIN;
        let bottom_margin = MAX_MARGIN;

        let mut pdf = Pdf { 
            title,
            title_x_offset,
            title_y_offset,
            title_font_size,

            lines,
            line_spacing,
            line_font_size,

            left_right_margin,
            bottom_margin,
        };

        pdf.compute_lines_position();
        pdf.compute_title_position();
        pdf
    }

    /// Computes :
    /// * lines font size
    /// * spacing between lines
    /// * margins
    /// * Line breaks if necessary
    fn compute_lines_position(&mut self) {


    }

    /// Computes :
    /// * title x offset
    /// * title y offset
    /// * title font size
    fn compute_title_position(&mut self) {
        let surface = cairo::PdfSurface::new(A4_WIDTH_IN_POINTS, A4_HEIGHT_IN_POINTS, "/home/paul/test.pdf")
            .expect("Could not create pdf surface");
        let c = cairo::Context::new(&surface);
        c.set_font_size(self.title_font_size);
        c.set_line_width(LINE_WIDTH);
        
        let title_extents = c.text_extents(&self.title);
        self.title_x_offset = A4_WIDTH_IN_POINTS / 2.0 - title_extents.width / 2.0;
    }

    /// Generates the pdf file with the parameters which have been calculated when creating the
    /// object.
    pub fn generate_pdf_file(&self, output_dir: &str) {
        // TODO
        let surface = cairo::PdfSurface::new(A4_WIDTH_IN_POINTS,
                                             A4_HEIGHT_IN_POINTS,
                                             "/home/paul/test.pdf")
            .expect("Could not create pdf surface");

        let c = cairo::Context::new(&surface);

        c.set_source_rgb(FONT_RGB, FONT_RGB, FONT_RGB); // Black
        c.set_font_size(self.title_font_size);
        c.set_line_width(LINE_WIDTH);
        c.move_to(self.title_x_offset, self.title_y_offset);
        c.show_text(&self.title);

        c.set_font_size(self.line_font_size);
        let mut current_y = self.title_y_offset * 2.0;
        for line in &self.lines {
            c.move_to(self.left_right_margin, current_y);
            c.show_text(&line);
            current_y += self.line_spacing;
        }
    }
}

pub fn generate_pdf(entity: String, activities: &[Activity], output_dir: &str) {
    // 1. Format data
    let pdf = Pdf::new(entity, &activities);

    // 2. Print data
    pdf.generate_pdf_file(output_dir);
}

#[cfg(test)]
mod tests;
