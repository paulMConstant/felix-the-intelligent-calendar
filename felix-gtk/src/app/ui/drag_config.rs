use felix_backend::data::ActivityID;

pub const DRAG_WIDTH: i32 = 300;
pub const DRAG_HEIGHT: i32 = 30;

pub const DRAG_BACKGROUND_RGB: f64 = 0.9;
pub const DRAG_FONT_RGB: f64 = 0.0;
pub const DRAG_FONT_SIZE: i32 = 14;

pub const DRAG_TEXT_Y_OFFSET: f64 = 20.0;

pub const DRAG_TYPE: &str = "ACTIVITY";
pub const DRAG_DATA_FORMAT: usize = std::mem::size_of::<ActivityID>();
