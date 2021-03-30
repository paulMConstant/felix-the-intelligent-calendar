use crate::{
    compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::{
        autoinsertion::node::Node, autoinsertion::tree::Tree, ActivityComputationStaticData,
    },
};

use std::sync::{Arc, Mutex, MutexGuard};

pub struct Worker {
    static_data: Vec<ActivityComputationStaticData>,
    tree: Arc<Mutex<Tree>>,
    current_nodes: Vec<Node>,
    exit: bool,
}

impl Worker {
    pub fn new(
        static_data: Vec<ActivityComputationStaticData>,
        tree: Arc<Mutex<Tree>>,
        current_nodes: Vec<Node>,
    ) -> Worker {
        Worker {
            static_data,
            tree,
            current_nodes,
            exit: false,
        }
    }

    pub fn work(&mut self) {
        while !self.exit {
            self.expand_best_node();
            self.try_sync_with_tree();
        }
    }

    /// Updates the tree and fetches a new node to explore.
    /// If the tree is locked, this operation is skipped.
    fn sync_with_tree(&mut self) {
        let tree = self.tree.clone();
        let tree = tree.lock().unwrap();
        self.update_tree_and_fetch_new_node(tree);
    }

    /// Updates the tree and fetches a new node to explore.
    /// If the tree is locked, this operation is skipped.
    fn try_sync_with_tree(&mut self) {
        if let Ok(tree) = self.tree.clone().try_lock() {
            self.update_tree_and_fetch_new_node(tree);
        };
    }

    fn update_tree_and_fetch_new_node(&mut self, mut tree: MutexGuard<Tree>) {
        if tree.solution_found() {
            self.exit = true;
        } else {
            tree.merge_and_load_best_nodes(&mut self.current_nodes);
        }
    }

    fn expand_best_node(&mut self) {
        // Find best node
        // If only one activity remains, cost will be zero, so we will find it fast
        if let Some(best_node) = self.current_nodes.pop() {
            // Current nodes is not empty: work
            let nb_activities_inserted = best_node.current_insertions.len();
            let nb_activities_to_insert = self.static_data.len();

            if nb_activities_inserted == nb_activities_to_insert {
                // All activities have been inserted. Yay !
                self.tree
                    .lock()
                    .unwrap()
                    .send_solution(best_node.current_insertions);
            } else {
                for insertion in get_activity_beginnings_with_conflicts(
                    &self.static_data,
                    &best_node.current_insertions,
                    nb_activities_inserted,
                ) {
                    // Create a node for each possible beginning
                    self.current_nodes.push(
                        Node::new(best_node.current_insertions.clone(), insertion)
                    );
                }
            }
        } else {
            // Current nodes is empty, fetch from tree
            self.sync_with_tree();
        }
    }
}
