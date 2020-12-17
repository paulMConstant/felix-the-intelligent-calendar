use gio::prelude::*;

use crate::app::App;

pub fn build_app(app: &gtk::Application) {
    build_resources(app);

    let mut app = App::new(app);

    app.connect_gtk();
    app.connect_data();
    app.show_mainwindow();
}

fn build_resources(app: &gtk::Application) {
    app.set_resource_base_path(Some("/com/github/paulmconstant/plan"));

    let resources_bytes = include_bytes!("../../res/resources.gresource");
    let resources_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resources_data).expect("Could not load resources_data");
    gio::resources_register(&res);
}
