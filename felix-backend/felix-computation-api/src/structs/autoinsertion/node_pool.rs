use crate::structs::{autoinsertion::Node, ActivityBeginningMinutes};

use std::sync::{mpsc, Arc, Mutex};

/// A destructive tree structure shared among all workers.
pub struct NodePool {
    unexplored_nodes: Vec<Node>,

    result_sender: mpsc::Sender<Option<Vec<ActivityBeginningMinutes>>>,
    worker_thread_terminate_handles: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
    n_workers: usize,
    n_inactive_workers: usize,
}

impl NodePool {
    #[must_use]
    pub fn new(
        unexplored_nodes: Vec<Node>,
        result_sender: mpsc::Sender<Option<Vec<ActivityBeginningMinutes>>>,
        worker_thread_terminate_handles: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
        n_workers: usize,
    ) -> NodePool {
        NodePool {
            unexplored_nodes,
            result_sender,
            worker_thread_terminate_handles,
            n_workers,
            n_inactive_workers: 0,
        }
    }

    pub fn send_solution(&mut self, solution: Vec<ActivityBeginningMinutes>) {
        self.kill_worker_threads();
        let _ = self.result_sender.send(Some(solution));
    }

    pub fn send_no_solution(&self) {
        self.kill_worker_threads();
        let _ = self.result_sender.send(None);
    }

    pub fn merge_and_load_nodes(&mut self, nodes: &mut Vec<Node>, worker_active: &mut bool) {
        // Add nodes to the node_pool
        self.unexplored_nodes.append(nodes);

        if let Some(node) = self.unexplored_nodes.pop() {
            nodes.push(node);
            if !*worker_active {
                self.n_inactive_workers -= 1;
                *worker_active = true;
            }
        } else if *worker_active {
            self.n_inactive_workers += 1;
            *worker_active = false;
            if self.n_inactive_workers == self.n_workers {
                self.send_no_solution();
            }
        } else {
            // Yield worker to wait for other nodes to synchronize
            std::thread::yield_now();
        }
    }

    pub fn kill_worker_threads(&self) {
        let mut worker_thread_terminate_handles =
            self.worker_thread_terminate_handles.lock().unwrap();

        // Kill worker threads
        for handle in &*worker_thread_terminate_handles {
            // If the worker thread is already dead, that's fine
            let _ = handle.send(());
        }
        worker_thread_terminate_handles.clear();
    }
}
