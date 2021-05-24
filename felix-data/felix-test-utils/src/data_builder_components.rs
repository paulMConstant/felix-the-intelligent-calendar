use felix_data::{Time, MIN_TIME_DISCRETIZATION};

#[derive(Default)]
pub struct Group {
    pub name: &'static str,
    pub entities: Vec<&'static str>,
}

impl Group {
    #[must_use]
    pub fn default(name: &'static str) -> Group {
        Group {
            name,
            ..Default::default()
        }
    }
}

pub struct Activity {
    pub name: &'static str,
    pub duration: Time,
    pub entities: Vec<&'static str>,
    pub groups: Vec<&'static str>,
    pub insertion_time: Option<Time>,
}

impl Default for Activity {
    fn default() -> Activity {
        Activity {
            name: "Activity",
            duration: MIN_TIME_DISCRETIZATION,
            entities: Vec::new(),
            groups: Vec::new(),
            insertion_time: None,
        }
    }
}
impl Activity {
    #[must_use]
    pub fn default() -> Activity {
        Default::default()
    }
}
