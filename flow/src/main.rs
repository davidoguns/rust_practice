fn main() {
    let _arr = [2, 4, 6, 8];
    println!("Array length? nope");
    let x: i64 = 28;
    println!("Value of x i64: {}", x);

    println!("What happens if I add strings? {}",
	     do_stuff("foolish"));

    println!("Loop range: {}", do_loop_range(1, 5));
}

//comment for doc popup?
fn do_stuff(input: &str) -> String {
    format!("{} world", input)
}

fn do_loop_range(start: i32, end: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in start..end {
        sum += i; 
    }
    sum
}
