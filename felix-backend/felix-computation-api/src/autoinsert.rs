use crate::{
    structs::{
        autoinsertion::{Tree, Worker},
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

    if let Some(mut init_nodes) = tree.init_with_n_nodes(num_workers,
                                                         static_data,
                                                         current_insertions) {
        // Exactly num_workers init nodes generated succesfuly
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
    }

    rx
}
