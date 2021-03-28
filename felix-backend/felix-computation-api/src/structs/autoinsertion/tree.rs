use crate::structs::{autoinsertion::Node, ActivityBeginningMinutes};

use std::sync::mpsc;

const N_NODES_TO_FETCH_WHEN_SYNCING_WITH_TREE: usize = 5;
const N_NODES_TO_LEAVE_IN_THE_TREE: usize = 16;

/// A destructive tree structure shared among all workers.
pub struct Tree {
    pub unexplored_nodes: Vec<Node>,

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

    pub fn merge_and_load_best_nodes(&mut self, nodes: &mut Vec<Node>) {
        // Add nodes to the tree
        self.unexplored_nodes.append(nodes);
        let tree_len = self.unexplored_nodes.len();

        if tree_len == 0 {
            // TODO if n_nodes_to_fetch = 0 ?
        } else {
            // Sort the tree
            //self.unexplored_nodes.sort_unstable_by_key(|node| std::cmp::Reverse(node.cost));

            // Fetch the best nodes to expand it
            let n_nodes_to_fetch = if tree_len > N_NODES_TO_LEAVE_IN_THE_TREE {
                N_NODES_TO_FETCH_WHEN_SYNCING_WITH_TREE.min(tree_len - N_NODES_TO_LEAVE_IN_THE_TREE)
            } else {
                1
            };

            *nodes = self.unexplored_nodes.split_off(tree_len - n_nodes_to_fetch);
        }
    }
}
