/// Removes multiple spaces and fix caps.
///
/// # Errors
///
/// Returns Err if the formatted string is empty.
#[must_use]
pub fn clean<S>(s: S) -> Result<String, String>
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
        Err("The formatted name is empty.".to_owned())
    } else {
        Ok(s)
    }
}

/// Turns the first letter to uppercase and last to lowercase.
#[must_use]
fn fix_case<S>(s: S) -> String
where
    S: Into<String>,
{
    s.into()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c.to_lowercase().next().unwrap()
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean() {
        let s = "LauREnt \t  outang";
        let expected = String::from("Laurent Outang");
        assert_eq!(clean(s).unwrap(), expected);

        let s = " \t";
        assert!(clean(s).is_err());
    }

    #[test]
    fn test_fix_case() {
        assert_eq!(fix_case("sTrinG"), "String".to_owned());
    }
}
