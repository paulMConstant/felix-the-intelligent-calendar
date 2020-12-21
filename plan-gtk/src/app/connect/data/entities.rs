use crate::app::App;

use glib::clone;

impl App {
    generate_connect_functions!(connect_entity_data_events:
        connect_entity_added => |entity, entities| on_entity_added,
        connect_entity_renamed => |entity, entities| on_entity_renamed,
        connect_entity_removed => |position, entities| on_entity_removed
    );
}
