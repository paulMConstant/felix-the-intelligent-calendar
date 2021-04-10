mod activity_beginnings_given_duration;
mod insertion_costs_updater;
mod possible_beginnings_pool;
mod thread_pool;

use crate::data::{
    computation_structs::WorkHoursAndActivityDurationsSorted, ActivityId, ActivityInsertionCosts,
};

use insertion_costs_updater::InsertionCostsUpdater;
use possible_beginnings_pool::{PossibleBeginningsComputationPool, PossibleBeginningsPool};

use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// Handles the computation of possible activity beginnings asynchronously.
#[derive(Debug)]
pub struct SeparateThreadActivityComputation {
    possible_beginnings_pool: PossibleBeginningsPool,
    insertion_costs_updater: InsertionCostsUpdater,
}

impl SeparateThreadActivityComputation {
    #[must_use]
    pub fn new() -> Self {
        let possible_beginnings_computation_pool =
            Arc::new(Mutex::new(PossibleBeginningsComputationPool::new()));
        let thread_pool = Rc::new(thread_pool::ThreadPool::new());

        let possible_beginnings_pool = PossibleBeginningsPool::new(
            possible_beginnings_computation_pool.clone(),
            thread_pool.clone(),
        );

        let insertion_costs_updater =
            InsertionCostsUpdater::new(possible_beginnings_computation_pool, thread_pool);

        SeparateThreadActivityComputation {
            possible_beginnings_pool,
            insertion_costs_updater,
        }
    }

    /// Informs the pool that a new activity has been added.
    pub fn register_new_activity(
        &mut self,
        id: ActivityId,
        insertion_costs: Arc<Mutex<ActivityInsertionCosts>>,
    ) {
        self.insertion_costs_updater
            .activities_insertion_costs
            .insert(id, insertion_costs);
    }

    /// Informs the pool that an activity has been deleted.
    pub fn register_activity_removed(&mut self, id: ActivityId) {
        self.insertion_costs_updater
            .activities_insertion_costs
            .remove(&id);
    }

    /// Computes the possible beginnings of activities with given durations for the given work
    /// hours.
    /// Then, fills the insertion costs of concerned activities.
    pub fn queue_work_hours_and_activity_durations(
        &self,
        work_hours_and_activity_durations: Vec<WorkHoursAndActivityDurationsSorted>,
        concerned_activities: HashSet<ActivityId>,
    ) {
    }
}

impl Default for SeparateThreadActivityComputation {
    fn default() -> Self {
        Self::new()
    }
}
