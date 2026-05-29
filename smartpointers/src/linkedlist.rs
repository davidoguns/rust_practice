use std::fmt::Display ;

pub trait List<T> {
    fn print_list(&self);

    fn value(&self) -> &T;

    fn set_value(&mut self, value: &T) -> ();
}

/// LinkNode is a type that when handling the "list" should always be wrapped
/// in an @Option<T> 
pub struct LinkNode<T> {
    value: T,
    next: Option<Box<LinkNode<T>>>
}

impl<T: Copy + Display> LinkNode<T> {
    pub fn build(values: &[T]) -> Option<Box<LinkNode<T>>> {
        if values.is_empty() {
            None
        } else {
            Some(Box::new(LinkNode {
                value: *values.get(0).unwrap(),
                next: Self::build(&values[1..]),
            }))
        }
    }
}

impl<T: Copy + Display> List<T> for LinkNode<T> {
    fn print_list(&self) {
        print!("(value={})", self.value);
        match &self.next {
            Some(next_node) => {
                next_node.print_list();
            },
            None => {}
        }
    }

    fn value(&self) -> &T {       
        &self.value
    }

    fn set_value(&mut self, value: &T) -> () {
        self.value = *value;
    }
}

pub enum LinkList<T> {
    Node { value: T, next: Box<LinkList<T>> },
    None,
}

impl<T: Copy + Display> LinkList<T> {
    pub fn build(values: &[T]) -> Box<LinkList<T>> {
        if values.is_empty() {
            Box::new(Self::None)
        } else {
            Box::new(Self::Node {
                value: *values.get(0).unwrap(),
                next: Self::build(&values[1..]),
            })
        }
    }
}

impl<T: Copy + Display> List<T> for LinkList<T> {
    fn print_list(&self) {
        match self {
            Self::None => { }
            Self::Node { value, next } => {
                print!("(value={value})");
                next.print_list();
            }
        }
    }

    fn value(&self) -> &T {
        match self {
            Self::None => { panic!("Retrieving the value") }
            Self::Node { value, .. } => {
                &value
            }
        }
    }

    fn set_value(&mut self, value: &T) -> () {
        match self {
            Self::None => {}
            Self::Node { value, next } => {
                print!("(value={value})");
                next.print_list();
            }
        }
    }
}

//
//     pub fn build(values: &[T]) -> Rc<RefCell<Option<LinkNode<T>>>> {
// impl<T: Copy + Display> LinkNode<T> {
//         if values.is_empty() {
//             Rc::new(RefCell::new(None))
//         } else {
//             let head = Rc::new(RefCell::new(Some(Self::new(
//                 *values.get(0).unwrap(),
//                 Self::build(&values[1..]),
//             ))));
//             head
//         }
//     }
//
//     pub fn new(value: T, next: Rc<RefCell<Option<LinkNode<T>>>>) -> LinkNode<T> {
//         LinkNode { value, next }
//     }
//
//     pub fn print_list(&self) {
//         print!("{},", self.value);
//         match self.next {
//             Err(_) => { panic!("wut") }
//             Ok(node) => {
//                 let node = node.clone();
//                 if let Some(node) = node {
//                     node.print_list();
//                 } else {
//                     print!(";");
//                 }
//             }
//         }
//     }
// }
