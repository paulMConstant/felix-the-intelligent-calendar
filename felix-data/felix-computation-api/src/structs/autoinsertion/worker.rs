use crate::{
    compute_insertion_costs::compute_insertion_costs,
    structs::{
        autoinsertion::NodePool,
        autoinsertion::{Node, NodesSortedByScore},
        ActivityComputationStaticData,
    },
};
use felix_datatypes::Cost;

use std::collections::btree_map::Entry;
use std::sync::{mpsc, Arc, Mutex};

const N_ITER_BEFORE_SYNC: usize = 1000;

pub struct Worker {
    static_data: Vec<ActivityComputationStaticData>,
    pool: Arc<Mutex<NodePool>>,
    current_nodes: NodesSortedByScore,
    active: bool,
    n_iter: usize,

    exit_receiver: mpsc::Receiver<()>,
}

impl Worker {
    pub fn new(
        static_data: Vec<ActivityComputationStaticData>,
        pool: Arc<Mutex<NodePool>>,
        current_nodes: Vec<(Cost, Node)>,
        exit_receiver: mpsc::Receiver<()>,
    ) -> Worker {
        Worker {
            static_data,
            pool,
            current_nodes: NodesSortedByScore::new(current_nodes),
            active: true,
            n_iter: 0,

            exit_receiver,
        }
    }

    pub fn work(&mut self) {
        // Exit as soon as we get data
        while self.exit_receiver.try_recv().is_err() {
            self.expand_node();
            self.n_iter += 1;
            if self.n_iter > N_ITER_BEFORE_SYNC {
                self.try_sync_with_pool();
            }
        }
    }

    /// Updates the pool and fetches a new node to explore.
    /// If the pool is locked, this operation is skipped.
    fn sync_with_pool(&mut self) {
        self.pool
            .lock()
            .unwrap()
            .merge_and_load_nodes(&mut self.current_nodes, &mut self.active);
        self.n_iter = 0;
    }

    /// Updates the pool and fetches a new node to explore.
    /// If the pool is locked, this operation is skipped.
    fn try_sync_with_pool(&mut self) {
        if let Ok(mut pool) = self.pool.try_lock() {
            pool.merge_and_load_nodes(&mut self.current_nodes, &mut self.active);
            self.n_iter = 0;
        };
    }

    /// Expands the nodes with the lowest cost.
    fn expand_node(&mut self) {
        if let Some((_key_cost, node)) = self.current_nodes.node_with_lowest_cost() {
            //let now = std::time::Instant::now();
            //println!("Expanding {} nodes with cost {}...", value_nodes.len(), _key_cost);

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
                for insertion_cost in compute_insertion_costs(
                    &self.static_data,
                    &node.current_insertions,
                    nb_activities_inserted,
                ) {
                    match self.current_nodes.entry(insertion_cost.cost) {
                        Entry::Vacant(entry) => {
                            entry.insert(vec![Node::new(
                                node.current_insertions.clone(),
                                insertion_cost.beginning_minutes,
                            )]);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push(Node::new(
                                node.current_insertions.clone(),
                                insertion_cost.beginning_minutes,
                            ));
                        }
                    }
                }
            }
            //println!("Took {:?} to expand best nodes", now.elapsed().as_millis());
        } else {
            // Current nodes is empty, fetch from pool
            //let now = std::time::Instant::now();
            self.sync_with_pool();
            //println!("Took {:?} to sync with pool", now.elapsed().as_millis());
        }
    }
}
