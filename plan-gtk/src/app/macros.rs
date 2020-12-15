// To be used by AppDataImpl

/// Should only be called from AppData impl.
/// Use with_blocked_signals instead.
macro_rules! block_signals {
    ($self:ident, $widget:ident) => {
        if let Some(signals) = $self.get_registered_signals(&$widget) {
            for signal_id in signals {
                $widget.block_signal(&signal_id);
            }
        }
    };

    ($self:ident, $widget:ident, $($rest: ident),*) => {
        block_signals!($self, $widget);
        block_signals!($self, $($rest),*);
    };
}

/// Should only be called from AppData impl.
/// Use with_blocked_signals instead.
macro_rules! unblock_signals {
    ($self:ident, $widget:ident) => {
        if let Some(signals) = $self.get_registered_signals(&$widget) {
            for signal_id in signals {
                $widget.unblock_signal(&signal_id);
            }
        }
    };

    ($self:ident, $widget:ident, $($rest: ident),*) => {
        unblock_signals!($self, $widget);
        unblock_signals!($self, $($rest),*);
    };
}

/// Should only be called from AppData impl.
///
/// Blocks the signals of given widgets,
/// executes the given block of code and unblock the signals.
///
/// # Arguments
///
/// $self: self, $do\_with\_blocked\_signals: block of code to do with blocked signals,
/// $widgets: widgets whose signals should be blocked
///
/// # Example
///
/// with\_blocked\_signals!(self, { do\_something(); }, blocked\_widget1, blocked\_widget2);
macro_rules! with_blocked_signals {
    ($self: ident, $do_whith_blocked_signals: expr, $widget:ident) => {
        block_signals!($self, $widget);
        $do_whith_blocked_signals;
        unblock_signals!($self, $widget);
    };

    ($self: ident, $do_whith_blocked_signals: expr, $widget:ident, $($rest: ident),*) => {
        block_signals!($self, $widget, $($rest),*);
        $do_whith_blocked_signals;
        unblock_signals!($self, $widget, $($rest),*);
    };
}

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

// Notify macros

/// If the given expression fails, notifies the user and returns.
/// Else, assigns the variable with given name to the result.
macro_rules! assign_or_return {
    ($var: ident, $expr: expr) => {
        let res = $expr;
        if let Err(e) = res {
            use crate::app::notify::notify_err;
            notify_err(e);
            return;
        }
        let $var = res.expect("Error case should have been taken care of in macro above");
    };
}

/// If the given expression fails, returns.
/// Else, assigns the variable with given name to the result.
macro_rules! no_notify_assign_or_return {
    ($var: ident, $expr: expr) => {
        let res = $expr;
        if res.is_err() {
            return;
        }
        let $var = res.expect("Error case should have been taken care of in macro above");
    };
}

/// If the given expression fails, notifies the user and returns.
macro_rules! return_if_err {
    ($expr: expr) => {
        use crate::app::notify::notify_err;
        if let Err(e) = $expr {
            notify_err(e);
            return;
        }
    };
}

/// If the given expression fails, silently returns.
macro_rules! no_notify_return_if_err {
    ($expr: expr) => {
        if let Err(_) = $expr {
            return;
        }
    };
}
// Fetch ui macros

/// Fetches the gtk component with given name from the builder.
macro_rules! fetch_ui_from_builder {
    ($from: expr, $id: literal) => {
        $from
            .builder
            .get_object($id)
            .expect(&format!("Could not get {} from ui file", $id)[..])
    };
}

// Free macros

/// Creates variables with the same name as object methods.
///
/// # Example
///
/// fetch\_from(self, x, y) expands to :
/// let x = self.x();
/// let y = self.y();
macro_rules! fetch_from {
    ($from: expr, $x: ident) => {
        let $x = $from.$x();
    };

    ($from: expr, $x: ident, $($rest:ident),*) => {
        fetch_from!($from, $x);
        fetch_from!($from, $($rest),*);
    };
}
