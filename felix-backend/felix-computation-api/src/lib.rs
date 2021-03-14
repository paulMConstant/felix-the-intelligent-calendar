extern crate itertools;

pub mod compute_insertion_costs;
pub mod find_possible_beginnings;
pub mod structs;

pub use compute_insertion_costs::compute_insertion_costs;
pub use find_possible_beginnings::find_possible_beginnings;

pub const MIN_TIME_DISCRETIZATION_MINUTES: u16 = 5;
