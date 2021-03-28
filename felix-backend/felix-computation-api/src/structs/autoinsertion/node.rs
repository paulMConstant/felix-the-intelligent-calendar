use crate::structs::ActivityBeginningMinutes;

#[derive(Debug)]
pub struct Node {
    pub current_insertions: Vec<ActivityBeginningMinutes>,
}

impl Node {
    #[must_use]
    pub fn new(current_insertions: Vec<ActivityBeginningMinutes>) -> Node {
        Node { current_insertions }
    }
}
