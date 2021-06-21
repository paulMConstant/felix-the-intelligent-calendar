use crate::{
    compute_insertion_costs,
    structs::{
        autoinsertion::{new_node, AutoinsertionThreadHandle, Node, NodePool, Worker},
        ActivityComputationStaticData,
    },
};
use felix_datatypes::{ActivityBeginningMinutes, Cost};

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
        // If no one is listening, it is fine, we just return as if nothing happened
        let _ = result_sender.send(Some(current_insertions.to_vec()));
        return auto_insertion_handle;
    }

    // Create n nodes
    let mut init_nodes: Vec<(Cost, Node)> = Vec::with_capacity(n_workers);

    // Create a node for each possible beginning
    for insertion_cost in
        compute_insertion_costs(static_data, current_insertions, current_insertions.len())
    {
        init_nodes.push((
            insertion_cost.cost,
            new_node(
                current_insertions.to_vec(),
                insertion_cost.beginning_minutes,
            ),
        ));
    }

    // Generate nodes until there are enough of them
    while init_nodes.len() < n_workers {
        if init_nodes.is_empty() {
            // Init nodes have all been expanded - no solution is available
            // If no one is listening, it is fine, we just return as if nothing happened
            let _ = result_sender.send(None);
            return auto_insertion_handle;
        }

        // Check if any node has reached a solution
        if let Some(node_with_solution) = init_nodes
            .iter()
            .map(|(_cost, node)| node)
            .find(|&node| n_activities_to_insert == node.len())
        {
            // All activities are inserted - return the solution
            // If no one is listening, it is fine, we just return as if nothing happened
            let _ = result_sender.send(Some(node_with_solution.clone()));
            return auto_insertion_handle;
        }

        // Expand the node with the least number of inserted activities
        let (index_node_with_least_number_of_insertions, _) = init_nodes
            .iter()
            .map(|(_cost, node)| node)
            .enumerate()
            .min_by_key(|(_index, node)| node.len())
            .expect("Taking min of empty vec");

        let (_cost, node_with_least_number_of_insertions) =
            init_nodes.swap_remove(index_node_with_least_number_of_insertions);

        // Create a node for each possible beginning
        init_nodes.extend(
            compute_insertion_costs(
                static_data,
                &node_with_least_number_of_insertions,
                node_with_least_number_of_insertions.len(),
            )
            .into_iter()
            .map(|insertion_cost| {
                (
                    insertion_cost.cost,
                    new_node(
                        node_with_least_number_of_insertions.clone(),
                        insertion_cost.beginning_minutes,
                    ),
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
