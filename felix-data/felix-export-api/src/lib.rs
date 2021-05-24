use felix_collections::{Entity, Activity};

pub struct Pdf {
    pub title: String,
    pub font_size: i32,
    pub lines: Vec<String>,
}

impl Pdf {
    pub fn new(entity: String, activities: &[Activity]) -> Pdf {
        let res = Pdf { 
            title: entity.to_string(),
            font_size: 12,
            lines: Vec::new()
        };

        res
    }
}

pub fn generate_pdf(entity: String, activities: &[Activity], output_dir: &str) {
    // 1. Format data
    let pdf = Pdf::new(entity, &activities);

    // 2. Print data
    // TODO
}

#[cfg(test)]
mod tests;
