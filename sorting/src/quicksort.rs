pub fn quicksort(numbers: &mut Vec<i32>) {
   quicksort_internal(numbers, 0, numbers.len()); 
}

fn quicksort_internal(numbers: &mut Vec<i32>, start: usize, end: usize) {
    let length: usize = end - start;
    if length == 1 || length == 0 {
        return
    } else if length == 2 {
        if numbers.get(start).unwrap() > numbers.get(end-1).unwrap() {
            numbers.swap(start, end-1);
        }
    } else {
        let pivot_index = end - 1;
        let pivot_value = *numbers.get(pivot_index).unwrap();
        //these two counts also serve as letting us know how the partition
        //will be split when recursively calling quicksort
        let (mut lesser_count, mut greater_count) = (0, 0);
        for index in start..pivot_index {
            if *numbers.get(index).unwrap() < pivot_value {
                if greater_count != 0 { //if greater count == 0; then the number is fine in place
                    let insert_value = *numbers.get(index).unwrap();
                    let shift_up_range = (start + lesser_count)..(start + lesser_count + greater_count);
                    for shift_index in shift_up_range.rev() {
                        *numbers.get_mut(shift_index+1).unwrap() = *numbers.get_mut(shift_index).unwrap();
                    }
                    *numbers.get_mut(lesser_count).unwrap() = insert_value;
                }
                lesser_count += 1;
            } else {
                greater_count += 1;
            }
        }
        let mid_index = start + lesser_count;
        quicksort_internal(numbers, start, mid_index);
        quicksort_internal(numbers, mid_index, end);
    }
}
