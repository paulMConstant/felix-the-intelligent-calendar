mod activity_beginnings_given_duration;
mod insertion_costs_updater;
mod possible_beginnings_pool;
mod thread_pool;

use crate::data::{
    computation_structs::WorkHoursAndActivityDurationsSorted, Activity, ActivityId,
    ActivityInsertionCosts,
};

use insertion_costs_updater::InsertionCostsUpdater;
use possible_beginnings_pool::{PossibleBeginningsComputationPool, PossibleBeginningsPool};

use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use std_semaphore::Semaphore;

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
        &mut self,
        work_hours_and_activity_durations: Vec<WorkHoursAndActivityDurationsSorted>,
        concerned_activities: HashSet<Activity>,
        computation_done_semaphore: Arc<Semaphore>,
    ) {
        let n_results_to_wait_for = work_hours_and_activity_durations.len();
        self.insertion_costs_updater.invalidate_activities(
            concerned_activities
                .iter()
                .map(|activity| activity.id())
                .collect(),
        );

        self.possible_beginnings_pool
            .queue_work_hours_and_activity_durations(
                work_hours_and_activity_durations,
                computation_done_semaphore.clone(),
            );

        // Wait for the possible beginnings pool to end computation then fuse the results and
        // compute insertion scores.
        std::thread::spawn(move || {
            // Wait for every computation to be done
            computation_done_semaphore.acquire();
            //self.possible_beginnings_pool.poll_and_fuse_possible_beginnings();
            // Every beginning has been computed
            // Fuse them
            // Build computation data
        });
    }
}

impl Default for SeparateThreadActivityComputation {
    fn default() -> Self {
        Self::new()
    }
}
