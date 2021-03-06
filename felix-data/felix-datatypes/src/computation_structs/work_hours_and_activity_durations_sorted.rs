use crate::{Time, TimeInterval, WorkHourInMinutes};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkHoursAndActivityDurationsSorted {
    work_hours: Vec<TimeInterval>,
    activity_durations: Vec<Time>,
}

impl WorkHoursAndActivityDurationsSorted {
    pub fn new(
        mut work_hours: Vec<TimeInterval>,
        mut activity_durations: Vec<Time>,
    ) -> WorkHoursAndActivityDurationsSorted {
        work_hours.sort_by_key(|a| a.duration());
        activity_durations.sort();
        WorkHoursAndActivityDurationsSorted {
            work_hours,
            activity_durations,
        }
    }

    pub fn work_hours_in_minutes(&self) -> Vec<WorkHourInMinutes> {
        self.work_hours
            .iter()
            .map(|&time_interval| {
                WorkHourInMinutes::new(
                    time_interval.beginning().total_minutes(),
                    time_interval.end().total_minutes(),
                )
            })
            .collect()
    }

    pub fn activity_durations_in_minutes(&self) -> Vec<u16> {
        self.activity_durations
            .iter()
            .map(|activity_duration| activity_duration.total_minutes())
            .collect()
    }
}
