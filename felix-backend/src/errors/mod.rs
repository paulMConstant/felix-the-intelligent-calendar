pub mod add_entity_to_inserted_activity_invalid_spot;
pub mod already_in;
pub mod change_work_hours_while_activity_inserted;
pub mod does_not_exist;
pub mod duration_too_short;
pub mod empty_name;
pub mod interval_overlaps;
pub mod invalid_insertion;
pub mod invalid_interval;
pub mod name_taken;
pub mod not_enough_time;
pub mod not_in;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
