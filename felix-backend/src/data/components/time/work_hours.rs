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
