pub fn merge_sort(numbers: Vec<i32>) -> Vec<i32> {
    merge_sort_internal(numbers)
}

pub fn merge_sort_in_place(numbers: &mut Vec<i32>) {
    let end = numbers.len();
    merge_sort_in_place_internal(numbers, 0, end);
}

fn merge_sort_internal(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() == 1 || numbers.len() == 0 {
        numbers
    }
    else {
        let (left_slice, right_slice) = numbers.split_at(numbers.len() / 2);
        let (left, right) = (merge_sort_internal(Vec::from(left_slice)),
            merge_sort_internal(Vec::from(right_slice)));
        let mut merged = Vec::<i32>::with_capacity(numbers.len());
        let (mut left_itr, mut right_itr) = (left.iter().peekable(), right.iter().peekable());

        while left_itr.peek().is_some() || right_itr.peek().is_some() {
            let (lip, rip) = (left_itr.peek(), right_itr.peek());

            if lip.is_none() {
                merged.push(*right_itr.next().unwrap());
            }
            else if rip.is_none() {
                merged.push(*left_itr.next().unwrap());
            }
            else {
                if lip.unwrap() < rip.unwrap() {
                    merged.push(*left_itr.next().unwrap());
                }
                else {
                    merged.push(*right_itr.next().unwrap());
                }
            }
        }
        merged
    }
}

fn merge_sort_in_place_internal(numbers: &mut Vec<i32>, start: usize, end: usize) {
    let length: usize = end - start;
    if length == 1 || length == 0 {
        return
    }
    else if length == 2 {
        if numbers.get(start).unwrap() > numbers.get(end-1).unwrap() {
            numbers.swap(start, end-1);
        }
    }
    else {
        let mid_index: usize = (end + start) / 2;
        //sort left side
        merge_sort_in_place_internal(numbers, start, mid_index);
        //sort right side
        merge_sort_in_place_internal(numbers, mid_index, end);
        //now merge here
        let (mut insert_index, mut left_index, mut right_index) = (start, start, mid_index);

        //left_index < right_index means left side elements have all been inserted and the rest is
        //already right sorted
        while left_index < right_index && right_index < end {
            if numbers.get(left_index).unwrap() < numbers.get(right_index).unwrap() {
                //order is fine, leave it
                left_index += 1;
            }
            else {
                //grab the value to insert from right list
                let insert_value = *numbers.get(right_index).unwrap();
                //shift every value down until it hits the right list
                for shift_index in (left_index..right_index).rev() {
                    *numbers.get_mut(shift_index+1).unwrap() = *numbers.get(shift_index).unwrap();
                }
                *numbers.get_mut(insert_index).unwrap() = insert_value;
                right_index += 1;
                left_index += 1;
            }
            insert_index += 1;
        }
    }
}
