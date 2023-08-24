use std::env;
use std::io::stdin;

use sorting::util::{load_nums, print_nums};
use sorting::mergesort::{merge_sort, merge_sort_in_place};

fn main() {
    let input_size = env::args().skip(1).next().unwrap().parse::<usize>().unwrap();

    match load_nums(input_size, &stdin()) {
        Ok(unsorted) => {
            let mut sorted_in_place = unsorted.clone();
            let sorted = merge_sort(unsorted);
            print_nums(&sorted);
            eprintln!("In place results...");
            merge_sort_in_place(&mut sorted_in_place);
            print_nums(&sorted_in_place);
        }
        Err(e) => {
            eprintln!("Error loading numbers: {e}");
        }
    }
}
