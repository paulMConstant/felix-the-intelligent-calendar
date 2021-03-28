use std::collections::{HashMap, HashSet};

mod activity_computation_static_data;
pub mod autoinsertion;
mod insertion_costs;
mod sum_and_duration_indexes;
mod work_hour_in_minutes;

pub use activity_computation_static_data::ActivityComputationStaticData;
pub use insertion_costs::InsertionCostsMinutes;
pub use sum_and_duration_indexes::SumAndDurationIndexes;
pub use work_hour_in_minutes::WorkHourInMinutes;

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
pub type ActivityBeginningsGivenDurationMinutes =
    HashMap<ActivityDurationMinutes, HashSet<ActivityBeginningMinutes>>;

pub type ActivityDurationMinutes = u16;
pub type Cost = usize;
pub type ActivityBeginningMinutes = u16;
