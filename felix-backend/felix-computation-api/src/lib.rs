#![feature(map_first_last)]

extern crate itertools;
extern crate num_cpus;

pub mod autoinsert;
pub mod compute_insertion_costs;
pub mod find_possible_beginnings;
pub mod structs;

pub use autoinsert::autoinsert;
pub use compute_insertion_costs::compute_insertion_costs;
pub use find_possible_beginnings::find_possible_beginnings;
