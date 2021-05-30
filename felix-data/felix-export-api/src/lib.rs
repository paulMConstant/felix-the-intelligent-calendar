use felix_collections::Activity;

const FONT_RGB: f64 = 0.0;
const LINE_WIDTH: f64 = 2.0;

const INCH_TO_POINT_MULTIPLIER: f64 = 72.0;
const A4_HEIGHT_IN_POINTS: f64 = 11.693 * INCH_TO_POINT_MULTIPLIER;
const A4_WIDTH_IN_POINTS: f64 = 8.268 * INCH_TO_POINT_MULTIPLIER;

type LineIndex = usize;
type LineSplits = Vec<LineIndex>;

pub struct Line {
    pub timestamp: String,
    pub activity_name_and_participants: String,
}

impl Line {
    pub fn print(&self) -> String {
        self.timestamp.clone() + &self.activity_name_and_participants
    }
}

struct PdfSize {
    pub title_font_size: f64,
    pub inter_line_break_spacing: f64,
    pub line_spacing: f64,
    pub line_font_size: f64,
    pub left_right_margin: f64,
    pub top_bottom_margin: f64,
}

enum PdfSizes {
    Uninitialized,
    Large,
    Big,
    Medium,
    Small,
    Tiny,
}

impl PdfSizes {
    const fn new() -> PdfSizes {
        PdfSizes::Uninitialized
    }

    fn value(&self) -> PdfSize {
        match *self {
            Self::Uninitialized => panic!("Pdf size is uninitialized"),
            Self::Large => PdfSize {
                title_font_size: 32.0,
                inter_line_break_spacing: 25.0,
                line_spacing: 35.0,
                line_font_size: 20.0,
                left_right_margin: 50.0,
                top_bottom_margin: 60.0,
            },
            Self::Big => PdfSize {
                title_font_size: 30.0,
                inter_line_break_spacing: 25.0,
                line_spacing: 30.0,
                line_font_size: 18.0,
                left_right_margin: 40.0,
                top_bottom_margin: 55.0,
            },
            Self::Medium => PdfSize {
                title_font_size: 28.0,
                inter_line_break_spacing: 23.0,
                line_spacing: 30.0,
                line_font_size: 17.0,
                left_right_margin: 40.0,
                top_bottom_margin: 50.0,
            },
            Self::Small => PdfSize {
                title_font_size: 26.0,
                inter_line_break_spacing: 20.0,
                line_spacing: 25.0,
                line_font_size: 16.0,
                left_right_margin: 35.0,
                top_bottom_margin: 45.0,
            },
            Self::Tiny => PdfSize {
                title_font_size: 24.0,
                inter_line_break_spacing: 18.0,
                line_spacing: 25.0,
                line_font_size: 14.0,
                left_right_margin: 30.0,
                top_bottom_margin: 40.0,
            },
        }
    }

    /// Returns the next smaller size or none if there is no smaller size.
    const fn next(&self) -> Option<Self> {
        match *self {
            Self::Uninitialized => Some(Self::Large),
            Self::Large => Some(Self::Big),
            Self::Big => Some(Self::Medium),
            Self::Medium => Some(Self::Small),
            Self::Small => Some(Self::Tiny),
            Self::Tiny => None,
        }
    }
}

fn extract_lines_from_activities(mut activities: Vec<Activity>) -> Vec<Line> {
    // Do not mention not inserted activities
    activities = activities
        .into_iter()
        .filter(|activity| activity.insertion_interval().is_some())
        .collect();

    // Earlier activities go first
    activities.sort_by(|a, b| {
        a.insertion_interval()
            .expect("Exporting uninserted activities")
            .cmp(
                &b.insertion_interval()
                    .expect("Exporting uninserted activities"),
            )
    });

    activities
        .iter()
        .map(|activity| Line {
            timestamp: activity
                .insertion_interval()
                .expect("Exporting uninserted activity")
                .to_string()
                + " : ",
            activity_name_and_participants: activity.name()
                + " ("
                + &activity.entities_sorted().join(", ")
                + ")",
        })
        .collect()
}

pub struct Pdf {
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
        self.render("") + self.size.top_bottom_margin
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

        let surface = cairo::PdfSurface::new(
            A4_WIDTH_IN_POINTS,
            A4_HEIGHT_IN_POINTS,
            "/home/paul/test.pdf",
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
        let surface = cairo::PdfSurface::new(
            A4_WIDTH_IN_POINTS,
            A4_HEIGHT_IN_POINTS,
            "/home/paul/test.pdf",
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
    pub fn render(&self, output_dir: &str) -> f64 {
        let surface = cairo::PdfSurface::new(
            A4_WIDTH_IN_POINTS,
            A4_HEIGHT_IN_POINTS,
            "/home/paul/test.pdf",
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

pub fn generate_pdf(entity: String, activities: Vec<Activity>, output_dir: &str) {
    // 1. Format data
    let pdf = Pdf::new(entity, activities);

    // 2. Print data
    pdf.render(output_dir);
}

#[cfg(test)]
mod tests;
