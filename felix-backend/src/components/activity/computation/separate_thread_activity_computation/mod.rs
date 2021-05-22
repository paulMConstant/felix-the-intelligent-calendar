mod activity_beginnings_given_duration;
mod computation_done_semaphore;
mod insertion_costs_updater;
mod thread_pool;

use crate::{computation_structs::WorkHoursAndActivityDurationsSorted, Activity};

use computation_done_semaphore::Semaphore;
use felix_computation_api::find_possible_beginnings;
//use insertion_costs_updater;
use thread_pool::ThreadPool;

use activity_beginnings_given_duration::{
    new_activity_beginnings_given_duration, ActivityBeginningsGivenDuration,
};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

type PossibleBeginningsPool =
    HashMap<WorkHoursAndActivityDurationsSorted, ActivityBeginningsGivenDuration>;

/// Handles the computation of possible activity beginnings asynchronously.
#[derive(Debug)]
pub struct SeparateThreadActivityComputation {
    thread_pool: Rc<ThreadPool>,
    possible_beginnings_pool: Arc<Mutex<PossibleBeginningsPool>>,
    computation_done_semaphore: Arc<Semaphore>,
}

impl SeparateThreadActivityComputation {
    #[must_use]
    pub fn new() -> Self {
        let thread_pool = Rc::new(ThreadPool::new());
        let possible_beginnings_pool = Arc::new(Mutex::new(PossibleBeginningsPool::new()));
        let computation_done_semaphore = Arc::new(Semaphore::new(1));

        SeparateThreadActivityComputation {
            thread_pool,
            possible_beginnings_pool,
            computation_done_semaphore,
        }
    }

    pub fn run_update_insertion_costs_thread(&self, activities: Arc<Mutex<Vec<Activity>>>) {
        let computation_done_semaphore = self.computation_done_semaphore.clone();
        let possible_beginnings_pool = self.possible_beginnings_pool.clone();

        self.thread_pool.spawn(move || {
            loop {
                // Wait until a result is up
                computation_done_semaphore.acquire();
                if !insertion_costs_updater::poll_and_fuse_possible_beginnings(
                    activities.clone(),
                    possible_beginnings_pool.clone(),
                ) {
                    break;
                }
            }
        });
    }

    /// Computes the possible beginnings of activities with given durations for the given work
    /// hours.
    /// Then, fills the insertion costs of concerned activities.
    pub fn queue_work_hours_and_activity_durations(
        &mut self,
        work_hours_and_activity_durations: Vec<WorkHoursAndActivityDurationsSorted>,
        activities: Arc<Mutex<Vec<Activity>>>,
    ) {
        invalidate_activities(activities);

        self.computation_done_semaphore
            .acquire_nonblocking(work_hours_and_activity_durations.len());

        self.launch_computation(work_hours_and_activity_durations);

        // Release once to let the insertion_cost_thread go once every computation is done
        self.computation_done_semaphore.release();
    }

    /// Computes and stores all possible beginnings for activities, not taking conflicts into
    /// account.
    /// This kind of computation is a variant of the backpack problem, in which you have to fit
    /// N items into N backpacks. Here, for one entity, we check how we can arrange its
    /// activities. The search is exhaustive.
    ///
    /// Storage is done via key-value pairs:
    ///     Key is a Vec of activity durations (e.g. 40 and 20 minutes)
    ///        and a Vec of Work Hours (e.g. 12:00 - 13:00))
    ///     Value is where the activities can go
    ///         (e.g. for a duration of 40 minutes, 12:00 and 12:20,
    ///               for a duration of 20 minutes, 12:00 and 12:40).
    fn launch_computation(
        &self,
        work_hours_and_activity_durations: Vec<WorkHoursAndActivityDurationsSorted>,
    ) {
        for key in work_hours_and_activity_durations {
            if !self
                .possible_beginnings_pool
                .lock()
                .unwrap()
                .contains_key(&key)
            {
                // Result not already computed
                // Launch the computation in a separate thread

                let pool = self.possible_beginnings_pool.clone();
                let computation_done_semaphore = self.computation_done_semaphore.clone();

                self.thread_pool.spawn(move || {
                    let activity_beginnings_given_duration_minutes = find_possible_beginnings(
                        &key.work_hours_in_minutes(),
                        &key.activity_durations_in_minutes(),
                    );

                    let result = new_activity_beginnings_given_duration(
                        activity_beginnings_given_duration_minutes,
                    );

                    // Make sure no thread has panicked with the lock
                    if let Ok(mut pool) = pool.lock() {
                        pool.insert(key.clone(), result);
                    }
                    computation_done_semaphore.release();
                });
            } else {
                self.computation_done_semaphore.release();
            }
        }
    }
}

/// Sets the possible insertion costs of the activities to None.
fn invalidate_activities(activities: Arc<Mutex<Vec<Activity>>>) {
    // TODO move this elsewhere
    for activity in activities
        .lock()
        .unwrap()
        .iter()
        .filter(|activity| activity.can_be_inserted())
    {
        // Invalidate current possible insertions
        *activity.computation_data.insertion_costs().lock().unwrap() = None;
    }
}

impl Default for SeparateThreadActivityComputation {
    fn default() -> Self {
        Self::new()
    }
}
