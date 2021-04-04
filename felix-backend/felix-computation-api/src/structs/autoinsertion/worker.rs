use crate::{
    compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::{
        autoinsertion::node::Node, autoinsertion::node_pool::NodePool,
        ActivityComputationStaticData,
    },
};

use std::sync::{mpsc, Arc, Mutex};

pub struct Worker {
    static_data: Vec<ActivityComputationStaticData>,
    pool: Arc<Mutex<NodePool>>,
    current_nodes: Vec<Node>,
    active: bool,

    exit_receiver: mpsc::Receiver<()>,
}

impl Worker {
    pub fn new(
        static_data: Vec<ActivityComputationStaticData>,
        pool: Arc<Mutex<NodePool>>,
        current_nodes: Vec<Node>,
        exit_receiver: mpsc::Receiver<()>,
    ) -> Worker {
        Worker {
            static_data,
            pool,
            current_nodes,
            active: true,

            exit_receiver,
        }
    }

    pub fn work(&mut self) {
        // Exit as soon as we get data
        while self.exit_receiver.try_recv().is_err() {
            self.expand_node();
            self.try_sync_with_pool();
        }
    }

    /// Updates the pool and fetches a new node to explore.
    /// If the pool is locked, this operation is skipped.
    fn sync_with_pool(&mut self) {
        self.pool
            .lock()
            .unwrap()
            .merge_and_load_nodes(&mut self.current_nodes, &mut self.active);
    }

    /// Updates the pool and fetches a new node to explore.
    /// If the pool is locked, this operation is skipped.
    fn try_sync_with_pool(&mut self) {
        if let Ok(mut pool) = self.pool.try_lock() {
            pool.merge_and_load_nodes(&mut self.current_nodes, &mut self.active);
        };
    }

    fn expand_node(&mut self) {
        if let Some(node) = self.current_nodes.pop() {
            // Current nodes is not empty: work
            let nb_activities_inserted = node.current_insertions.len();
            let nb_activities_to_insert = self.static_data.len();

            if nb_activities_inserted == nb_activities_to_insert {
                // All activities have been inserted. Yay !
                self.pool
                    .lock()
                    .unwrap()
                    .send_solution(node.current_insertions);
            } else {
                self.current_nodes.extend(
                    get_activity_beginnings_with_conflicts(
                        &self.static_data,
                        &node.current_insertions,
                        nb_activities_inserted,
                    )
                    .into_iter()
                    .map(|insertion| Node::new(node.current_insertions.clone(), insertion)),
                );
            }
        } else {
            // Current nodes is empty, fetch from pool
            self.sync_with_pool();
        }
    }
}
