use crate::structs::autoinsertion::{Node, NodesSortedByScore};
use felix_datatypes::{ActivityBeginningMinutes, Cost};

use std::sync::{mpsc, Arc, Mutex};

/// A destructive tree structure shared among all workers.
pub struct NodePool {
    unexplored_nodes: NodesSortedByScore,

    result_sender: mpsc::Sender<Option<Vec<ActivityBeginningMinutes>>>,
    worker_thread_terminate_handles: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
    n_workers: usize,
    n_inactive_workers: usize,
    most_activities_inserted: usize,
}

impl NodePool {
    #[must_use]
    pub fn new(
        unexplored_nodes: Vec<(Cost, Node)>,
        result_sender: mpsc::Sender<Option<Vec<ActivityBeginningMinutes>>>,
        worker_thread_terminate_handles: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
        n_workers: usize,
    ) -> NodePool {
        NodePool {
            unexplored_nodes: NodesSortedByScore::new(unexplored_nodes),
            result_sender,
            worker_thread_terminate_handles,
            n_workers,
            n_inactive_workers: 0,
            most_activities_inserted: 0,
        }
    }

    pub fn get_most_activities_inserted(&self) -> usize {
        self.most_activities_inserted
    }

    pub fn send_partial_solution(&mut self, solution: Vec<ActivityBeginningMinutes>) {
        let nb_activities_inserted = solution.len();

        if self.most_activities_inserted < nb_activities_inserted {
            self.most_activities_inserted = nb_activities_inserted;

            if self.result_sender.send(Some(solution)).is_err() {
                // There is no receiving end -> abort
                self.kill_worker_threads();
            }
        }
    }

    pub fn send_complete_solution(&mut self, solution: Vec<ActivityBeginningMinutes>) {
        self.kill_worker_threads();
        let _ = self.result_sender.send(Some(solution));
    }

    pub fn send_no_solution(&self) {
        self.kill_worker_threads();
        let _ = self.result_sender.send(None);
    }

    pub fn merge_and_load_nodes(
        &mut self,
        nodes_to_merge: &mut NodesSortedByScore,
        worker_active: &mut bool,
    ) {
        // Add nodes to the node_pool
        self.unexplored_nodes.merge_append(nodes_to_merge);

        if let Some((cost, node)) = self.unexplored_nodes.node_with_lowest_cost() {
            nodes_to_merge.insert(cost, vec![node]);
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

        // Killing worker threads will kill this object (they all own it)
        for handle in &*worker_thread_terminate_handles {
            // If the worker thread is already dead, that's fine
            let _ = handle.send(());
        }
        worker_thread_terminate_handles.clear();
    }
}
