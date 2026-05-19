pub struct SearchConfig {
    file_path: String,
    search_term: String,
    ignore_case: bool,
}

impl SearchConfig {
    pub fn new(file_path: &String, search_term: &String) -> SearchConfig {
        SearchConfig {
            file_path: String::from(file_path),
            search_term: String::from(search_term),
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        }
    }

    pub fn build(args: &[String]) -> Result<SearchConfig, &'static str> {
        match args.len() {
            3 => Ok(SearchConfig::new(&args[2], &args[1])),
            _ => Err("Must specify exactly 2 CLI arguments! minigrep <search> <file>"),
        }
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn search_term(&self) -> &String {
        &self.search_term
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut matches = Vec::<(usize, &str)>::new();
    for (line_no, line) in contents.lines().enumerate() {
        if line.contains(query) {
            matches.push((line_no, line));
        }
    }
    matches
}

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
        assert_eq!(vec![(1, "safe, fast, productive.")],
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
        assert_eq!(vec![(1, "safe, fast, productive.")],
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
        assert_eq!(vec![(0, "Rust:"), (3, "Trust me.")],
            search_insensitive(query.as_str(), contents)
        );
    }

    #[test]
    pub fn test_search_config() {
        let fp = String::from("foofile");
        let st = String::from("barterm");
        let search_config = SearchConfig::new(&fp, &st);
        assert!(
            fp.cmp(search_config.file_path()) == std::cmp::Ordering::Equal,
            "Expected filepath to be [{}], found [{}]",
            fp,
            search_config.file_path()
        );
        assert!(
            st.cmp(search_config.search_term()) == std::cmp::Ordering::Equal,
            "Expected search term to be [{}], found [{}]",
            st,
            search_config.search_term()
        );
    }
}
