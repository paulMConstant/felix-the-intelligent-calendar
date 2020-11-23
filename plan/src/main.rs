extern crate gio;
extern crate gtk;

//use gio::prelude::*;
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../glade/gui.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("MainWindow").unwrap();

    window.show_all();

    gtk::main();
}

//use std::env::args;

//fn build_ui(application: &gtk::Application) {
    //let window = gtk::ApplicationWindow::new(application);

    //window.set_title("First GTK+ Program");
    //window.set_border_width(10);
    //window.set_position(gtk::WindowPosition::Center);
    //window.set_default_size(350, 70);

    //let button = gtk::Button::with_label("Click me!");

    //window.add(&button);

    //window.show_all();
//}

//fn main() {
    //let application =
        //gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            //.expect("Initialization failed...");

    //application.connect_activate(|app| {
        //build_ui(app);
    //});

    //application.run(&args().collect::<Vec<_>>());
//}
