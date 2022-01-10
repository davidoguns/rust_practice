use std::io::Write;
use std::io;

struct Book {
    title: String,
    isbn: String,
    avg_review: f32,
    num_ratings: u32,
}

fn main() {
    println!("Hello, world!");

    let books = vec![
        Book { title: "Harry Potter".to_string(),
               isbn: "1234-56-7890".to_string(),
               avg_review: 9.2,
               num_ratings: 15687,
            },
        Book { title: "The Lord of the Rings".to_string(),
               isbn: "8234-31-4429".to_string(),
               avg_review: 8.2,
               num_ratings: 35413,
            }
    ];


    for book in books {
        println!("Book title: {}; Average Review: {:.2}",
                 book.title, book.avg_review);
        println!("Num Ratings: {}; ISBN: {}",
                 book.num_ratings, book.isbn);
    }

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Couldn't read guess from stdin");

        let x = guess.trim_end().parse::<i32>();
        match x {
            Ok(num) =>
                println!("Your guess is a valid number {}", num),
            Err(_) => {
                if guess.trim_end().to_lowercase() == "quit" ||
                    guess.trim_end().to_lowercase() == "q" {
                    break;
                } else {
                    println!("Your guess is not a valid number");
                }
            }
        }
    }
}
