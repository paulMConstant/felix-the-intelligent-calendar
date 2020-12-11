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

/// Should only be called from AppData class.
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
    ($self: ident, $do_whith_blocked_signals: block, $widget:ident, $($rest: ident),*) => {
        block_signals!($self, $widget, $($rest),*);
        $do_whith_blocked_signals;
        unblock_signals!($self, $widget, $($rest),*);
    };
}

// MACRO SHOULD STAY ABOVE MOD DECLARATION.
// Otherwise, it will not be able to find it.

mod entities;
