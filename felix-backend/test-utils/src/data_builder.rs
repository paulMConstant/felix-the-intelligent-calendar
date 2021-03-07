use felix_backend::data::{Data, Time, TimeInterval};

use crate::{Activity, Group};

pub struct DataBuilder {
    data: Data,
}

impl DataBuilder {
    #[must_use]
    pub fn new() -> DataBuilder {
        DataBuilder { data: Data::new() }
    }

    #[must_use]
    pub fn with_entity<S>(&mut self, entity: S) -> &mut DataBuilder
    where
        S: Into<String>,
    {
        self.data.add_entity(entity).expect("Could not add entity");
        self
    }

    #[must_use]
    pub fn with_entities(&mut self, entities: Vec<&'static str>) -> &mut DataBuilder {
        for entity in entities {
            let _ = self.with_entity(entity);
        }
        self
    }

    #[must_use]
    pub fn with_custom_work_interval_for<S>(
        &mut self,
        entity: S,
        interval: TimeInterval,
    ) -> &mut DataBuilder
    where
        S: Into<String>,
    {
        self.data
            .add_custom_work_interval_for(entity, interval)
            .expect("Could not add custom work interval");
        self
    }

    #[must_use]
    pub fn with_custom_work_intervals_for<S>(
        &mut self,
        entity: S,
        intervals: Vec<TimeInterval>,
    ) -> &mut DataBuilder
    where
        S: Into<String> + Clone,
    {
        for interval in intervals {
            let _ = self.with_custom_work_interval_for(entity.clone(), interval);
        }
        self
    }

    #[must_use]
    pub fn with_group(&mut self, group: Group) -> &mut DataBuilder {
        let group_name = self
            .data
            .add_group(group.name)
            .expect("Could not add group");
        for entity in group.entities {
            self.data
                .add_entity_to_group(&group_name, entity)
                .expect("Could not add entity to group");
        }
        self
    }

    #[must_use]
    pub fn with_groups(&mut self, groups: Vec<Group>) -> &mut DataBuilder {
        for group in groups {
            let _ = self.with_group(group);
        }
        self
    }

    #[must_use]
    pub fn with_work_interval(&mut self, work_interval: TimeInterval) -> &mut DataBuilder {
        self.data
            .add_work_interval(work_interval)
            .expect("Could not add work interval");
        self
    }

    #[must_use]
    pub fn with_work_intervals(&mut self, work_intervals: Vec<TimeInterval>) -> &mut DataBuilder {
        for work_interval in work_intervals {
            let _ = self.with_work_interval(work_interval);
        }
        self
    }

    /// Convenience functions which adds an interval from [00:00 to hours:00].
    #[must_use]
    pub fn with_work_interval_of_duration(&mut self, hours: i8) -> &mut DataBuilder {
        let interval = TimeInterval::new(Time::new(0, 0), Time::new(hours, 0));
        self.with_work_interval(interval)
    }

    #[must_use]
    pub fn with_activity(&mut self, activity: Activity) -> &mut DataBuilder {
        let id = self
            .data
            .add_activity(activity.name)
            .expect("Could not add activity")
            .id();

        for entity in activity.entities {
            self.data
                .add_entity_to_activity(id, entity)
                .expect("Could not add entity to activity");
        }

        for group in activity.groups {
            self.data
                .add_group_to_activity(id, group)
                .expect("Could not add group to activity");
        }
        self.data
            .set_activity_duration(id, activity.duration)
            .expect("Could not set activity duration");

        while self
            .data
            .possible_insertion_times_of_activity(id)
            .expect("Could not get activity by ID")
            .is_none()
        {
            // Wait for possible insertion times to be asynchronously calculated
        }
        self.data
            .insert_activity(id, activity.insertion_time)
            .expect("Could not insert activity");
        self
    }

    #[must_use]
    pub fn with_activities(&mut self, activities: Vec<Activity>) -> &mut DataBuilder {
        for activity in activities {
            let _ = self.with_activity(activity);
        }
        self
    }

    /// Consumes the data builder and returns the built Data object.
    #[must_use]
    pub fn into_data(&mut self) -> Data {
        std::mem::replace(&mut self.data, Data::new())
    }
}
