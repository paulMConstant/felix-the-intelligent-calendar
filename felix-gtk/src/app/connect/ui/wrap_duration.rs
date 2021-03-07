use felix_backend::data::Time;

/// When the minutes skip from 0 to 55 or the inverse, removes or adds an hour
/// and returns the new result.
///
/// # Example
///
/// ```
///
/// let (old, new) = (Time::new(1, 0), Time::new(1, 55));
/// assert_eq!(wrap_duration(old, new), Time::new(0, 55));

/// let (old, new) = (Time::new(0, 55), Time::new(0, 0));
/// assert_eq!(wrap_duration(old, new), Time::new(1, 0));
/// ```
pub fn wrap_duration(old_duration: Time, new_duration: Time) -> Time {
    // Wrap if the duration goes from 0:55 to 0 or from 0 to 0:55
    if old_duration.minutes() == 55 && new_duration.minutes() == 0 {
        // Duration goes up from 0:55 to 1:00
        Time::new((new_duration.hours() + 1).min(23), new_duration.minutes())
    } else if old_duration.minutes() == 0 && new_duration.minutes() == 55 {
        // Duration goes down from 1:00 to 0:55
        Time::new((new_duration.hours() - 1).max(0), new_duration.minutes())
    } else {
        // No wrap
        new_duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_duration() {
        let (old, new) = (Time::new(1, 0), Time::new(1, 55));
        assert_eq!(wrap_duration(old, new), Time::new(0, 55));

        let (old, new) = (Time::new(0, 55), Time::new(0, 0));
        assert_eq!(wrap_duration(old, new), Time::new(1, 0));

        let (old, new) = (Time::new(0, 50), Time::new(0, 55));
        assert_eq!(wrap_duration(old, new), Time::new(0, 55));
    }
}
