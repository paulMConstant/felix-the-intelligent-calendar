// Block signals macros

/// Use 'with_blocked_signals' instead.
macro_rules! block_signals {
    ($blocker:expr, $widget:ident) => {
        if let Some(signals) = $blocker.get_registered_signals(&$widget) {
            for signal_id in signals {
                $widget.block_signal(&signal_id);
            }
        }
    };

    ($blocker:expr, $widget:ident, $($rest: ident),*) => {
        block_signals!($blocker, $widget);
        block_signals!($blocker, $($rest),*);
    };
}

/// Use 'with_blocked_signals' instead.
macro_rules! unblock_signals {
    ($blocker:expr, $widget:ident) => {
        if let Some(signals) = $blocker.get_registered_signals(&$widget) {
            for signal_id in signals {
                $widget.unblock_signal(&signal_id);
            }
        }
    };

    ($blocker:expr, $widget:ident, $($rest: ident),*) => {
        unblock_signals!($blocker, $widget);
        unblock_signals!($blocker, $($rest),*);
    };
}

/// Blocks the signals of given widgets,
/// executes the given block of code and unblock the signals.
///
/// # Arguments
///
/// $blocker: who registered the signals, $do\_with\_blocked\_signals: block of code to do with blocked signals,
/// $widgets: widgets whose signals should be blocked
///
/// # Example
///
/// with\_blocked\_signals!(self, { do\_something(); }, blocked\_widget1, blocked\_widget2);
macro_rules! with_blocked_signals {
    ($blocker: expr, $do_whith_blocked_signals: expr, $widget:ident) => {
        block_signals!($blocker, $widget);
        $do_whith_blocked_signals;
        unblock_signals!($blocker, $widget);
    };

    ($blocker: expr, $do_whith_blocked_signals: expr, $widget:ident, $($rest: ident),*) => {
        block_signals!($blocker, $widget, $($rest),*);
        $do_whith_blocked_signals;
        unblock_signals!($blocker, $widget, $($rest),*);
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

// Return if err macros

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
        if $expr.is_err() {
            return;
        }
    };
}

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
