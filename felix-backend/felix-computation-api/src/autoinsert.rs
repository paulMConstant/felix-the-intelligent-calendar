use crate::{
    compute_insertion_costs::get_activity_beginnings_with_conflicts,
    structs::{
        autoinsertion::{Node, Tree, Worker},
        ActivityBeginningMinutes, ActivityComputationStaticData,
    },
};

use std::sync::{mpsc, Arc, Mutex};

pub fn autoinsert(
    static_data: &[ActivityComputationStaticData],
    current_insertions: &[ActivityBeginningMinutes],
) -> mpsc::Receiver<Result<Vec<ActivityBeginningMinutes>, ()>> {
    let num_workers = (num_cpus::get() - 1).max(1);
    let (tx, rx) = mpsc::channel();
    let mut tree = Tree::new(tx);

    // Init N beginning nodes
    let mut init_nodes: Vec<Node> = Vec::new();

    while init_nodes.len() < num_workers {
        // Expand the last node to get more nodes
        // TODO this will bug in some cases ?
        let insertions = if let Some(node) = init_nodes.pop() {
            node.current_insertions
        } else {
            current_insertions.to_vec()
        };

        let nb_activities_inserted = insertions.len();

        for insertion in
            get_activity_beginnings_with_conflicts(static_data, &insertions, nb_activities_inserted)
        {
            let mut new_insertions = insertions.clone();
            new_insertions.push(insertion);
            init_nodes.push(Node::new(new_insertions));
        }
    }

    tree.unexplored_nodes = init_nodes.split_off(num_workers);

    let arc_tree = Arc::new(Mutex::new(tree));
    for _ in 0..num_workers {
        let mut worker = Worker::new(
            static_data.to_vec(),
            arc_tree.clone(),
            // We made sure before that there were exactly enough init nodes
            vec![init_nodes.pop().expect("Popping out of empty vector")],
        );

        std::thread::spawn(move || {
            worker.work();
        });
    }

    rx
}
