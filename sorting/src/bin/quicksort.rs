use std::env;
use std::io::stdin;

use sorting::util::{load_nums, print_nums};
use sorting::quicksort::quicksort;

fn main() {

    let input_size = env::args().skip(1).next().unwrap().parse::<usize>().unwrap();

    match load_nums(input_size, &stdin()) {
        Ok(mut numbers) => {
            quicksort(&mut numbers);
            eprintln!("Postsort...");
            print_nums(&numbers);
        }
        Err(e) => {
            eprintln!("Error loading numbers: {e}");
        }
    }
}
