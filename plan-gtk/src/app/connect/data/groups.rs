use crate::app::App;
use glib::clone;

impl App {
    generate_connect_functions!(connect_group_data_events:
        connect_group_added => |group, groups| on_group_added,
        connect_group_renamed => |group, groups| on_group_renamed,
        connect_group_removed => |position, groups| on_group_removed,
        connect_entity_added_to_group => | | on_group_members_changed,
        connect_entity_removed_from_group => | | on_group_members_changed
    );
}
