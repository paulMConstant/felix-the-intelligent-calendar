use crate::app::App;

//use glib::clone;

impl App {
    pub fn connect_entity_data_events(&mut self) {
        self.connect_entity_renamed();
    }

    fn connect_entity_renamed(&mut self) {
        //let app_data = self.app_data.clone();
        //app_data.lock().unwrap().events().do_when_entity_renamed(
        //vec![
        //Box::new(clone!(@strong app_data => move || {
        //app_data.lock().unwrap().on_entity_renamed();
        //}))]
        //);
    }
}
