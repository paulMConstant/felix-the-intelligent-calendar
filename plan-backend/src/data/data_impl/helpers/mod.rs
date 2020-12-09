use crate::errors::{Result, empty_name::EmptyName};

/// Removes multiple spaces and fix caps.
///
/// # Errors
///
/// Returns Err if the formatted string is empty.
#[must_use]
pub fn clean_string<S>(s: S) -> Result<String>
where
    S: Into<String>,
{
    let s = s
        .into()
        .split_whitespace()
        .map(|s| fix_case(s))
        .collect::<Vec<String>>()
        .join(" ");
    if s.is_empty() {
        Err(EmptyName::new())
    } else {
        Ok(s)
    }
}

/// Turns the first letter to uppercase and last to lowercase.
#[must_use]
fn fix_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase()
                    .next()
                    .expect("next() failed while iterating with known size")
            } else {
                c.to_lowercase()
                    .next()
                    .expect("next() failed while iterating with known size")
            }
        })
        .collect::<String>()
}
