//! Module that defines the necessary CLI support functionality for the
//! minigrep binary crate. This is probably silly as it's own module in the
//! library crate and should be a part of the binary crate but we're learning
//! and going to overuse some features and organization as if this project
//! was much larger.

/// Configuration to use to represent a search configuration chosen through
/// some means.
pub struct SearchConfig {
    file_path: String,
    search_term: String,
    ignore_case: bool,
}

impl SearchConfig {
    pub fn new(file_path: &String, search_term: &String, ignore_case: bool) -> SearchConfig {
        SearchConfig {
            file_path: String::from(file_path),
            search_term: String::from(search_term),
            ignore_case,
        }
    }

    pub fn build(args: &[String]) -> Result<SearchConfig, &'static str> {
        let mut file_path = Option::<String>::None;
        let mut search_term = Option::<String>::None;
        let mut ignore_case = Option::<bool>::None;

        if std::env::var("IGNORE_CASE").is_ok() {
            ignore_case = Some(true)
        }
        
        let mut args_itr = args.iter().skip(1); //skip one because the first is the program name
        while let Some(arg) = args_itr.next() {
           match arg.as_str() {
               "-f" => {
                   if let Some(value) = args_itr.next() {
                       file_path = Some(value.clone())
                   }
               },
               "-s" => {
                   if let Some(value) = args_itr.next() {
                       search_term = Some(value.clone())
                   }
               },
               "-i" => { ignore_case = Some(true) },
               a => {
                   eprintln!("Unrecognized, and ignored CLI option found: {a}");
               }
           }
        }
        if file_path.is_some() &&  search_term.is_some() {
            Ok(Self::new(&file_path.unwrap(), &search_term.unwrap(), ignore_case.unwrap_or(false)))
        } else {
            Err("Insufficient CLI arguments specified. Must at least specify -s <search_term> and -f <file_path>")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_search_config() {
        let fp = String::from("foofile");
        let st = String::from("barterm");
        let search_config = SearchConfig::new(&fp, &st, false);
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
