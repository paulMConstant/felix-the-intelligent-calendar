use crate::{
    compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::{
        autoinsertion::{AutoinsertionThreadHandle, Node, NodePool, Worker},
        ActivityBeginningMinutes, ActivityComputationStaticData,
    },
};

use std::sync::{mpsc, Arc, Mutex};

pub fn autoinsert(
    static_data: &[ActivityComputationStaticData],
    current_insertions: &[ActivityBeginningMinutes],
) -> AutoinsertionThreadHandle {
    // Init structs
    let (result_sender, result_receiver) = mpsc::channel();
    let worker_thread_terminate_handles = Arc::new(Mutex::new(Vec::new()));
    let n_workers = num_cpus::get();

    let auto_insertion_handle =
        AutoinsertionThreadHandle::new(result_receiver, worker_thread_terminate_handles.clone());

    // Init logic
    let n_activities_to_insert = static_data.len();

    if n_activities_to_insert == current_insertions.len() {
        // All activities are inserted - return the solution
        NodePool::new(
            Vec::new(),
            result_sender,
            worker_thread_terminate_handles,
            n_workers,
        )
        .send_solution(current_insertions.to_vec());
        return auto_insertion_handle;
    }

    // Create n nodes
    let mut init_nodes: Vec<Node> = Vec::with_capacity(n_workers);

    // Create a node for each possible beginning
    for insertion in get_activity_beginnings_with_conflicts(
        static_data,
        current_insertions,
        current_insertions.len(),
    ) {
        init_nodes.push(Node::new(current_insertions.to_vec(), insertion));
    }

    // Generate nodes until there are enough of them
    while init_nodes.len() < n_workers {
        if init_nodes.is_empty() {
            // Init nodes have all been expanded - no solution is available
            NodePool::new(
                Vec::new(),
                result_sender,
                worker_thread_terminate_handles,
                n_workers,
            )
            .send_no_solution();
            return auto_insertion_handle;
        }

        // Check if any node has reached a solution
        if let Some(node_with_solution) = init_nodes
            .iter()
            .find(|node| n_activities_to_insert == node.current_insertions.len())
        {
            // All activities are inserted - return the solution
            NodePool::new(
                Vec::new(),
                result_sender,
                worker_thread_terminate_handles,
                n_workers,
            )
            .send_solution(node_with_solution.current_insertions.clone());
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
        init_nodes.extend(
            get_activity_beginnings_with_conflicts(
                static_data,
                &node_with_least_number_of_insertions.current_insertions,
                node_with_least_number_of_insertions
                    .current_insertions
                    .len(),
            )
            .into_iter()
            .map(|insertion| {
                Node::new(
                    node_with_least_number_of_insertions
                        .current_insertions
                        .to_vec(),
                    insertion,
                )
            }),
        );
    }

    // Keep exactly one node per worker and put the rest in the node_pool
    let node_pool = NodePool::new(
        init_nodes.split_off(n_workers),
        result_sender,
        worker_thread_terminate_handles.clone(),
        n_workers,
    );

    // Create workers and send them to their respective threads
    let arc_node_pool = Arc::new(Mutex::new(node_pool));
    for _ in 0..n_workers {
        let (exit_sender, exit_receiver) = mpsc::channel();
        let mut worker = Worker::new(
            static_data.to_vec(),
            arc_node_pool.clone(),
            // We made sure before that there were exactly enough init nodes
            vec![init_nodes.pop().expect("Popping out of empty vector")],
            exit_receiver,
        );

        worker_thread_terminate_handles
            .lock()
            .unwrap()
            .push(exit_sender);

        std::thread::spawn(move || {
            worker.work();
        });
    }

    auto_insertion_handle
}
