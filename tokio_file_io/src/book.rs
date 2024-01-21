
pub struct Book {
    title: String,
    author: String,
    year_published: i16,
    isbn: String
}

impl Book {
    pub fn new(title: String, author: String, year_published: i16, isbn: String) -> Self {
        Book { title, author, year_published, isbn }
    } 

    fn validate(self) -> bool {
        true
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{{}, {}, {}, {}}}", self.title, self.author, self.year_published, self.isbn))
    }
}
