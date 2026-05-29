use std::io;
use std::io::Write;

trait Reviewed {
    fn show_review(&self) -> String;
}

struct Book {
    title: String,
    isbn: String,
    avg_review: f32,
    num_ratings: u32,
}

impl Reviewed for Book {
    fn show_review(&self) -> String {
        format!("Rating: {}; Count {}", self.avg_review, self.num_ratings)
    }
}

fn show_reviews<T: Reviewed>(items: &[T]) {
    for item in items {
        println!("{}", item.show_review())
    }
}

fn make_book(title: &str, isbn: &str, review: f32, ratings: u32) -> Book {
    Book {
        title: String::from(title),
        isbn: String::from(isbn),
        avg_review: review,
        num_ratings: ratings,
    }
}

fn get_string() -> &'static str {
    return "hello world";
}

fn main() {
    let mut bstr1 = Box::new(String::from("hello"));
    // since this is String::push_str(), the mutable borrow on self
    // maintains bstr1. The Box<String> was not borrowed on self so
    // it remains valid.
    bstr1.push_str(" world");
    // this looks like it should borrow out of Box string, but it's compiler
    // checked so the mutable borrow is allowed, while bstr1 is valid
    let unsure_str = bstr1.as_mut();
    // this line borrows out of unsure_str who's type is &mut String.
    // Because String::push_str() borrows mut from self, we cannot push_str
    // immediately after. The reference is immediately consumed
    unsure_str.push_str(" and again");
    unsure_str.push_str(" and again");
    // trouble resolving this line right here, why is 
    // bstr1 not borrowed out of here from the bstr1.as_mut()?
    // why do we still have a usable reference here?
    // Rust somehow knows and enforces that the bstr1.as_mut() borrow
    // must be completely finished, and then allows it to be used again
    // knowing the borrow is over? Thus reference is still valid?
    let unsure_strr2 = bstr1.as_mut();
    unsure_strr2.push_str(" what");
    bstr1.push_str(" forever");
    let bstr2 = bstr1;
    println!("Hello: {bstr2}");

    // use std::rc::Rc;
    // let rcstr1: Rc<String> = Rc::new(String::from("hello"));
    // rcstr1.push_str(" world");
    // let rcstr2 = &rcstr1;
    // println!("Hello: {rcstr2}");

    let mut list = vec![1 ,2, 3];
    println!("Before defining closure: {list:?}");
    std::thread::spawn(move || {
        println!("From thread: {list:?}");
        list.push(9);
        println!("From thread: {list:?}");
    }).join().unwrap();
    return;

    //named parameters are not a first-class feature of the language
    //this is possible because println!() is a macro
    println!("Named {x}, {x}, {z}, {y}", z = 3, y = 4, x = 5);

    println!(
        "Size of non-boxed option: {}",
        std::mem::size_of::<Option<i32>>()
    );
    println!(
        "Size of boxed option: {}",
        std::mem::size_of::<Option<Box<i32>>>()
    );
    let s = get_string();
    println!("String that was gotten: {}", s);
    let books = vec![
        Book {
            title: "Harry Potter".to_string(),
            isbn: "1234-56-7890".to_string(),
            avg_review: 9.2,
            num_ratings: 15687,
        },
        Book {
            title: "The Lord of the Rings".to_string(),
            isbn: "8234-31-4429".to_string(),
            avg_review: 8.2,
            num_ratings: 35413,
        },
        make_book("The Divide", "2345-23-6438", 8.6, 7562),
    ];

    show_reviews(&books);

    for book in books {
        println!(
            "Book title: {}; Average Review: {:.2}",
            book.title, book.avg_review
        );
        println!("Num Ratings: {}; ISBN: {}", book.num_ratings, book.isbn);
    }

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read guess from stdin");

        let x = guess.trim_end().parse::<i32>();
        match x {
            Ok(num) => println!("Your guess is a valid number {}", num),
            Err(_) => {
                if guess.trim_end().to_lowercase() == "quit"
                    || guess.trim_end().to_lowercase() == "q"
                {
                    break;
                } else {
                    println!("Your guess is not a valid number")
                }
            }
        }
    }
}
