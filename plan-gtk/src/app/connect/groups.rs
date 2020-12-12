use crate::app::App;

impl App {
    pub fn connect_groups_tab(&self) {
        self.connect_add_group();
        self.connect_add_entity_to_group();
    }

    fn connect_add_group(&self) {}

    fn connect_add_entity_to_group(&self) {}
}
