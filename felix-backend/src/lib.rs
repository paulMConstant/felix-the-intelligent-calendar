//! # Felix
//!
//! Scheduling made easy.
//!
//! Felix helps schedule multiple activities involving many people, on a daily rate,
//! with 5-minute precision.
//!
//! Use cases include :
//! * Schools
//! * Summer camps
//! * Any organization with resources to manage (rooms, meetings...)

extern crate num_cpus;

/// Backend module which performs calculations to generate schedules and prevent errors.
pub mod data;

/// Errors used in data.
pub mod errors;

pub use felix_datatypes::{Time, MIN_TIME_DISCRETIZATION};
