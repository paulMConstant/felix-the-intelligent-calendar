mod components;
mod data_impl;
mod events;

use std::cell::RefCell;
use std::rc::Rc;

pub use events::Events;

use components::{
    activity::activities::Activities, entity::entities::Entities, group::groups::Groups,
    time::work_hours::WorkHours,
};

pub use components::{
    activity::Activity,
    activity::ActivityID,
    entity::Entity,
    group::Group,
    time::{
        time_interval::TimeInterval, Time, MIN_TIME_DISCRETIZATION, MIN_TIME_DISCRETIZATION_MINUTES,
    },
};

pub use data_impl::helpers::clean_string;

/// Stores, calculates and maintains coherency between entities, work hours and activities.
///
/// This is the only mutable object in the data module.
///
/// # Examples
///
/// Add, remove and modify work intervals :
///
/// ```
/// # use felix_backend::data::{Data, Time, TimeInterval};
/// let mut data = Data::new();
///
/// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
/// let afternoon_shift = TimeInterval::new(Time::new(14, 0), Time::new(18, 0));
///
/// // Intervals are automatically sorted, the order of addition does not matter
/// data.add_work_interval(afternoon_shift).unwrap();
/// data.add_work_interval(morning_shift).unwrap();
///
/// let work_hours = data.work_hours();
/// assert_eq!(work_hours[0], morning_shift);
/// assert_eq!(work_hours[1], afternoon_shift);
///
/// let new_morning_shift = TimeInterval::new(Time::new(9,0), Time::new(12, 0));
/// data.update_work_interval(morning_shift, new_morning_shift);
///
/// data.remove_work_interval(new_morning_shift).unwrap();
/// let work_hours = data.work_hours();
/// assert_eq!(work_hours.len(), 1);
/// ```
///
/// Add, remove and modify entities :
///
/// ```
/// # use felix_backend::data::{Data, Time, TimeInterval};
/// let mut data = Data::new();
///
/// let entity_name = data.add_entity("Bernard").unwrap();
///
/// let mail = "bernard@xyz.com";
/// data.set_entity_mail(entity_name.clone(), mail.clone()).unwrap();
///
/// let custom_morning_shift = TimeInterval::new(Time::new(10, 0), Time::new(12, 0));
/// data.add_custom_work_interval_for(entity_name.clone(), custom_morning_shift);
///
/// // new_name = "Jean" because set_entity_name formats it.
/// let new_name = data.set_entity_name(entity_name, "jean").unwrap();
///
/// let send_mail = true;
/// data.set_send_mail_to(new_name.clone(), send_mail).unwrap();
///
/// let entity = data.entity(new_name.clone()).unwrap();
///
/// assert!(entity.send_me_a_mail(), send_mail);
/// assert_eq!(entity.mail(), mail);
/// assert_eq!(entity.custom_work_hours()[0], custom_morning_shift);
///
/// data.remove_entity(new_name).unwrap();
/// assert!(data.entities_sorted().is_empty());
/// ```
///
/// Add, remove and modify activities :
///
/// ```
/// # use felix_backend::data::{Data, Time, TimeInterval};
/// let mut data = Data::new();
///
/// let activity_id = data.add_activity("My Activity").unwrap().id();
/// let entity_name = data.add_entity("My Entity").unwrap();
///
/// let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
/// data.add_work_interval(morning_shift).unwrap();
///
/// data.set_activity_duration(activity_id, Time::new(1, 0));
/// data.add_entity_to_activity(activity_id, entity_name);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    work_hours: WorkHours,
    entities: Entities,
    groups: Groups,
    activities: Activities,
    events: Rc<RefCell<Events>>,
}

impl Data {
    /// Creates a new data object.
    pub fn new() -> Data {
        Data {
            work_hours: WorkHours::new(),
            entities: Entities::new(),
            groups: Groups::new(),
            activities: Activities::new(),
            events: Rc::new(RefCell::new(Events::new())),
        }
    }
}
