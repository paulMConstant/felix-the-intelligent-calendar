#[macro_use]
mod macros;

use crate::data::Data;
use paste::paste;
use std::fmt;

type Callbacks = Vec<Box<dyn FnMut()>>;

// Build the event struct with fields and accessers.
// See macros for more info.
create_events!(
    entity_added,
    entity_removed,
    entity_renamed,
    entity_mail_changed,
    entity_send_me_a_mail_changed,
    entity_custom_work_hours_changed,
    group_added,
    group_removed,
    group_renamed,
    activity_added,
    activity_removed,
    activity_renamed,
    activity_duration_changed,
    entity_added_to_activity,
    entity_removed_from_activity,
    entity_added_to_group,
    entity_removed_from_group,
    group_added_to_activity,
    group_removed_from_activity,
    work_hours_changed
);

impl Eq for Events {}
impl PartialEq for Events {
    fn eq(&self, _other: &Self) -> bool {
        // We don't care about event equality. This is implemented for Data.
        true
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Events::new()
    }
}

impl fmt::Debug for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event does not implement Debug")
    }
}

/// Data implementation for events.
impl Data {
    pub fn events(&mut self) -> &mut Events {
        &mut self.events
    }
}
