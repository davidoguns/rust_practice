pub struct ListNode<T> {
    value: T,
    next: Box<ListNode<T>>
}

impl<T> ListNode<T> {
    pub fn new(value: T, next: Box<ListNode<T>>) -> Self {
        Self { value, next }
    }

    pub fn next(&self) -> &Self {
        &self.next
    }
    
    pub fn next_mut(&mut self) -> &mut Self {
        &mut self.next
    }
}

#[cfg(test)]
mod test {
    
    #[test]
    pub fn test_list_basics() {
    }
}
