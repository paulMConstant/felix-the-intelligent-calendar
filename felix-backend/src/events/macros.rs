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
        /// Manages events. Connects callbacks which are called in emit functions.
        ///
        /// PartialEq, Eq, Clone, Debug are implemented for compatibility with other structs which
        /// may contain this struct. They do not do anything relevant: the containing
        /// struct should behave as if this struct did not exist.
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

        impl Default for $events_name {
            fn default() -> Self {
                $events_name::new()
            }
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
///     renamed {old_name: &str, new_name: &str},
///     something_changed {}
/// )
///```
/// expands to :
///
///```
/// pub struct Events {
///     renamed: Vec<Box<dyn FnMut(&Data, &str, &str)>>,
///     something_changed: Vec<Box<dyn FnMut(&Data)>>,
/// }
///
/// /// Structure holding all events.
/// impl Events {
///     pub fn new() -> Events {
///         Events {
///             renamed: Vec::new(),
///             something_changed: Vec::new(),
///         }
///     }
///
///     pub fn connect_renamed(&mut self,
///         callbacks: Vec<Box<dyn FnMut(&Data, &str, &str)>>) {
///         self.renamed.extend(callbacks);
///     }
///
///     pub fn connect_when_something_changed(&mut self,
///         callbacks: Vec<Box<dyn FnMut(&Data)>>) {
///         self.something_changed.extend(callbacks);
///     }
///
///     pub fn emit_renamed(&mut self,
///                         data: &Data,
///                         old_name: &str,
///                         new_name: &str) {
///         for callback in &mut self.renamed {
///             callback(data, old_name, new_name);
///         }
///     }
///
///     pub fn emit_something_changed(&mut self, data: &Data) {
///         for callback in &mut self.something_changed {
///             callback(data);
///         }
///     }
/// }
///
/// impl Default for Events {
///     fn default() -> Self {
///         Events::new()
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
