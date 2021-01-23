#[macro_use]
mod macros;

use crate::app::App;
use glib::clone;

impl App {
    generate_connect_data_function!(connect_data_events:
        connect_entity_added => |entity| { on_entity_added, on_entities_or_groups_changed },
        connect_entity_renamed => |entity| { on_entity_renamed, on_group_members_changed, on_entities_or_groups_changed },
        connect_entity_removed => |position| { on_entity_removed, on_group_members_changed, on_entities_or_groups_changed },
        connect_group_added => |group| { on_group_added, on_entities_or_groups_changed },
        connect_group_renamed => |group| { on_group_renamed, on_entities_or_groups_changed },
        connect_group_removed => |position| { on_group_removed, on_entities_or_groups_changed },
        connect_entity_added_to_group => |group| { on_group_members_changed },
        connect_entity_removed_from_group => |group| { on_group_members_changed },
        connect_activity_added => |activity| { on_activity_added },
        connect_activity_removed => |position| { on_activity_removed },
        connect_activity_renamed => |activity| { on_activity_renamed },
        connect_entity_added_to_activity => |activity| { on_activity_entities_changed },
        connect_entity_removed_from_activity => |activity| {on_activity_entities_changed},
        connect_group_added_to_activity => |activity| { on_activity_groups_changed },
        connect_group_removed_from_activity => |activity| { on_activity_groups_changed},
        connect_work_hours_changed => | | { on_work_hours_changed, on_work_hours_changed_update_schedules }
    );
}
