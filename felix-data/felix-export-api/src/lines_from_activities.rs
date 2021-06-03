use crate::Line;

use felix_collections::Activity;

pub(crate) fn extract_lines_from_activities(mut activities: Vec<Activity>) -> Vec<Line> {
    // Do not mention not inserted activities
    activities = activities
        .into_iter()
        .filter(|activity| activity.insertion_interval().is_some())
        .collect();

    // Earlier activities go first
    activities.sort_by(|a, b| {
        a.insertion_interval()
            .expect("Exporting uninserted activities")
            .cmp(
                &b.insertion_interval()
                    .expect("Exporting uninserted activities"),
            )
    });

    activities
        .iter()
        .map(|activity| Line {
            timestamp: activity
                .insertion_interval()
                .expect("Exporting uninserted activity")
                .to_string()
                + " : ",
            activity_name_and_participants: activity.name()
                + " ("
                + &activity.entities_sorted().join(", ")
                + ")",
        })
        .collect()
}
