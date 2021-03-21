use crate::data::{
    computation_structs::WorkHoursAndActivityDurationsSorted, Activity, ActivityId, ThreadPool,
    Time,
};

use felix_computation_api::{find_possible_beginnings, MIN_TIME_DISCRETIZATION_MINUTES};

use super::activity_beginnings_given_duration::{
    new_activity_beginnings_given_duration, ActivityBeginningsGivenDuration,
};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

type WorkHoursAndActivityDurationsSortedCache =
    HashMap<WorkHoursAndActivityDurationsSorted, ActivityBeginningsGivenDuration>;

/// Keeps track of which activity possible beginnings are out of date
/// and handles the computation.
///
/// This class is NOT thread-safe, it only runs the computations in a separate thread pool.
#[derive(Debug, Serialize, Deserialize)]
pub struct PossibleBeginningsUpdater {
    possible_beginnings_up_to_date: HashMap<ActivityId, bool>,
    // Prototype design pattern
    computation_cache: Arc<Mutex<WorkHoursAndActivityDurationsSortedCache>>,
    #[serde(skip)]
    thread_pool: Rc<ThreadPool>,
}

impl PossibleBeginningsUpdater {
    pub fn new() -> PossibleBeginningsUpdater {
        PossibleBeginningsUpdater {
            possible_beginnings_up_to_date: HashMap::new(),
            computation_cache: Arc::new(
                Mutex::new(WorkHoursAndActivityDurationsSortedCache::new()),
            ),
            thread_pool: Rc::new(ThreadPool::new()),
        }
    }

    /// Informs the updater that a new activity has been added.
    pub fn notify_new_activity(&mut self, id: ActivityId) {
        self.possible_beginnings_up_to_date.insert(id, true);
    }

    /// Informs the updater that an activity has been deleted.
    pub fn notify_activity_removed(&mut self, id: ActivityId) {
        self.possible_beginnings_up_to_date.remove(&id);
    }

    /// Returns true if the activity possible beginnings are up to date.
    #[must_use]
    pub fn activity_beginnings_are_up_to_date(&self, id: &ActivityId) -> bool {
        *self
            .possible_beginnings_up_to_date
            .get(id)
            .expect("Querying invalid activity ID !")
    }

    /// For the given work hours and activity durations, computes the possible activity beginnings.
    /// Invalidates the concerned activities.
    pub fn queue_work_hours_and_activity_durations(
        &mut self,
        work_hours_and_activity_durations: &[WorkHoursAndActivityDurationsSorted],
        out_of_date_activities: HashSet<ActivityId>,
    ) {
        if out_of_date_activities.is_empty() {
            // No activities are concerned - return
            return;
        }

        for id in out_of_date_activities {
            self.possible_beginnings_up_to_date.insert(id, false);
        }

        for key in work_hours_and_activity_durations {
            if !self.computation_cache.lock().unwrap().contains_key(&key) {
                let computation_cache = &self.computation_cache;

                // Launch the computation in a separate thread
                self.thread_pool.install(|| {
                    let activity_beginnings_given_duration_minutes = find_possible_beginnings(
                        &key.work_hours_in_minutes(),
                        &key.activity_durations_in_minutes(),
                        MIN_TIME_DISCRETIZATION_MINUTES.into(),
                    );

                    let result = new_activity_beginnings_given_duration(
                        activity_beginnings_given_duration_minutes,
                    );

                    computation_cache
                        .lock()
                        .unwrap()
                        .insert(key.clone(), result);
                });
            }
        }
    }

    /// Fuses the possible beginnings given every work\_hour\_and\_activity\_duration key.
    /// If the data is not yet available, returns None.
    /// If the data is available, sets 'possible_beginnings_up_to_date' to true.
    #[must_use]
    pub fn poll_and_fuse_possible_beginnings(
        &mut self,
        schedules_of_participants: &[WorkHoursAndActivityDurationsSorted],
        activity: &Activity,
    ) -> Option<HashSet<Time>> {
        let computation_cache = self.computation_cache.lock().unwrap();
        let activity_duration = activity.duration();

        // Fetch possible beginnings
        let maybe_all_possible_beginnings: Option<Vec<_>> =
            schedules_of_participants
                .iter()
                .map(|work_hours_and_activity_durations| {
                    if let Some(beginnings_given_duration) =
                        computation_cache.get(work_hours_and_activity_durations)
                    {
                        // Computation result is there.
                        // Fetch only the possible beginnings for the specified duration.
                        Some(beginnings_given_duration.get(&activity_duration).expect(
                            "Activity duration not in durations calculated for participants",
                        ))
                    } else {
                        // Computation result is missing
                        None
                    }
                })
                .collect();

        // Intersect all possible beginnings
        if let Some(mut all_possible_beginnings) = maybe_all_possible_beginnings {
            // Sort sets by ascending size so that fewer checks are done for intersections
            all_possible_beginnings.sort_by_key(|a| a.len());

            let first_set = all_possible_beginnings[0];
            let intersection = first_set
                .iter()
                .filter(|k| all_possible_beginnings[1..].iter().all(|s| s.contains(k)));

            self.possible_beginnings_up_to_date
                .insert(activity.id(), true);
            Some(intersection.copied().collect())
        } else {
            // At least one computation result was missing
            None
        }
    }
}
