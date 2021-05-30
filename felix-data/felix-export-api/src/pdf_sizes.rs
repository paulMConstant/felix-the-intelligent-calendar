pub(crate) struct PdfSize {
    pub title_font_size: f64,
    pub inter_line_break_spacing: f64,
    pub line_spacing: f64,
    pub line_font_size: f64,
    pub left_right_margin: f64,
    pub top_bottom_margin: f64,
}

pub(crate) enum PdfSizes {
    Uninitialized,
    Large,
    Big,
    Medium,
    Small,
    Tiny,
}

impl PdfSizes {
    pub const fn new() -> PdfSizes {
        PdfSizes::Uninitialized
    }

    pub fn value(&self) -> PdfSize {
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
    pub const fn next(&self) -> Option<Self> {
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

