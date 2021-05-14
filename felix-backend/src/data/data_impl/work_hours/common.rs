//! Functions used by both global and custom work hours.

use crate::data::Data;
use crate::errors::{
    change_work_hours_while_activity_inserted::ChangeWorkHoursWhileActivityInserted, Result,
};

impl Data {
    pub(in super::super) fn notify_work_hours_changed(&mut self) {
        self.events().borrow_mut().emit_work_hours_changed(self);
        self.queue_entities_on_global_work_hour_change();
    }

    pub(in super::super) fn check_no_activity_inserted(&self) -> Result<()> {
        if self
            .activities_not_sorted()
            .iter()
            .any(|activity| activity.insertion_interval().is_some())
        {
            Err(ChangeWorkHoursWhileActivityInserted::new())
        } else {
            Ok(())
        }
    }
}
