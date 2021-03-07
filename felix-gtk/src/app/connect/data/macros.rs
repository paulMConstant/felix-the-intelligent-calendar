macro_rules! call_ui_events {
    ($caller: expr, $callback: ident : $($arg: ident),*) => {
        $caller.$callback($($arg),*);
    };

    ($caller: expr, $callback:ident, $($rest_callback: ident),* : $($arg: ident),*) => {
        call_ui_events!($caller, $callback : $($arg),*);
        call_ui_events!($caller, $($rest_callback),* : $($arg),*);
    };
}

macro_rules! generate_private_connect_function {
    ($event: ident => | | { $($callback: ident),* }) => {
        fn $event(&mut self) {
            let events = self.data.lock().unwrap().events();
            let ui = self.ui.clone();

            events.borrow_mut().$event(Box::new(
                    move |data| {
                           call_ui_events!(ui.lock().unwrap(), $($callback),* : data);
                    }
                ));
        }
    };

    ($event: ident => | $($arg:ident),* | { $($callback: ident),* }) => {
        fn $event(&mut self) {
            let events = self.data.lock().unwrap().events();
            let ui = self.ui.clone();

            events.borrow_mut().$event(Box::new(
                    move |data, $($arg),*| {
                        call_ui_events!(ui.lock().unwrap(), $($callback),* : data, $($arg),*);
                    }
                ));
        }
    };

    ($event: ident => | $($arg:ident),* | { $($callback: ident),* }, $($rest_events: ident => | $($rest_args:ident),* | { $($rest_callbacks: ident),* }),*) => {
        generate_private_connect_function!($event => | $($arg),* | { $($callback),* });
        generate_private_connect_function!($($rest_events => | $($rest_args),* | { $($rest_callbacks),* }),*);
    };
}

macro_rules! call_private_connect_function {
    ($self: ident, $f: ident) => {
        $self.$f();
    };

    ($self: ident, $f:ident, $($rest: ident),*) => {
        call_private_connect_function!($self, $f);
        call_private_connect_function!($self, $($rest),*);
    };
}

macro_rules! generate_connect_data_function {
    ($main_connect_function: ident : $($event:ident => | $($arg:ident),* | { $($callbacks: ident),* }),*) => {
        generate_private_connect_function!($($event => | $($arg),* | { $($callbacks),* }),*);

        pub fn $main_connect_function(&mut self) {
            call_private_connect_function!(self, $($event),*);
        }
        };
}
