/// # Arguments
///
/// $self: self, $widget: name of the widget, $connection: normal connection expression
///
/// # Definition
///
/// macro_rules! app_register_signal {
/// ($self: ident, $widget: ident, $connection: expr) => {
///     let signal = $connection;
///     $self.ui.borrow_mut().register\_signal($widget, signal);
/// };
macro_rules! app_register_signal {
    ($self: expr, $widget: ident, $connection: expr) => {
        let signal = $connection;
        $self.ui.borrow_mut().register_signal($widget, signal);
    };
}

/// # Arguments
///
/// $self: App
/// $entry: gtk::Entry to clean
macro_rules! connect_clean {
    ($self: expr, $entry: ident) => {
        fetch_from!($self.ui.borrow(), $entry);

        let ui = $self.ui.clone();
        app_register_signal!(
            $self,
            $entry,
            $entry.connect_changed(
                clone!(@strong ui, @weak $entry => move |_| {
                    ui.borrow().event_clean_entry_content($entry);
                })));
    };

    ($self: expr, $entry: ident, $($rest_entry: ident),*) => {
        connect_clean!($self, $entry);
        connect_clean!($self, $($rest_entry),*);
    };
}
