// 415. Add Strings
// Easy

// Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.

// Note:

// The length of both num1 and num2 is < 5100.
// Both num1 and num2 contains only digits 0-9.
// Both num1 and num2 does not contain any leading zero.
// You must not use any built-in BigInteger library or convert the inputs to integer directly.

pub struct Solution {}


use std::char::from_digit;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut part_one: Vec<char> = num1.chars().collect();
        let mut part_two: Vec<char> = num2.chars().collect();

        let mut tag = 0;
        let mut result = String::new();

        while !(part_one.is_empty() && part_two.is_empty()) {
            let mut sum = part_one.pop().map_or(0, |ch| ch.to_digit(10).unwrap())
                + part_two.pop().map_or(0, |ch| ch.to_digit(10).unwrap()) + tag;
            if sum > 9 {
                sum -= 10;
                tag = 1;
            }else {
                tag = 0;
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
    fn add_strings_test() {
        assert_eq!(Solution::add_strings("0".to_owned(), "0".to_owned()), "0".to_owned());
        assert_eq!(Solution::add_strings("123".to_owned(), "123".to_owned()), "246".to_owned());
        assert_eq!(Solution::add_strings("123".to_owned(), "999".to_owned()), "1122".to_owned());
    }
}
