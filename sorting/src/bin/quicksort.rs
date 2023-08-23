
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut vitr = v.iter();
    print_iter(&mut vitr);
}

fn print_iter(iter: &mut dyn Iterator<Item = &i32>) {
    while let Some(item) = iter.next() {
        println!("Number:  {}", item);
    }
}
