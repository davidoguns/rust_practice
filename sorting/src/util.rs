use std::num::ParseIntError;
use std::io::Stdin;

pub fn load_nums(number_of_numbers: usize, stdin: &Stdin) -> Result<Vec<i32>, ParseIntError> {
    let mut vec = Vec::<i32>::with_capacity(number_of_numbers);

    for _ in 0..number_of_numbers {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);
        let trimmed = buffer.trim_end();
        match trimmed.parse::<i32>() {
            Ok(num) => vec.push(num),
            Err(e) => return Err(e)
        }
    }
    Ok(vec)
}

pub fn print_nums(numbers: &Vec<i32>) {
    for n in numbers {
        println!("{n}");
    }
}
