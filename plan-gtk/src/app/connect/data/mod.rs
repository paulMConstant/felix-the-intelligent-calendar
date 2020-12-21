macro_rules! generate_private_connect_function {
    ($event: ident => | | $callback: ident) => {
        fn $event(&mut self) {
            let events = self.data.lock().unwrap().events();
            let ui = self.ui.clone();

            events.borrow_mut().$event(Box::new(
                    clone!(@strong ui => move || {
                           ui.lock().unwrap().$callback();
                    }
                )));
        }
    };

    ($event: ident => | $($arg:ident),* | $callback: ident) => {
        fn $event(&mut self) {
            let events = self.data.lock().unwrap().events();
            let ui = self.ui.clone();

            events.borrow_mut().$event(Box::new(
                    clone!(@strong ui => move |$($arg),*| {
                           ui.lock().unwrap().$callback($($arg),*);
                    }
                )));
        }
    };

    ($event: ident => | $($arg:ident),* | $callback: ident, $($rest_events: ident => | $($rest_args:ident),* | $rest_callbacks: ident),*) => {
        generate_private_connect_function!($event => | $($arg),* | $callback);
        generate_private_connect_function!($($rest_events => | $($rest_args),* | $rest_callbacks),*);
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

macro_rules! generate_connect_functions {
    ($main_connect_function: ident : $($event:ident => | $($arg:ident),* | $callback: ident),*) => {
        generate_private_connect_function!($($event => | $($arg),* | $callback),*);

        pub fn $main_connect_function(&mut self) {
            call_private_connect_function!(self, $($event),*);
        }
        };
}

mod activities;
mod entities;
mod groups;
