use std::fmt::Display;

use smartpointers::linkedlist;
use smartpointers::linkedlist::List;

fn print_lists<T: Copy + Display>(
    list1: &linkedlist::LinkList<T>,
    list2: &Option<Box<linkedlist::LinkNode<T>>>,
) {
    println!("\n==================List 1==========================");
    list1.print_list();
    println!("\n==================List 2==========================");
    if let Some(list2) = list2 {
        list2.print_list();
    }
}

fn print_list<T>(list: &impl List<T>) {
    println!("\n====================List==========================");
    list.print_list();
}

fn main() {
    //Note: having a type based off of an Enum with it's own "None" value is
    //way more ergonomic than a struct that is constantly wrapped in Option
    let mut list1 = linkedlist::LinkList::build(&[5, 4, 2, 1]);
    let mut list2 = linkedlist::LinkNode::build(&[5, 4, 2, 1]);
    print_lists(&list1, &list2);
    // change list1's first element
    if let linkedlist::LinkList::Node{ref mut value, ..} = *list1 {
        println!("\nAdjusting the value of the first element of list 1");
        *value = 6;
    }
    // change list2's first element
    if let Some(ref mut list2_head) = list2 {
        list2_head.set_value(&18);
    }

    print_list(&*list1);
    if let Some(ref list2_head) = list2 {
        print_list(&**list2_head);
    }

    list1.push_value(&42);
    if let Some(ref mut list2_head) = list2 {
        list2_head.push_value(&22);
    }

    print_list(&*list1);
    if let Some(ref list2_head) = list2 {
        print_list(&**list2_head);
    }
}
