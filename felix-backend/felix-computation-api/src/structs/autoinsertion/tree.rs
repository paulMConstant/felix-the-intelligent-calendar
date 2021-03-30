use crate::{
    compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::{autoinsertion::Node, ActivityBeginningMinutes, ActivityComputationStaticData}
};

use std::sync::mpsc;

const N_NODES_TO_FETCH_WHEN_SYNCING_WITH_TREE: usize = 5;
const N_NODES_TO_LEAVE_IN_THE_TREE: usize = 16;

/// A destructive tree structure shared among all workers.
pub struct Tree {
    unexplored_nodes: Vec<Node>,

    // We stop at the first solution
    solution_found: bool,
    result_sender: mpsc::Sender<Result<Vec<ActivityBeginningMinutes>, ()>>,
}

impl Tree {
    #[must_use]
    pub fn new(result_sender: mpsc::Sender<Result<Vec<ActivityBeginningMinutes>, ()>>) -> Tree {
        Tree {
            unexplored_nodes: Vec::new(),
            solution_found: false,
            result_sender,
        }
    }

    /// From the current insertions, creates nodes and returns exactly n nodes.
    /// If n\_nodes cannot be created, returns None, and:
    /// * If the result is found directly, sends it.
    /// * If no result is found, sends an error message.
    #[must_use]
    pub fn init_with_n_nodes(
        &mut self,
        n_nodes: usize,
        static_data: &[ActivityComputationStaticData],
        current_insertions: &[ActivityBeginningMinutes],
    ) -> Option<Vec<Node>>
    {
        let n_activities_to_insert = static_data.len();

        if n_activities_to_insert == current_insertions.len() {
            // All activities are inserted - return the solution
            self.send_solution(current_insertions.to_vec());
            return None;
        }

        // Init n nodes
        let mut init_nodes: Vec<Node> = Vec::with_capacity(n_nodes);
    
        // Create a node for each possible beginning
        for insertion in get_activity_beginnings_with_conflicts(
            static_data,
            current_insertions,
            current_insertions.len()
        ) {
            init_nodes.push(
                Node::new(current_insertions.to_vec(), insertion)
            );
        }

        while init_nodes.len() < n_nodes {
            if init_nodes.is_empty() {
                // Init nodes have all been expanded - no solution is available
                self.send_no_solution();
                return None;
            }

            // Check if any node has reached a solution
            if let Some(node_with_solution) = init_nodes
                .iter()
                .find(|node| n_activities_to_insert == node.current_insertions.len()) {
                // All activities are inserted - return the solution
                self.send_solution(node_with_solution.current_insertions.clone());
                return None;
            }

            // Expand the node with the least number of inserted activities
            let (index_node_with_least_number_of_insertions, _) = init_nodes
                .iter()
                .enumerate()
                .min_by_key(|(_index, node)| node.current_insertions.len())
                .expect("Taking min of empty vec");

            let node_with_least_number_of_insertions = init_nodes
                .swap_remove(index_node_with_least_number_of_insertions);

            // Create a node for each possible beginning
            for insertion in get_activity_beginnings_with_conflicts(
                static_data,
                &node_with_least_number_of_insertions.current_insertions,
                node_with_least_number_of_insertions.current_insertions.len()
            ) {
                init_nodes.push(Node::new(
                    node_with_least_number_of_insertions.current_insertions.to_vec(),
                    insertion
                ));
            }
        }

        // Cut off init nodes to return exactly n nodes
        self.unexplored_nodes = init_nodes.split_off(n_nodes);

        Some(init_nodes)
    }

    #[must_use]
    #[inline]
    pub fn solution_found(&self) -> bool {
        self.solution_found
    }

    pub fn send_solution(&mut self, solution: Vec<ActivityBeginningMinutes>) {
        if !self.solution_found {
            self.solution_found = true;
            self.result_sender
                .send(Ok(solution))
                .expect("No thread to receive autoinsertion solution");
        }
    }

    pub fn send_no_solution(&self) {
        self.result_sender
            .send(Err(()))
            .expect("No thread to receive autoinsertion solution");
    }

    pub fn merge_and_load_best_nodes(&mut self, nodes: &mut Vec<Node>) {
        // Add nodes to the tree
        self.unexplored_nodes.append(nodes);
        let tree_len = self.unexplored_nodes.len();

        if tree_len == 0 {
            // TODO if n_nodes_to_fetch = 0 ?
        } else {
            let n_nodes_to_fetch = if tree_len > N_NODES_TO_LEAVE_IN_THE_TREE {
                N_NODES_TO_FETCH_WHEN_SYNCING_WITH_TREE.min(tree_len - N_NODES_TO_LEAVE_IN_THE_TREE)
            } else {
                1
            };

            *nodes = self.unexplored_nodes.split_off(tree_len - n_nodes_to_fetch);
        }
    }
}
