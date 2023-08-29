
pub fn quicksort(numbers: &mut Vec<i32>) {
   quicksort_internal(numbers, 0, numbers.len()); 
}

fn quicksort_internal(numbers: &mut Vec<i32>, start: usize, end: usize) {
    let pivot_index = end - 1;
    let pivot_value = numbers.get(pivot_index).unwrap();

    //these two counts also serve as letting us know how the partition
    //will be split when recursively calling quicksort
    let (mut lesser_count, mut greater_count) = (0, 0);

    for index in start..pivot_index {
        if numbers.get(index).unwrap() < pivot_value {
            lesser_count += 1;
        } else {
            greater_count += 1;
        }
    }
}
