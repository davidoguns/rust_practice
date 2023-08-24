
pub fn quicksort(numbers: &mut Vec<i32>) {
   quicksort_internal(numbers, 0, numbers.len()); 
}

fn get_pivot_index(_numbers: &mut Vec<i32>, start: usize, end: usize) -> usize {
    (start + end) / 2
}

fn quicksort_internal(numbers: &mut Vec<i32>, start: usize, end: usize) {
    let _pivot_index = get_pivot_index(numbers, start, end);
}
