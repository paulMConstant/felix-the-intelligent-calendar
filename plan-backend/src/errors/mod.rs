//! Custom errors following Rust By Example guideline
//! https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html 
pub mod does_not_exist;
pub mod name_taken;
pub mod not_enough_time;
pub mod interval_overlaps;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
