use std::collections::{HashMap, HashSet};

mod activity_computation_static_data;
mod sum_and_duration_indexes;
mod work_hour_in_minutes;

pub use activity_computation_static_data::ActivityComputationStaticData;
pub use sum_and_duration_indexes::SumAndDurationIndexes;
pub use work_hour_in_minutes::WorkHourInMinutes;

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
pub type ActivityBeginningsGivenDurationMinutes = HashMap<u16, HashSet<u16>>;

pub type ActivityInsertionBeginningMinutes = Option<u16>;
