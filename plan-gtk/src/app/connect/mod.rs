/// Should be used only in the App impl.
///
/// # Arguments
///
/// $self: self, $widget: name of the widget, $connection: normal connection expression
///
/// # Definition
///
/// macro_rules! app_register_signal {
/// ($self: ident, $widget: ident, $connection: expr) => {
///     let signal = $connection;
///     $self.app\_data.lock().unwrap().register\_signal($widget, signal);
/// };
macro_rules! app_register_signal {
    ($self: ident, $widget: ident, $connection: expr) => {
        let signal = $connection;
        $self
            .app_data
            .lock()
            .unwrap()
            .register_signal($widget, signal);
    };
}

// MACRO SHOULD STAY ABOVE THE MOD DECLARATION !
// Otherwise, the macro is not found in the child modules.

//mod activities;
mod entities;
//mod work_hours;
//mod groups;
//mod activity_insertion;
mod header;

use crate::app::App;

impl App {
    pub fn connect_gtk(&self) {
        self.connect_header_buttons();
        self.connect_entities_tab();
    }
}
