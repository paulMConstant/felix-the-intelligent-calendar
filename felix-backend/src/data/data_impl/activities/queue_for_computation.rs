use crate::data::{Activity, Data};

use std::collections::HashSet;

/// Functions to trigger & update activity insertion computation
impl Data {
    /// Queues up every entity to compute the possible beginnings of their entities.
    /// Must be called on startup if data is not created from stratch (i.e. instantiated without
    /// new(), with serde for example).
    pub fn queue_every_activity_for_beginning_computation(&mut self) {
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
            .filter(|entity| entity.custom_work_hours().is_empty())
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
        let work_hours_and_activity_durations =
            self.work_hours_and_activity_durations_from_entities(&entities);
        let activities_to_invalidate = self.activities_of_entities(&entities);

        self.activities.trigger_update_possible_activity_beginnings(
            work_hours_and_activity_durations,
            activities_to_invalidate,
        );
    }

    /// Given a vector of entities, outputs their activities without duplicates.
    ///
    /// # Panics
    ///
    /// Panics if one entity name is empty.
    fn activities_of_entities(&self, entities: &[String]) -> HashSet<Activity> {
        entities
            .iter()
            .flat_map(|entity| {
                self.activities_of(entity)
                    .expect(&format!("Could not find activities of '{}'", entity))
            })
            .cloned()
            .collect::<HashSet<_>>()
    }
}
