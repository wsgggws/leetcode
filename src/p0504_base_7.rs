// 504. Base 7
// Easy

// Given an integer, return its base 7 string representation.

// Example 1:
// Input: 100
// Output: "202"
// Example 2:
// Input: -7
// Output: "-10"
// Note: The input will be in range of [-1e7, 1e7].

pub struct Solution {}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_owned();
        }
        let is_negative = if num < 0 { true } else { false };
        let mut num = if is_negative { -num } else { num };
        let mut result = String::new();
        while num > 0 {
            result.push_str(&(num % 7).to_string());
            num /= 7;
        }
        if is_negative {
            result.push('-');
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_base7_test() {
        assert_eq!(Solution::convert_to_base7(100), "202".to_owned());
        assert_eq!(Solution::convert_to_base7(-7), "-10".to_owned());
    }
}
