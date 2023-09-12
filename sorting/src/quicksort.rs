pub fn quicksort(numbers: &mut Vec<i32>) {
    //unhandled: if len()-1 underflows usize; len = 0
   quicksort_internal(numbers, 0, numbers.len()-1); 
}

fn quicksort_internal(numbers: &mut Vec<i32>, start: usize, end: usize) {
    if  start >= end  {
        return
    } else if end - start == 1 {
        if numbers.get(start).unwrap() > numbers.get(end).unwrap() {
            numbers.swap(start, end);
        }
    } else {
        let pivot_index = end;
        let pivot_value = *numbers.get(pivot_index).unwrap();
        //these two counts also serve as letting us know how the partition
        //will be split when recursively calling quicksort
        let (mut lesser_count, mut greater_count) = (0, 0);
        for index in start..pivot_index {
            if *numbers.get(index).unwrap() < pivot_value {
                if greater_count != 0 { //if greater count == 0; then the number is fine in place
                    let insert_value = *numbers.get(index).unwrap();
                    let start_shift_up = start + lesser_count;
                    let shift_up_range = (start_shift_up..(start_shift_up + greater_count)).rev();
                    for shift_index in shift_up_range {
                        *numbers.get_mut(shift_index+1).unwrap() = *numbers.get_mut(shift_index).unwrap();
                    }
                    *numbers.get_mut(start + lesser_count).unwrap() = insert_value;
                }
                lesser_count += 1;
            } else {
                greater_count += 1;
            }
        }
        // let mid_index = if lesser_count > 0 { start + lesser_count } else { pivot_index };
        let mid_index = (start + lesser_count).clamp(1, end);
        //place pivot in the middle of array, all values above have been shifted over
        numbers.swap(mid_index, pivot_index);
        quicksort_internal(numbers, start, mid_index-1);
        quicksort_internal(numbers, mid_index+1, end);
    }
}
