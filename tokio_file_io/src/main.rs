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
