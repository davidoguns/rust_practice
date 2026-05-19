pub struct SearchConfig {
    file_path: String,
    search_term: String,
}

impl SearchConfig {
    pub fn new(file_path: &String, search_term: &String) -> SearchConfig {
        SearchConfig {
            file_path: String::from(file_path),
            search_term: String::from(search_term),
        }
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn search_term(&self) -> &String {
        &self.search_term
    }
}

pub struct SearchLineResult {}

#[cfg(test)]
mod tests {
    use super::*;

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
