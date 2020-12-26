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
    ($self: expr, $widget: ident, $connection: expr) => {
        let signal = $connection;
        $self.ui().register_signal($widget, signal);
    };
}
