use crate::structs::ActivityBeginningMinutes;

use std::sync::{mpsc, Arc, Mutex};

pub struct AutoinsertionThreadHandle {
    result: Arc<Mutex<Option<Result<Vec<ActivityBeginningMinutes>, ()>>>>,
    worker_terminate_senders: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
}

impl AutoinsertionThreadHandle {
    #[must_use]
    pub fn new(
        result: Arc<Mutex<Option<Result<Vec<ActivityBeginningMinutes>, ()>>>>,
        worker_terminate_senders: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
    ) -> AutoinsertionThreadHandle {
        AutoinsertionThreadHandle {
            result,
            worker_terminate_senders,
        }
    }

    #[must_use]
    pub fn try_get_result(&self) -> Option<Result<Vec<ActivityBeginningMinutes>, ()>> {
        self.result.lock().unwrap().clone()
    }

    #[must_use]
    pub fn get_result(&self) -> Result<Vec<ActivityBeginningMinutes>, ()> {
        loop {
            let maybe_result = self.try_get_result();
            if let Some(result) = maybe_result {
                return result;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn stop(&self) {
        for terminate_sender in &*self.worker_terminate_senders.lock().unwrap() {
            // If no one is listening, this is fine, this is what we want
            let _ = terminate_sender.send(());
        }
    }
}
