use std::{collections::LinkedList, io};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use serde::{Serialize, Deserialize};

use crate::book::Book;

pub trait BookStorage {
    fn save_books_json(&self, book_db: &LinkedList<Book>) -> Result<(), std::io::Error>;
    fn save_books(&self, book_db: &LinkedList<Book>) -> Result<(), std::io::Error>;
    fn load_books(&self, book_db: &mut LinkedList<Book>) -> Result<(), std::io::Error>;
}

pub(crate) enum StorageFormat {
    Json,
    Text
}

pub struct BookDiskStorage {
    format: StorageFormat,
    tokio_rt: tokio::runtime::Runtime,
    filepath: String,
}

impl BookDiskStorage {
    pub(crate) fn new(trt: tokio::runtime::Runtime, file: &str) -> Self {
        Self { format: StorageFormat::Json, tokio_rt : trt, filepath: String::from(file) }
    }
}

impl BookStorage for BookDiskStorage {
    fn save_books_json(&self, book_db: &LinkedList<Book>) -> Result<(), std::io::Error> {
        use tokio::fs::File;
        return self.tokio_rt.block_on(async {
            let mut json_filepath = self.filepath.to_owned();
            json_filepath.push_str(".json");
            let mut file = File::create(&json_filepath).await?;
            match serde_json::to_string_pretty(&book_db) {
                Ok(serialized) => {
                    file.write(serialized.as_bytes()).await?; 
                    file.flush().await?;
                    Ok(())
                },
                Err(_e) => {
                   return Err(std::io::Error::new(io::ErrorKind::Other, "Could serialize data into JSON"));
                }
            }
        });
    }

    fn save_books(&self, book_db: &LinkedList<Book>) -> Result<(), std::io::Error> {
        use tokio::fs::File;
        return self.tokio_rt.block_on(async {
            let mut file = File::create(&self.filepath).await?;
            let num_books = format!("{}\n", book_db.len());
            file.write_all(num_books.as_bytes()).await?;

            for book in book_db {
                let _size_written = file
                    .write_all(
                        format!(
                            "{}|{}|{}|{}\n",
                            book.title(),
                            book.author(),
                            book.year_published(),
                            book.isbn()
                        )
                        .as_bytes(),
                    ).await?;
            }
            file.flush().await?;
            Ok(())
        });
    }

    fn load_books(&self, book_db: &mut LinkedList<Book>) -> Result<(), std::io::Error> {
        use tokio::fs::File;
        return self.tokio_rt.block_on(async {
            let mut file = File::open(&self.filepath).await?;
            let mut file_contents = String::default();
            file.read_to_string(&mut file_contents).await?;
            let mut line_itr = file_contents.lines();
            if let Some(first_line) = line_itr.next() {
                if let Ok(num_books) = first_line.parse::<usize>() {
                    for index in 0..num_books {
                        let book_line = line_itr.next().unwrap();
                        let parts = book_line.split('|').collect::<Vec<&str>>();
                        if parts.len() == 4 {
                            if let Ok(year_published) = parts[2].parse::<i16>()
                            {
                                book_db.push_back(Book::new(
                                    String::from(parts[0]),
                                    String::from(parts[1]),
                                    year_published,
                                    String::from(parts[3]),
                                ));
                            } else {
                                println!(
                                    "Line {} doesn't have a valid publish year as number",
                                    index + 1
                                );
                                continue;
                            }
                        } else {
                            println!(
                                "Line {} is malformed, skipping...",
                                index + 1
                            );
                            continue;
                        }
                    }
                    Ok(())
                } else {
                   return Err(std::io::Error::new(io::ErrorKind::InvalidData, "Could not parse number of book records present in books DB file"));
                }
            } else {
                return Err(std::io::Error::new(io::ErrorKind::NotFound, format!("Failed to read file {}", self.filepath)));
            }
        });
    }
}
