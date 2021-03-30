use crate::structs::ActivityBeginningMinutes;

#[derive(Debug)]
pub struct Node {
    pub current_insertions: Vec<ActivityBeginningMinutes>,
}

impl Node {
    #[must_use]
    pub fn new(mut current_insertions: Vec<ActivityBeginningMinutes>,
               next_insertion: ActivityBeginningMinutes) -> Node {
        current_insertions.push(next_insertion);
        Node { current_insertions }
    }
}
