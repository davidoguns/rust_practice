use std::io::Write;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let num = rand::thread_rng().gen_range(1, 1000);

    println!("Number is: {}", num);

    let mut guess =  String::new();

    print!("Please enter your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
	.expect("Couldn't read guess from stdin");

    println!("Your guess is: {}", guess);
}
