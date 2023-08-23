use std::env;

fn parse_args() -> (i32, i32, Option<u32>) {
    let mut args = env::args().skip(1);
    let lower_bound = args.next().unwrap().parse::<i32>().unwrap();
    let upper_bound = args.next().unwrap().parse::<i32>().unwrap();
    let amount = match args.next() {
        Some(amount_arg) => 
            match amount_arg.parse::<u32>() {
                Ok(x) => Some(x),
                Err(_) => {
                    None
                }
            },
        None => None
    };
    (lower_bound, upper_bound, amount)
}

use std::io::stdin;
use rand::prelude::{Rng, thread_rng};
fn main() {
    let bounds = parse_args();
    let mut rng = thread_rng();
    if let Some(amount) = bounds.2 {
        for _ in 0..amount {
            println!("{}", rng.gen_range(bounds.0..=bounds.1));
        }
    } else {
        loop {
            let mut buffer = String::new();
            let _ = stdin().read_line(&mut buffer);
            let trimmed_buffer = buffer.trim_end();
            match trimmed_buffer.parse::<u32>() {
                Ok(amount) => {
                    for _ in 0..amount {
                        println!("{}", rng.gen_range(bounds.0..=bounds.1));
                    }
                }
                Err(_) => break
            }
        }
    }
}
