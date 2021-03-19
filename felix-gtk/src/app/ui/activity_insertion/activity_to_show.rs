use felix_backend::data::{Activity, ActivityId, Rgba, TimeInterval};

/// Simple struct holding an activity's name and insertion interval.
#[derive(Clone, Debug)]
pub struct ActivityToShow {
    id: ActivityId,
    name: String,
    insertion_interval: Option<TimeInterval>,
    color: Rgba,
}

impl ActivityToShow {
    #[must_use]
    pub fn new(activity: &Activity) -> ActivityToShow {
        ActivityToShow {
            id: activity.id(),
            name: activity.name(),
            insertion_interval: activity.insertion_interval(),
            color: activity.color(),
        }
    }

    #[must_use]
    pub fn id(&self) -> ActivityId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn insertion_interval(&self) -> &Option<TimeInterval> {
        &self.insertion_interval
    }

    #[must_use]
    pub fn color(&self) -> &Rgba {
        &self.color
    }
}
