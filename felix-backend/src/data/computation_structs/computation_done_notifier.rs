use std::sync::{Condvar, Mutex};

/// Simple abstraction for a condvar and mutex with a wait and notify function.
#[derive(Debug)]
pub struct ComputationDoneNotifier {
    mutex: Mutex<bool>,
    condvar: Condvar,
}

impl ComputationDoneNotifier {
    pub fn new() -> ComputationDoneNotifier {
        ComputationDoneNotifier {
            mutex: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    /// Waits for a computation result to come.
    ///
    /// This will put the current thread to sleep and yield.
    /// This function can be run in a separate 'watcher' thread which will sleep most of the time
    /// and wake up when a computation result is up.
    pub fn wait_for_computation_result(&self) {
        let mut computation_available = self.mutex.lock().expect("Could not lock mutex !");
        // Protect against spurious wakeups
        while !*computation_available {
            computation_available = self
                .condvar
                .wait(computation_available)
                .expect("Wait on condvar failed !");
        }
        // Reset computation_available before continuing
        *computation_available = false;
    }

    /// Unblocks all waiting thread waiting on a computation result.
    pub fn notify_computation_result(&self) {
        let mut computation_available = self.mutex.lock().expect("Could not lock mutex !");
        *computation_available = true;
        // Notify that the value has changed
        self.condvar.notify_one();
    }
}
