use felix_datatypes::ActivityBeginningMinutes;

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

    /// Returns the latest result if available.
    /// If no result is available or if the autoinsertion is done, returns None.
    #[must_use]
    pub fn try_get_latest_result(&self) -> Option<Option<Vec<ActivityBeginningMinutes>>> {
        let mut latest_result = None;
        while let Ok(result) = self.result_receiver.try_recv() {
            latest_result = Some(result);
        }
        latest_result
    }

    /// Blocks until the autoinsertion is done then return the final result.
    #[must_use]
    pub fn get_final_result(&self) -> Option<Vec<ActivityBeginningMinutes>> {
        let mut final_result = None;
        // Fetch result until the channel hangs up
        while let Ok(result) = self.result_receiver.recv() {
            final_result = result;
        }
        final_result
    }

    pub fn stop(&self) {
        for terminate_sender in &*self.worker_terminate_senders.lock().unwrap() {
            // If no one is listening, this is fine, this is what we want
            let _ = terminate_sender.send(());
        }
    }
}
