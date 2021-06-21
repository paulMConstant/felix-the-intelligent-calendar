use std::collections::{btree_map::Entry, BTreeMap};

use felix_datatypes::{ActivityBeginningMinutes, Cost};

pub type Node = Vec<ActivityBeginningMinutes>;

//#[derive(Debug)]
//pub struct Node {
//    pub current_insertions: Vec<ActivityBeginningMinutes>,
//}
//
//impl Node {
//    #[must_use]
//    pub fn new(
//        mut current_insertions: Vec<ActivityBeginningMinutes>,
//        next_insertion: ActivityBeginningMinutes,
//    ) -> Node {
//        current_insertions.push(next_insertion);
//        Node { current_insertions }
//    }
//}

#[must_use]
pub fn new_node(mut current_insertions: Node, next_insertion: ActivityBeginningMinutes) -> Node {
    current_insertions.push(next_insertion);
    current_insertions
}

/// A wrapper around BTreeMap to keep nodes sorted in ascending cost order.
#[derive(Debug)]
pub struct NodesSortedByScore {
    pub nodes: BTreeMap<Cost, Vec<Node>>,
}

impl NodesSortedByScore {
    pub fn new(init_nodes: Vec<(Cost, Node)>) -> NodesSortedByScore {
        let mut nodes = BTreeMap::new();
        for (cost, node) in init_nodes {
            match nodes.entry(cost) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![node]);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(node);
                }
            }
        }

        NodesSortedByScore { nodes }
    }

    /// Merges all nodes of other into self, leaving other empty.
    pub fn merge_append(&mut self, other: &mut NodesSortedByScore) {
        // Remove each node at a time from the other map and append
        // the nodes into our map
        while let Some((key_cost, mut value_nodes)) = other.nodes.pop_first() {
            match self.nodes.entry(key_cost) {
                Entry::Vacant(entry) => {
                    entry.insert(value_nodes);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().append(&mut value_nodes);
                }
            }
        }
    }

    /// Returns the first node with the lowest cost.
    /// If there are no nodes, return None.
    pub fn node_with_lowest_cost(&mut self) -> Option<(Cost, Node)> {
        self.nodes.first_entry().and_then(|mut entry| {
            let cost = *entry.key();
            let nodes = entry.get_mut();

            // Take the last node in the list
            let result = nodes.pop().map(|node| (cost, node));

            // Remove the entry associated to the cost if there are no more nodes
            if nodes.is_empty() {
                entry.remove();
            }
            result
        })
    }

    /// Inserts the given nodes with given cost into the map.
    /// Simply forwards the call to map.insert().
    #[inline]
    pub fn insert(&mut self, key_cost: Cost, value_nodes: Vec<Node>) {
        self.nodes.insert(key_cost, value_nodes);
    }

    /// Returns the entry for the given cost.
    /// Simply forwards the call to map.entry().
    #[inline]
    #[must_use]
    pub fn entry(&mut self, key_cost: Cost) -> Entry<Cost, Vec<Node>> {
        self.nodes.entry(key_cost)
    }
}
