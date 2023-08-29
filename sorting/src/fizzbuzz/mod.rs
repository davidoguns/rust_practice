mod tests;

pub fn fizz_buzz(to: u32, replacements: &Vec<(u32, &str)>) -> Vec<String> {
    let mut output = Vec::new();
    for n in 1..=to {
        match get_replacement(n, &replacements) {
            Some(x) => output.push(x),
            None => output.push(n.to_string())
        }
    }
    output
}

fn get_replacement(number: u32, replacements: &Vec<(u32, &str)>) -> Option<String> {
    let mut s = String::new();
    let mut count = 0;
    for replacement in replacements {
        if number % replacement.0 == 0 {
            s.push_str(replacement.1);
            count += 1;
        }
    }
    if count == 0 {
        return None
    }
    Some(s)
}

