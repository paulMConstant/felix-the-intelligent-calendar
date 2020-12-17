macro_rules! create_events_struct {
    ($($element: ident),*) => {
        pub struct Events { $($element: Callbacks),* }
    }
}

macro_rules! create_events_new {
    ($($element: ident),*) => {
        pub(in super::super) fn new() -> Events {
            Events {$($element: Vec::new()),* }
        }
    };
}

macro_rules! create_do_when_events {
    ($($element: ident),*) => {
        paste! {
            $(
        pub fn [<do_when_ $element>](&mut self, callbacks: Callbacks) {
            self.$element.extend(callbacks);
        }
        )*
        }
    };
}

macro_rules! create_emit_events {
    ($($element: ident),*) => {
        paste! {
            $(
        pub(in super::super) fn [<emit_ $element>](&mut self, data: &Data) {
            for callback in &mut self.$element {
                callback(data);
            }
        })*
        }
    };
}

macro_rules! create_events_impl {
    ($($element: ident),*) => {
        impl Events {
            create_events_new!($($element),*);
            create_do_when_events!($($element),*);
            create_emit_events!($($element),*);
        }
    };
}

/// Builds the events struct.
///
/// # Example
///
/// create\_events!(event1, event2) expands to :
///
///```
/// pub struct Events {
///     event1: Callbacks,
///     event2: Callbacks,
/// }
///
/// impl Events {
///     pub(in super::super) fn new() -> Events {
///         Events {
///             event1: Vec::new(),
///             event2: Vec::new(),
///         }
///     }
///     pub fn do_when_event1(&mut self, callbacks: Callbacks) {
///         self.event1.extend(callbacks);
///     }
///     pub fn do_when_event2(&mut self, callbacks: Callbacks) {
///         self.event2.extend(callbacks);
///     }
///     pub(in super::super) fn emit_event1(&mut self) {
///         for callback in &mut self.event1 {
///             callback();
///         }
///     }
///     pub(in super::super) fn emit_event2(&mut self) {
///         for callback in &mut self.event2 {
///             callback();
///         }
///     }
/// }
///
/// assert!(true);
/// ```
macro_rules! create_events {
    ($($element: ident),*) => {
        create_events_struct!($($element),*);
        create_events_impl!($($element),*);
    };
}
