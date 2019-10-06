// 67. Add Binary
// Easy

// Given two binary strings, return their sum (also a binary string).

// The input strings are both non-empty and contains only characters 1 or 0.

// Example 1:

// Input: a = "11", b = "1"
// Output: "100"
// Example 2:

// Input: a = "1010", b = "1011"
// Output: "10101"

pub struct Solution {}


use std::char::from_digit;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {

        let mut part_one: Vec<char> = a.chars().collect();
        let mut part_two: Vec<char> = b.chars().collect();

        let mut tag = 0;
        let mut result = String::new();

        while !(part_one.is_empty() && part_two.is_empty()) {
            let mut sum = part_one.pop().map_or(0, |ch| ch.to_digit(10).unwrap())
                + part_two.pop().map_or(0, |ch| ch.to_digit(10).unwrap()) + tag;
            if sum > 1 {
                sum -= 2;
                tag = 1;
            }else {
                tag = 0
            }
            result.insert(0, from_digit(sum, 10).unwrap());
        }
        if tag > 0 {
            result.insert(0, '1');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_binary_test() {
        assert_eq!(Solution::add_binary("11".to_string(), "1".to_string()), "100".to_string());
        assert_eq!(Solution::add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
    }
}
