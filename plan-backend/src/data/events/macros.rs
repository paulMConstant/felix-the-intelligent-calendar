macro_rules! create_callback {
    ($($param_type: ty),*) => {
        Box<dyn FnMut(&Data, $($param_type),*)>
    };
}

macro_rules! create_callback_vec {
    ($($param_type: ty),*) => {
        Vec<create_callback!($($param_type),*)>
    };
}

macro_rules! create_events_struct {
    ($events_name: ident: $($element: ident { $($param_type: ty),* }),*) => {
        pub struct $events_name { $($element: create_callback_vec!($($param_type),*)),* }
    }
}

macro_rules! create_events_new {
    ($events_name: ident: $($element: ident),*) => {
        pub fn new() -> $events_name {
            $events_name {$($element: Vec::new()),* }
        }
    };
}

macro_rules! create_connect_events {
    ($($element: ident { $($param_type: ty),*}),*) => {
        paste! {
            $(
        pub fn [<connect_ $element>](&mut self, callback: create_callback!($($param_type),*)) {
            self.$element.push(callback);
        }
        )*
        }
    };
}

macro_rules! create_emit_events {
    ($($element: ident { $($param_name: ident : $param_type: ty),* }),*) => {
        paste! {
            $(
        pub fn [<emit_ $element>](&mut self, data: &Data, $($param_name: $param_type),*) {
            for callback in &mut self.$element {
                callback(data, $($param_name),*);
            }
        })*
        }
    };
}

macro_rules! create_events_impl {
    ($events_name: ident: $($element: ident { $($param_name: ident : $param_type: ty),* }),*) => {
        impl $events_name {
            create_events_new!($events_name: $($element),*);
            create_connect_events!($($element { $($param_type),* }),*);
            create_emit_events!($($element { $($param_name: $param_type),* }),*);
        }

        impl Eq for $events_name {}
        impl PartialEq for $events_name {
            fn eq(&self, _other: &Self) -> bool {
                // We don't care about event equality. This is implemented for convenience.
                true
            }
        }

        impl Clone for $events_name {
            fn clone(&self) -> Self {
                // We don't care about cloning events. This is implemented for convenience.
                $events_name::new()
            }
        }

        impl fmt::Debug for $events_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Event does not implement Debug")
            }
        }
    };
}

/// Builds the events struct.
///
/// # Example
///
///```
/// create_events!(Events:
///     renamed {old_name: &String, new_name: &String},
///     something_changed {}
/// )
///```
/// expands to :
///
///```
/// pub struct Events {
///     renamed: Vec<Box<dyn FnMut(&Data, &String, &String)>>,
///     something_changed: Vec<Box<dyn FnMut(&Data)>>,
/// }
///
/// impl Events {
///     pub(in super::super) fn new() -> Events {
///         Events {
///             renamed: Vec::new(),
///             something_changed: Vec::new(),
///         }
///     }
///
///     pub fn do_when_renamed(&mut self,
///         callbacks: Vec<Box<dyn FnMut(&Data, &String, &String)>>) {
///         self.renamed.extend(callbacks);
///     }
///
///     pub fn do_when_something_changed(&mut self,
///         callbacks: Vec<Box<dyn FnMut(&Data)>>) {
///         self.something_changed.extend(callbacks);
///     }
///
///     pub(in super::super) fn emit_renamed(&mut self,
///                                          data: &Data,
///                                          old_name: &String,
///                                          new_name: &String) {
///         for callback in &mut self.renamed {
///             callback(data, old_name, new_name);
///         }
///     }
///
///     pub(in super::super) fn emit_something_changed(&mut self, data: &Data) {
///         for callback in &mut self.something_changed {
///             callback(data);
///         }
///     }
/// }
///
/// impl Eq for Events {}
/// impl PartialEq for Events {
///     fn eq(&self, _other: &Self) -> bool {
///         // We don't care about event equality. This is implemented for convenience.
///         true
///     }
/// }
///
/// impl Clone for Events {
///     fn clone(&self) -> Self {
///         Events::new()
///     }
/// }
///
/// impl fmt::Debug for Events {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "Event does not implement Debug")
///     }
/// }
/// ```
macro_rules! create_events {
    ($events_name:ident :$($element: ident { $($param_name: ident : $param_type: ty),* } ),*) => {
        create_events_struct!($events_name : $($element { $($param_type),* }),*);
        create_events_impl!($events_name: $($element { $($param_name: $param_type),* }),*);
    };
}
