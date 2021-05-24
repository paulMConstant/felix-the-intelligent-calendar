use std::collections::{HashMap, HashSet};

mod computation_structs;
mod insertion_cost;
mod rgba;
mod time;

pub use computation_structs::WorkHoursAndActivityDurationsSorted;
pub use insertion_cost::{insertion_cost_minutes::InsertionCostsMinutes, InsertionCost};
pub use rgba::Rgba;
pub use time::{
    Time, TimeInterval, WorkHourInMinutes, MIN_TIME_DISCRETIZATION, MIN_TIME_DISCRETIZATION_MINUTES,
};

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
pub type ActivityBeginningsGivenDurationMinutes =
    HashMap<ActivityDurationMinutes, HashSet<ActivityBeginningMinutes>>;

pub type ActivityInsertionCosts = Option<Vec<InsertionCost>>;
pub type ActivityId = usize;
pub type ActivityDurationMinutes = u16;
pub type ActivityBeginningMinutes = u16;
pub type Cost = usize;
