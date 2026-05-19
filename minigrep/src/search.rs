pub struct SearchConfig {
    file_path: String,
    search_term: String,
}

impl SearchConfig {
    pub fn new(file_path: String, search_term: String) -> SearchConfig {
        SearchConfig {
            file_path: String::from(file_path),
            search_term: String::from(search_term),
        }
    }
}

pub struct SearchLineResult {}
