//! Custom errors following Rust By Example guideline
//! https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
pub mod already_in;
pub mod does_not_exist;
pub mod duration_too_short;
pub mod empty_name;
pub mod interval_overlaps;
pub mod name_taken;
pub mod not_enough_time;
pub mod not_in;
pub mod invalid_interval;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
