pub mod util;
pub mod mergesort;
pub mod quicksort;

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

#[cfg(test)]
mod test {
    use crate::fizz_buzz;

    #[test]
    pub fn test_fizz_buzz() {
        let v_answer = fizz_buzz(15, &vec![(3_u32, "fizz"), (4_u32, "bam"), (5_u32, "buzz")]);
        assert_eq!(v_answer, vec!["1", "2", "fizz", "bam", "buzz", "fizz", "7", "bam", "fizz", "buzz", "11", "fizzbam", "13", "14", "fizzbuzz"]);
        dbg!(v_answer);
    }
}
