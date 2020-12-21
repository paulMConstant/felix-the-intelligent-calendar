macro_rules! create_callback_vec {
    ($($param_type: ty),*) => {
        Vec<create_callback!($($param_type),*)>
    };
}

macro_rules! create_callback {
    ($($param_type: ty),*) => {
        Box<dyn FnMut($($param_type),*)>
    };
}

macro_rules! create_events_struct {
    ($($element: ident { $($param_type: ty),* }),*) => {
        pub struct Events { $($element: create_callback_vec!($($param_type),*)),* }
    }
}

macro_rules! create_events_new {
    ($($element: ident),*) => {
        pub(in super::super) fn new() -> Events {
            Events {$($element: Vec::new()),* }
        }
    };
}

macro_rules! create_connect_events {
    ($($element: ident { $($param_type: ty),*}),*) => {
        paste! {
            $(
        pub fn [<connect_ $element>](&mut self, callback: create_callback!($($param_type),*)) {
            self.$element.push(callback);
            println!("Connect {} : {} callback", stringify!($element), self.$element.len());
        }
        )*
        }
    };
}

macro_rules! create_emit_events {
    ($($element: ident { $($param_name: ident : $param_type: ty),* }),*) => {
        paste! {
            $(
        pub(in super::super) fn [<emit_ $element>](&mut self, $($param_name: $param_type),*) {
            println!("Emit {}: {} callback", stringify!($element), self.$element.len());
            for callback in &mut self.$element {
                println!("Callback");
                callback($($param_name),*);
            }
        })*
        }
    };
}

macro_rules! create_events_impl {
    ($($element: ident { $($param_name: ident : $param_type: ty),* }),*) => {
        impl Events {
            create_events_new!($($element),*);
            create_connect_events!($($element { $($param_type),* }),*);
            create_emit_events!($($element { $($param_name: $param_type),* }),*);
        }
    };
}

/// Builds the events struct.
///
/// # Example
///
///```
/// create_events!(
///     renamed {old_name: &String, new_name: &String},
///     something_changed {}
/// )
///```
/// expands to :
///
///```
/// pub struct Events {
///     renamed: Vec<Box<dyn FnMut(&String, &String)>>,
///     something_changed: Vec<Box<dyn FnMut()>>,
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
///         callbacks: Vec<Box<dyn FnMut(&String, &String)>>) {
///         self.renamed.extend(callbacks);
///     }
///
///     pub fn do_when_something_changed(&mut self,
///         callbacks: Vec<Box<dyn FnMut()>>) {
///         self.something_changed.extend(callbacks);
///     }
///
///     pub(in super::super) fn emit_renamed(&mut self,
///                                          old_name: &String,
///                                          new_name: &String) {
///         for callback in &mut self.renamed {
///             callback(old_name, new_name);
///         }
///     }
///
///     pub(in super::super) fn emit_something_changed(&mut self) {
///         for callback in &mut self.something_changed {
///             callback();
///         }
///     }
/// }
/// ```
macro_rules! create_events {
    ($($element: ident { $($param_name: ident : $param_type: ty),* } ),*) => {
        create_events_struct!($($element { $($param_type),* }),*);
        create_events_impl!($($element { $($param_name: $param_type),* }),*);
    };
}
