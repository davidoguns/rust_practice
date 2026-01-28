pub struct Book {
    title: String,
    author: String,
    year_published: i16,
    isbn: String,
}

impl Book {
    pub fn new(title: String, author: String, year_published: i16, isbn: String) -> Self {
        Self {
            title,
            author,
            year_published,
            isbn,
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn author(&self) -> &str {
        self.author.as_str()
    }

    pub fn year_published(&self) -> i16 {
        self.year_published
    }

    pub fn isbn(&self) -> &str {
        self.isbn.as_str()
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{{}, {}, {}, {}}}",
            self.title, self.author, self.year_published, self.isbn
        ))
    }
}
