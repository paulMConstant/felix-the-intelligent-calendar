/// A simple struct holding the beginning of an activity and the cost it gets.
/// The higher the cost, the more the activtiy blocks other activities.
#[derive(Debug, Clone, Copy)]
pub struct InsertionCosts {
    pub beginning_minutes: u16,
    pub cost: u16,
}
