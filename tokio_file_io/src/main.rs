mod book;

use std::collections::linked_list::LinkedList;
use book::Book;
use std::io::Write;

fn read_stdin_option() -> Option<i16> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<i16>() {
                Ok(number) => {
                    Some(number)
                },
                Err(_e) => {
                    eprintln!("Could not parse number from stdin");
                    None
                }
            }
        },
        Err(_e) => {
            eprintln!("Error attempting to read from stdin");
            None
        }
    }
}

fn add_book(book_db: &mut LinkedList<Book>) {
    let title;
    let author;
    let year_published;
    let isbn;
    let mut input = String::new();

    print!("Title: ");
    std::io::stdout().flush().unwrap(); // requires import of std::io::Write
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            title = String::from(input.trim());
        },
        Err(_e) => {
            println!("Error reading title from stdin.");
            return
        }
    }
    
    print!("Author: ");
    std::io::stdout().flush().unwrap(); // requires import of std::io::Write
    input.clear();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            author = String::from(input.trim());
        },
        Err(_e) => {
            println!("Error reading author from stdin");
            return
        }
    }

    print!("Publish year: ");
    std::io::stdout().flush().unwrap(); // requires import of std::io::Write
    input.clear();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            if let Ok(number) = String::from(input.trim()).parse::<i16>() {
                year_published = number;
            } else {
                println!("Failed to parse number from input");
                return
            }
        },
        Err(_e) => {
            return
        }
    }

    print!("ISBN: ");
    std::io::stdout().flush().unwrap(); // requires import of std::io::Write
    input.clear();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            let unvalidated = String::from(input.trim());
            let pattern = regex::Regex::new(r"\d{10}|\d{10}").unwrap();
            if pattern.is_match(unvalidated.as_str()) {
                isbn = unvalidated;
            } else {
                println!("Invalid ISBN number entered");
                return
            }
        },
        Err(_e) => {
            println!("Error reading ISBN from stdin");
            return
        }
    }
    
    book_db.push_back(Book::new(title, author, year_published, isbn));
}

fn delete_book(book_db: &mut LinkedList<Book>) {
    let mut input = String::new();
    print!("Enter index of the book to delete: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => { 
            if let Ok(delete_index) = String::from(input.trim()).parse::<usize>() {
                if (0..book_db.len()).contains(&delete_index) {
                    // seems rust cannot remove an item from a list through and iterator, index, or
                    // any other stable function at this time
                    let mut split_off = book_db.split_off(delete_index);
                    split_off.pop_front();
                    book_db.append(&mut split_off);
                } else {
                    println!("Book index to delete is out of range! Pick between 0 and {}", book_db.len()-1);
                }
            } else {
                println!("Could not parse index(usize) from input");
                return
            }
        },
        Err(_e) => {
            println!("Error reading index from stdin");
            return
        }
    }
}

fn view_all_books(book_db: &LinkedList<Book>) {
    println!("==================================================================");
    for (book_index, book) in book_db.iter().enumerate() {
        println!("Book {}: {}", book_index, book);
    }
    println!("==================================================================");
}

fn view_book(book_db: &LinkedList<Book>) {
    let mut input = String::new();
    print!("Enter index of the book to view: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => { 
            if let Ok(view_index) = String::from(input.trim()).parse::<usize>() {
                if (0..book_db.len()).contains(&view_index) {
                    if let Some(book) = book_db.iter().skip(view_index).next() {
                        println!("Book {}: {}", view_index, book);
                    }
                } else {
                    println!("Book index to view is out of range! Pick between 0 and {}", book_db.len()-1);
                }
            } else {
                println!("Could not parse index(usize) from input");
                return
            }
        },
        Err(_e) => {
            println!("Error reading index from stdin");
            return
        }
    }
}

fn save_books(book_db: &LinkedList<Book>) {
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;

    let mut input = String::new();

    print!("Enter filename to save to: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            let path = Path::new(input.trim());
            match File::create(&path) {
                Err(_e) => {
                    println!("Error creating file [{}] for writing", input.trim());
                    return
                },
                Ok(mut file) => {
                    let num_books = format!("{}\n", book_db.len());
                    //write number of books first
                    let _size_written = file.write_all(num_books.as_bytes());

                    for book in book_db {
                        let _size_written = file.write_all(format!("{}|{}|{}|{}\n",
                                book.title(), book.author(), book.year_published(), book.isbn()).as_bytes());
                    }
                }
            };
        },
        Err(_e) => {
            println!("Error reading filename from stdin");
            return
        }
    }
}

fn load_books(book_db: &mut LinkedList<Book>) {
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;

    let mut input = String::new();
    let mut overwrite = true;
    
    if !book_db.is_empty() {
        print!("Books exist in DB, [o]verwrite DB, [a]ppend DB, or [c]ancel? [c]: ");
        std::io::stdout().flush().unwrap();
        input.clear();
        overwrite = match std::io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                match input.trim().to_lowercase().as_str() {
                    "a" => { false },
                    "o" => { true },
                    _ => { return }
                }
            },
            Err(_e) => {
                println!("Error reading write option choice from stdin");
                return
            }
        }
    }

    if overwrite {
        book_db.clear();
    }

    print!("Enter filename to load from: ");
    std::io::stdout().flush().unwrap();
    input.clear();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            match File::open(&Path::new(input.trim())) {
                Ok(mut file) => {
                    input.clear();
                    match file.read_to_string(&mut input) {
                        Ok(_bytes_read) => {
                            //first line should be the proper count of books in the database
                            let mut line_itr = input.lines();
                            if let Some(num_books_line) = line_itr.next() {
                                if let Ok(num_books) = num_books_line.parse::<usize>() {
                                    for index in 0..num_books {
                                        if let Some(book_line) = line_itr.next() {
                                            let parts = book_line.split('|').collect::<Vec<&str>>();
                                            if parts.len() == 4 {
                                                if let Ok(year_published) = parts[2].parse::<i16>() {
                                                    book_db.push_back(Book::new(
                                                        String::from(parts[0]),
                                                        String::from(parts[1]),
                                                        year_published,
                                                        String::from(parts[3])));
                                                } else {
                                                    println!("Line {} doesn't have a valid publish year as number", index+1);
                                                    continue;
                                                }
                                            } else {
                                                println!("Line {} is malformed, skipping...", index+1);
                                                continue;
                                            }
                                        } else {
                                            println!("Unexpected end of books db at line {}", index+1);
                                            return
                                        }
                                    }
                                } else {
                                    println!("Couldn't parse number of books from the first line [{}]", num_books_line);
                                    return
                                }
                            } else {
                                println!("Couldn't get line to read number of books");
                                return
                            }
                        },
                        Err(_e) => {
                            println!("Error reading contents of file {} into string", input.trim());
                        }
                    }
                },
                Err(_e) => {
                    println!("Error opening file {} for loading", input.trim());
                    return
                }
            }
        },
        Err(_e) => {
            println!("Error reading filename to load from stdin");
            return
        }
    }
}

fn main() {
    let mut book_db = LinkedList::<Book>::new();

    loop {
        println!(""); // just want a newline
        println!("=========================================================================");
        println!("Welcome to the books database. There are {} books in the database.", book_db.len());
        println!("1. Add book");
        println!("2. Delete book");
        println!("3. View book");
        println!("4. View full DB");
        println!("5. Load books from DB");
        println!("6. Save DB");
        println!("7. Exit");
        print!("Please select an option: ");
        std::io::stdout().flush().unwrap();

        match read_stdin_option() {
            Some(1) => {
                add_book(&mut book_db);
            },
            Some(2) => {
                delete_book(&mut book_db);
            },
            Some(3) => {
                view_book(&book_db);
            },
            Some(4) => {
                view_all_books(&book_db);
            },
            Some(5) => {
                load_books(&mut book_db);
            },
            Some(6) => {
                save_books(&book_db);
            },
            Some(7) => {
                println!("Thank you for using the book library.");
                break;
            },
            _ => {
                println!("Invalid option chosen. Please select from the menu options presented.");
            }
        }
    }
}
