use crate::Time;
use crate::{computation_structs::WorkHoursAndActivityDurationsSorted, Activity, Data};

use std::collections::{HashMap, HashSet};

/// Functions to trigger & update activity insertion computation
impl Data {
    /// Initializes activity computation in a separate thread.
    /// Must be called on startup.
    pub fn init_computation_module(&mut self) {
        self.activities.run_separate_thread_computation();
        self.queue_every_activity_for_beginning_computation();
    }

    /// Queues up every entity to compute the possible beginnings of their entities.
    fn queue_every_activity_for_beginning_computation(&mut self) {
        let entity_names = self
            .entities_sorted()
            .iter()
            .map(|entity| entity.name())
            .collect::<Vec<_>>();

        self.queue_entities(entity_names);
    }

    /// Starts the computation of the possible beginnings of activities of entities whose work
    /// hours were modified.
    pub(crate) fn queue_entities_on_global_work_hour_change(&mut self) {
        let entities_to_queue = self
            .entities_sorted()
            .iter()
            .filter(|entity| {
                self.custom_work_hours_of(entity.name())
                    .unwrap_or_else(|_| {
                        panic!(
                            "The custom work hours of {} are not registered",
                            entity.name()
                        )
                    })
                    .is_empty()
            })
            .map(|entity| entity.name())
            .collect::<Vec<_>>();
        self.queue_entities(entities_to_queue);
    }

    /// Starts the computation of the possible beginnings of the given activity.
    pub(crate) fn queue_activity_participants(&mut self, activity: Activity) {
        // Queue every entity of the activity
        self.queue_entities(activity.entities_sorted());
    }

    /// Starts the computation of the possible beginnings of the activities of the given entities.
    ///
    /// # Panics
    ///
    /// Panics if one of the entities does not exist.
    pub(crate) fn queue_entities(&mut self, entities: Vec<String>) {
        // For each activity
        // Gather all of its participants
        let activities_to_update = self.activities_of_entities_with_non_empty_duration(&entities);

        // Compute their schedule
        let schedules_of_all_participants =
            self.schedules_of_entities_in_activities(&activities_to_update);

        // Set it into the activity
        self.update_schedules_of_participants_of_activities(
            &activities_to_update,
            &schedules_of_all_participants,
        );

        // Then update insertion costs
        self.activities.trigger_update_possible_activity_beginnings(
            schedules_of_all_participants
                .values()
                .cloned()
                .collect::<Vec<_>>(),
        );
    }

    /// Returns all activities in which at least one entity in the given slice participates.
    /// Does not return activities with empty duration (0 minutes, 0 hours).
    fn activities_of_entities_with_non_empty_duration(
        &self,
        entities: &[String],
    ) -> HashSet<Activity> {
        entities
            .iter()
            .flat_map(|entity| {
                self.activities_of(entity)
                    .expect("Entity name is empty - this is a bug")
            })
            .filter(|activity| activity.duration() > Time::new(0, 0))
            .collect()
    }

    /// Returns a (Entity, Schedule) map with an entry for each entity taking part in at least one
    /// activity.
    fn schedules_of_entities_in_activities(
        &self,
        activities: &HashSet<Activity>,
    ) -> HashMap<String, WorkHoursAndActivityDurationsSorted> {
        let entities = activities
            .iter()
            .flat_map(|activity| activity.entities_sorted())
            .collect::<HashSet<_>>();

        entities
            .iter()
            .map(|entity| {
                (
                    entity.clone(),
                    self.work_hours_and_activity_durations_from_entity(entity),
                )
            })
            .collect()
    }

    /// Given a list of activities and the schedules of all participants
    /// (all activities included),
    /// fills each activity with the schedules of their participants.
    fn update_schedules_of_participants_of_activities(
        &mut self,
        activities: &HashSet<Activity>,
        schedules: &HashMap<String, WorkHoursAndActivityDurationsSorted>,
    ) {
        for activity in activities {
            let schedules_of_participants_of_this_activity = activity
                .entities_sorted()
                .iter()
                .map(|entity| schedules[entity].clone())
                .collect::<Vec<_>>();

            self.activities
                .update_schedules_of_participants_of_activity(
                    activity.id(),
                    schedules_of_participants_of_this_activity,
                );
        }
    }
}
