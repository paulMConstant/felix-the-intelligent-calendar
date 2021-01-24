#[macro_use]
mod macros;

use crate::data::{Activity, Data, Entity, Group};

use paste::paste;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// Build the event struct with fields and accessers.
// See macros for more info.
create_events!(Events:
    entity_added { new_entity: &Entity },
    entity_removed { position_of_removed_entity: usize },
    entity_renamed { entity: &Entity, old_name: &String },
    custom_work_hours_changed {},
    group_added {new_group: &Group},
    group_removed {position_of_removed_group: usize },
    group_renamed {group: &Group},
    activity_added { new_activity: &Activity},
    activity_removed { position_of_removed_activity: usize },
    activity_renamed { activity: &Activity},
    activity_duration_changed { activity: &Activity},
    entity_added_to_activity { activity: &Activity},
    entity_removed_from_activity { activity: &Activity},
    entity_added_to_group {group: &Group},
    entity_removed_from_group {group: &Group},
    group_added_to_activity { activity: &Activity},
    group_removed_from_activity { activity: &Activity},
    work_hours_changed {}
);

/// Data implementation for events.
impl Data {
    pub fn events(&self) -> Rc<RefCell<Events>> {
        self.events.clone()
    }
}
