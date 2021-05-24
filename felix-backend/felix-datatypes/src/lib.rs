use std::collections::{HashMap, HashSet};

mod time;
mod insertion_cost;
mod computation_structs;

pub use time::{Time, TimeInterval, WorkHourInMinutes, MIN_TIME_DISCRETIZATION, MIN_TIME_DISCRETIZATION_MINUTES};
pub use insertion_cost::{insertion_cost_minutes::InsertionCostsMinutes, InsertionCost};
pub use computation_structs::WorkHoursAndActivityDurationsSorted;

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
pub type ActivityBeginningsGivenDurationMinutes =
    HashMap<ActivityDurationMinutes, HashSet<ActivityBeginningMinutes>>;

pub type ActivityDurationMinutes = u16;
pub type Cost = usize;
pub type ActivityBeginningMinutes = u16;
