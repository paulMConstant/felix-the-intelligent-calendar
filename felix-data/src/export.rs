use crate::Data;

use felix_collections::PrintableActivities;
use felix_export_api::generate_pdf;

use std::path::PathBuf;

/// Functions related to exporting current data.
impl Data {
    pub fn export_as_pdf(&self, output_dir: PathBuf) {
        let printable_data = self.as_printable();

        for (entity, activities) in printable_data {
            generate_pdf(entity, activities, output_dir.clone());
        }
    }

    pub fn as_printable(&self) -> PrintableActivities {
        let mut res = PrintableActivities::new();
        for entity in self.entities_sorted() {
            let activities_of_entity = self
                .activities_of(entity.name())
                .unwrap_or_else(|_| panic!("Could not get the activities of {}", entity.name()));
            res.insert(entity.name(), activities_of_entity);
        }
        res
    }
}
