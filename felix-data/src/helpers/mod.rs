use crate::errors::{empty_name::EmptyName, Result};

/// Removes multiple spaces and fixes caps.
///
/// # Errors
///
/// Returns Err if the formatted string is empty.
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
/// Turns any letter following a dash into uppercase.
#[must_use]
fn fix_case(s: &str) -> String {
    let mut last_char = 'x'; // Random character which does not cause uppercase-ing
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let res = if i == 0 || last_char == '-' || last_char == '_' {
                c.to_uppercase()
                    .next()
                    .expect("next() failed while iterating with known size")
            } else {
                c.to_lowercase()
                    .next()
                    .expect("next() failed while iterating with known size")
            };
            last_char = c;
            res
        })
        .collect::<String>()
}
