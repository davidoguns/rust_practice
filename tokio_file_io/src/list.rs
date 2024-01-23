pub enum List<T> {
    Empty,
    NonEmpty(Box<ListNode<T>>),
}

struct ListNode<T> {
    value: T,
    next: List<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn push_back(&mut self, value: T) {
        match *self {
            List::Empty => {
                *self = List::NonEmpty(Box::new(ListNode::<T> { value, next: List::Empty }))
            },
            List::NonEmpty(ref mut node) => {
                node.next.push_back(value)
            }
        }
    }

    pub fn length(&self) -> usize {
        match &self {
            List::Empty => { 0 },
            List::NonEmpty(ref node) => {
                1usize + node.next.length()
            }
        }
    }
    
    pub fn length_itr(&self) -> usize {
        let mut count = 0usize;
        let mut current = self;
        while let List::NonEmpty(current_node) = current {
            count += 1;
            current = &current_node.next;
        }
        count
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut curr_index = 0usize;
        let mut current = self;
        while let List::NonEmpty(current_node) = current {
            if index == curr_index {
                return Some(&current_node.value);
            }
            current = &current_node.next;
            curr_index += 1;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    pub fn test_list_basics() {
        let mut list1 = List::<i32>::new(); 
        list1.push_back(1);
        list1.push_back(2);
        list1.push_back(3);
        assert_eq!(3, list1.length());
        assert_eq!(3, list1.length_itr());
        assert_eq!(Some(&3), list1.get(2));
        assert_eq!(None, list1.get(3));
    }
}
