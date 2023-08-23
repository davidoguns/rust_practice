use std::env;
use std::io::stdin;

use sorting::util::{load_nums, print_nums};
use sorting::mergesort::merge_sort;

fn main() {
    let input_size = env::args().skip(1).next().unwrap().parse::<usize>().unwrap();

    match load_nums(input_size, &stdin()) {
        Ok(unsorted) => {
            let sorted = merge_sort(unsorted);
            print_nums(&sorted);
        }
        Err(e) => {
            eprintln!("Error loading numbers: {e}");
        }
    }
}
