pub(crate) type LineIndex = usize;
pub(crate) type LineSplits = Vec<LineIndex>;

pub(crate) struct Line {
    pub timestamp: String,
    pub activity_name_and_participants: String,
}

impl Line {
    pub fn print(&self) -> String {
        self.timestamp.clone() + &self.activity_name_and_participants
    }
}

