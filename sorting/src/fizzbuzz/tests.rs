#![cfg(test)]
use crate::fizz_buzz;

#[test]
pub fn test_fizz_buzz() {
    let v_answer = fizz_buzz(15, &vec![(3_u32, "fizz"), (4_u32, "bam"), (5_u32, "buzz")]);
    assert_eq!(v_answer, vec!["1", "2", "fizz", "bam", "buzz", "fizz", "7", "bam", "fizz", "buzz", "11", "fizzbam", "13", "14", "fizzbuzz"]);
    dbg!(v_answer);
}

