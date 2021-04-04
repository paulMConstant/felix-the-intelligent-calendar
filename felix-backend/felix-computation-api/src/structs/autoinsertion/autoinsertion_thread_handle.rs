use crate::structs::ActivityBeginningMinutes;

use std::sync::{mpsc, Arc, Mutex};

pub struct AutoinsertionThreadHandle {
    result_receiver: mpsc::Receiver<Option<Vec<ActivityBeginningMinutes>>>,
    worker_terminate_senders: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
}

impl AutoinsertionThreadHandle {
    #[must_use]
    pub fn new(
        result_receiver: mpsc::Receiver<Option<Vec<ActivityBeginningMinutes>>>,
        worker_terminate_senders: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
    ) -> AutoinsertionThreadHandle {
        AutoinsertionThreadHandle {
            result_receiver,
            worker_terminate_senders,
        }
    }

    #[must_use]
    pub fn try_get_result(&self) -> Option<Option<Vec<ActivityBeginningMinutes>>> {
        self.result_receiver.try_recv().ok()
    }

    #[must_use]
    pub fn get_result(&self) -> Option<Vec<ActivityBeginningMinutes>> {
        self.result_receiver.recv().unwrap()
    }

    pub fn stop(&self) {
        for terminate_sender in &*self.worker_terminate_senders.lock().unwrap() {
            // If no one is listening, this is fine, this is what we want
            let _ = terminate_sender.send(());
        }
    }
}
