use crate::{
    compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::{
        autoinsertion::{AutoinsertionThreadHandle, Node, Worker},
        ActivityBeginningMinutes, ActivityComputationStaticData,
    },
};

use std::sync::{mpsc, Arc, Mutex};

const N_NODES_TO_FETCH_WHEN_SYNCING_WITH_TREE: usize = 5;
const N_NODES_TO_LEAVE_IN_THE_TREE: usize = 16;

/// A destructive tree structure shared among all workers.
pub struct Tree {
    unexplored_nodes: Vec<Node>,

    result: Arc<Mutex<Option<Result<Vec<ActivityBeginningMinutes>, ()>>>>,
    worker_thread_terminate_handles: Arc<Mutex<Vec<mpsc::Sender<()>>>>,
}

impl Tree {
    #[must_use]
    pub fn new(
        static_data: &[ActivityComputationStaticData],
        current_insertions: &[ActivityBeginningMinutes],
    ) -> AutoinsertionThreadHandle {
        // Init structs
        let result = Arc::new(Mutex::new(None));
        let worker_thread_terminate_handles = Arc::new(Mutex::new(Vec::new()));

        let auto_insertion_handle =
            AutoinsertionThreadHandle::new(result.clone(), worker_thread_terminate_handles.clone());

        let mut tree = Tree {
            unexplored_nodes: Vec::new(),
            result,
            worker_thread_terminate_handles: worker_thread_terminate_handles.clone(),
        };

        // Init logic
        let num_workers = (num_cpus::get() - 1).max(1);
        let n_activities_to_insert = static_data.len();

        if n_activities_to_insert == current_insertions.len() {
            // All activities are inserted - return the solution
            tree.send_solution(current_insertions.to_vec());
            return auto_insertion_handle;
        }

        // Init n nodes
        let mut init_nodes: Vec<Node> = Vec::with_capacity(num_workers);

        // Create a node for each possible beginning
        for insertion in get_activity_beginnings_with_conflicts(
            static_data,
            current_insertions,
            current_insertions.len(),
        ) {
            init_nodes.push(Node::new(current_insertions.to_vec(), insertion));
        }

        while init_nodes.len() < num_workers {
            if init_nodes.is_empty() {
                // Init nodes have all been expanded - no solution is available
                tree.send_no_solution();
                return auto_insertion_handle;
            }

            // Check if any node has reached a solution
            if let Some(node_with_solution) = init_nodes
                .iter()
                .find(|node| n_activities_to_insert == node.current_insertions.len())
            {
                // All activities are inserted - return the solution
                tree.send_solution(node_with_solution.current_insertions.clone());
                return auto_insertion_handle;
            }

            // Expand the node with the least number of inserted activities
            let (index_node_with_least_number_of_insertions, _) = init_nodes
                .iter()
                .enumerate()
                .min_by_key(|(_index, node)| node.current_insertions.len())
                .expect("Taking min of empty vec");

            let node_with_least_number_of_insertions =
                init_nodes.swap_remove(index_node_with_least_number_of_insertions);

            // Create a node for each possible beginning
            for insertion in get_activity_beginnings_with_conflicts(
                static_data,
                &node_with_least_number_of_insertions.current_insertions,
                node_with_least_number_of_insertions
                    .current_insertions
                    .len(),
            ) {
                init_nodes.push(Node::new(
                    node_with_least_number_of_insertions
                        .current_insertions
                        .to_vec(),
                    insertion,
                ));
            }
        }

        // Cut off init nodes to return exactly n nodes
        tree.unexplored_nodes = init_nodes.split_off(num_workers);

        // Exactly num_workers init nodes generated succesfuly
        let arc_tree = Arc::new(Mutex::new(tree));
        for _ in 0..num_workers {
            let mut worker = Worker::new(
                static_data.to_vec(),
                arc_tree.clone(),
                // We made sure before that there were exactly enough init nodes
                vec![init_nodes.pop().expect("Popping out of empty vector")],
            );

            worker_thread_terminate_handles
                .lock()
                .unwrap()
                .push(worker.exit_sender());
            std::thread::spawn(move || {
                worker.work();
            });
        }

        auto_insertion_handle
    }

    pub fn send_solution(&mut self, solution: Vec<ActivityBeginningMinutes>) {
        self.kill_worker_threads();
        *self.result.lock().unwrap() = Some(Ok(solution));
    }

    pub fn send_no_solution(&self) {
        self.kill_worker_threads();
        *self.result.lock().unwrap() = Some(Err(()));
    }

    pub fn merge_and_load_best_nodes(&mut self, nodes: &mut Vec<Node>) {
        // Add nodes to the tree
        self.unexplored_nodes.append(nodes);
        let tree_len = self.unexplored_nodes.len();

        if tree_len == 0 {
            // TODO better than this:
            // Yield worker for a certain time
            self.send_no_solution();
        } else {
            let n_nodes_to_fetch = if tree_len > N_NODES_TO_LEAVE_IN_THE_TREE {
                N_NODES_TO_FETCH_WHEN_SYNCING_WITH_TREE.min(tree_len - N_NODES_TO_LEAVE_IN_THE_TREE)
            } else {
                1
            };

            *nodes = self.unexplored_nodes.split_off(tree_len - n_nodes_to_fetch);
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
