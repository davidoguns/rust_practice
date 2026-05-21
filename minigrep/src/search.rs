//! Module defines the core search functions that enable programmatic
//! access to grep-like functionality. Rust library for direct integration
//! or use the co-located binary as a standalone frontend app.

/// Case sensitive search function.
///
/// # Examples
///
/// ```
/// use minigrep::search::search;
///
/// let query = String::from("duct");
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
/// assert_eq!(vec![(1, "safe, fast, productive.")],
///     search(query.as_str(), contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut matches = Vec::<(usize, &str)>::new();
    for (line_no, line) in contents.lines().enumerate() {
        if line.contains(query) {
            matches.push((line_no, line));
        }
    }
    matches
}

/// Case insensitive search function.
///
/// # Examples
///
/// ```
/// use minigrep::search::search_insensitive;
/// let query = String::from("rUsT");
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
/// assert_eq!(vec![(0, "Rust:"), (3, "Trust me.")],
///     search_insensitive(query.as_str(), contents));
/// ```
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut matches = Vec::<(usize, &str)>::new();
    for (line_no, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(query.as_str()) {
            matches.push((line_no, line));
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_one_result() {
        let query = String::from("duct");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search(query.as_str(), contents)
        );
    }

    #[test]
    pub fn test_case_sensitive() {
        let query = String::from("duct");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search(query.as_str(), contents)
        );
    }

    #[test]
    pub fn test_case_insensitive() {
        let query = String::from("rUsT");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec![(0, "Rust:"), (3, "Trust me.")],
            search_insensitive(query.as_str(), contents)
        );
    }
}
