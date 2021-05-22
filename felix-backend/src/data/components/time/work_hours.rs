use crate::{data::{EntityName, TimeInterval}, errors::Result};
use crate::errors::does_not_exist::DoesNotExist;
use super::work_intervals::WorkIntervals;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Contains work hours represented as time intervals.
/// Stays sorted by ascending order and prevents work intervals from overlapping.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorkHours {
    global_work_intervals: WorkIntervals,
    custom_work_intervals: HashMap<EntityName, WorkIntervals>,
}

impl WorkHours {
    /// Creates new work hours.
    #[must_use]
    pub fn new() -> WorkHours {
        WorkHours {
            global_work_intervals: WorkIntervals::new(),
            custom_work_intervals: HashMap::new(),
        }
    }

    /// Returns immutable reference to the work hours.
    #[must_use]
    pub fn work_intervals(&self) -> &Vec<TimeInterval> {
        &self.global_work_intervals.work_intervals()
    }

    /// Adds the given time interval to the work hours.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval overlaps with the current work intervals.
    pub fn add_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.global_work_intervals.add_work_interval(interval)
    }

    /// Removes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found.
    pub fn remove_work_interval(&mut self, interval: TimeInterval) -> Result<()> {
        self.global_work_intervals.remove_work_interval(interval)
    }

    /// Changes the given interval.
    ///
    /// # Errors
    ///
    /// Returns Err if the interval was not found or if the new interval overlaps with
    /// the work hours.
    pub fn update_work_interval(
        &mut self,
        old_interval: TimeInterval,
        new_interval: TimeInterval,
    ) -> Result<()> {
        self.global_work_intervals.update_work_interval(old_interval, new_interval)
    }

    /// Add empty custom work hours for the given entity.
    /// 
    /// # Panics
    ///
    /// Panics if the entity name already exists as a key.
    pub fn add_empty_custom_work_intervals_for(&mut self, entity_name: String) {
        assert!(self.custom_work_intervals.get(&entity_name).is_none(), "The custom work hours of {} are already registered", &entity_name);
        self.custom_work_intervals.insert(entity_name, WorkIntervals::new());
    }

    /// Updates the key for the custom work hours of an entity whose name changed.
    /// 
    /// # Panics
    ///
    /// Panics if the entity whose name changed is not found.
    pub fn rename_entity_for_custom_work_hours(&mut self, old_name: &str, new_name: String) {
        let custom_work_intervals = self
            .custom_work_intervals
            .remove(old_name)
            .unwrap_or_else(|| panic!("The custom work hours of {} are not registered", old_name));
        self.custom_work_intervals.insert(new_name, custom_work_intervals);
    }

    /// Unregisters the custom work hours of an entity. This should be done when an entity is
    /// removed.
    /// 
    /// # Panics
    ///
    /// Panics if the entity has no custom work hours (not even empty ones).
    pub fn remove_custom_work_hours_of(&mut self, entity_name: &str) {
        self.custom_work_intervals.remove(entity_name)
            .unwrap_or_else(|| panic!("The custom work hours of {} are not registered", entity_name));
    }

    /// Adds a work interval to the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist or if the work interval overlaps with another.
    pub fn add_custom_work_interval_for(
        &mut self,
        entity_name: &str,
        interval: TimeInterval,
    ) -> Result<()> {
        match self.custom_work_intervals.get_mut(entity_name) {
            None => Err(DoesNotExist::entity_does_not_exist(entity_name)),
            Some(custom_work_intervals) => custom_work_intervals.add_work_interval(interval),
        }
    }


    /// Returns the custom work hours of the entity with the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not registered (we assume it does not exist).
    pub fn custom_work_intervals_of(&self, entity_name: &str) -> Result<Vec<TimeInterval>> {
        match self.custom_work_intervals.get(entity_name) {
            None => Err(DoesNotExist::entity_does_not_exist(entity_name)),
            Some(custom_work_intervals) => Ok(custom_work_intervals.work_intervals().clone()),
        }
    }
}
