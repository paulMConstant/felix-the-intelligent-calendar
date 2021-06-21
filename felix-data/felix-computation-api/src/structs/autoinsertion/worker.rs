use crate::{
    compute_insertion_costs::compute_insertion_costs,
    structs::{
        autoinsertion::NodePool,
        autoinsertion::{new_node, Node, NodesSortedByScore},
        ActivityComputationStaticData,
    },
};
use felix_datatypes::{Cost, InsertionCostsMinutes};

use std::collections::btree_map::Entry;
use std::sync::{mpsc, Arc, Mutex};

const N_ITER_BEFORE_SYNC: usize = 1000;

pub struct Worker {
    static_data: Vec<ActivityComputationStaticData>,
    pool: Arc<Mutex<NodePool>>,
    current_nodes: NodesSortedByScore,
    active: bool,
    n_iter: usize,
    most_activities_inserted: usize,

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
            most_activities_inserted: 0,

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
    fn sync_with_pool(&mut self) {
        let mut pool = self.pool.lock().unwrap();
        pool.merge_and_load_nodes(&mut self.current_nodes, &mut self.active);

        debug_assert!(self.most_activities_inserted <= pool.get_most_activities_inserted());
        self.most_activities_inserted = pool.get_most_activities_inserted();
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
        if let Some((cost_of_parent, node)) = self.current_nodes.node_with_lowest_cost() {
            // Current nodes is not empty: work
            let nb_activities_inserted = node.len();
            let nb_activities_to_insert = self.static_data.len();

            if nb_activities_inserted == nb_activities_to_insert {
                // All activities have been inserted. Yay !
                self.pool.lock().unwrap().send_complete_solution(node);
            } else {
                if nb_activities_inserted > self.most_activities_inserted {
                    // This is the furthest we got, send it and continue computing
                    self.pool
                        .lock()
                        .unwrap()
                        .send_partial_solution(node.clone());

                    // While we wait for the autoinsertion
                    self.most_activities_inserted = nb_activities_inserted;
                }

                let insertion_costs =
                    compute_insertion_costs(&self.static_data, &node, nb_activities_inserted);

                if let Some(min_insertion_cost) = insertion_costs
                    .iter()
                    .min_by_key(|insertion_cost| insertion_cost.cost)
                {
                    if min_insertion_cost.cost == 0 {
                        // The best insertion slot does not bother any activity. We will not get better
                        // results with the others => discard them
                        self.insert_node_into_current_nodes(
                            node.clone(),
                            *min_insertion_cost,
                            cost_of_parent,
                        );
                    } else {
                        for insertion_cost in insertion_costs {
                            self.insert_node_into_current_nodes(
                                node.clone(),
                                insertion_cost,
                                cost_of_parent,
                            );
                        }
                    }
                }
            }
        } else {
            // Current nodes is empty, fetch from pool
            self.sync_with_pool();
        }
    }

    fn insert_node_into_current_nodes(
        &mut self,
        current_node: Node,
        insertion_cost: InsertionCostsMinutes,
        cost_of_parent: Cost,
    ) {
        match self
            .current_nodes
            .entry((insertion_cost.cost + cost_of_parent) / current_node.len())
        {
            Entry::Vacant(entry) => {
                entry.insert(vec![new_node(
                    current_node,
                    insertion_cost.beginning_minutes,
                )]);
            }
            Entry::Occupied(mut entry) => {
                entry
                    .get_mut()
                    .push(new_node(current_node, insertion_cost.beginning_minutes));
            }
        }
    }
}
