use gio::prelude::*;

use crate::app::App;

pub fn build_app(application: &gtk::Application) {
    build_resources(application);

    let mut app = App::new(application);

    app.connect_ui();
    app.connect_data();
    app.ui.borrow_mut().show_mainwindow();
    app.init_ui_with_existing_data();
    app.recreate_last_ui_state();
}

fn build_resources(app: &gtk::Application) {
    app.set_resource_base_path(Some("/res"));

    let resources_bytes = include_bytes!("../../res/resources.gresource");
    let resources_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resources_data).expect("Could not load resources_data");
    gio::resources_register(&res);
}
