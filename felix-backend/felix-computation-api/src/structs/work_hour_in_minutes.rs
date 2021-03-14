#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WorkHourInMinutes {
    pub beginning: u16,
    pub end: u16,
}

impl WorkHourInMinutes {
    pub fn new(beginning: u16, end: u16) -> WorkHourInMinutes {
        WorkHourInMinutes { beginning, end }
    }

    pub fn duration(&self) -> u16 {
        self.end - self.beginning
    }
}

