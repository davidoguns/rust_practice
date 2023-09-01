use std::env;
use std::io::stdin;

use sorting::util::{load_nums, print_nums};
use sorting::quicksort::quicksort;

fn main() {
    for i in 1..1 {
        println!("This is i: {i}");
    }
    println!("End of i...");

    let input_size = env::args().skip(1).next().unwrap().parse::<usize>().unwrap();

    match load_nums(input_size, &stdin()) {
        Ok(mut numbers) => {
            println!("Presort...");
            print_nums(&numbers);
            quicksort(&mut numbers);
            println!("Postsort...");
            print_nums(&numbers);
        }
        Err(e) => {
            eprintln!("Error loading numbers: {e}");
        }
    }
}
