// Factorial digit sum
// Problem 20
// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!

pub struct Solution {}

impl Solution {
    pub fn factorial_digit_sum(n: u32) -> u32 {
        let mut result = String::from("1");
        for digit in 2..=n {
            result = Solution::mul_number_for_string(&result, digit);
        }
        result.chars()
            .map(|num| num.to_digit(10).unwrap())
            .sum::<u32>()
    }

    fn mul_number_for_string(strs: &String, digit: u32) -> String {
        let mut result = String::from(strs);
        for _ in 1..digit {
            result = Solution::add_strings(&result, strs);
        }
        result
    }

    fn add_strings(number1: &String, number2: &String) -> String {
        let len1 = number1.len();
        let chars1: Vec<char> = number1.chars().rev().collect();
        let len2 = number2.len();
        let mut chars2: Vec<char> = number2.chars().rev().collect();
        for _ in 0..len1 - len2 {
            chars2.push('0')
        }
        let mut result = String::new();
        let mut carry = 0;
        for i in 0..len1 {
            let num = chars1[i].to_digit(10).unwrap() + chars2[i].to_digit(10).unwrap();
            result.push_str(&((num + carry) % 10).to_string());
            carry = (num + carry) / 10;
        }
        if carry > 0 {
            result.push_str(&carry.to_string());
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_digit_sum_test() {
        assert_eq!(Solution::factorial_digit_sum(1), 1);
        assert_eq!(Solution::factorial_digit_sum(2), 2);
        assert_eq!(Solution::factorial_digit_sum(3), 6);
        assert_eq!(Solution::factorial_digit_sum(4), 6);
        assert_eq!(Solution::factorial_digit_sum(10), 27);
        assert_eq!(Solution::factorial_digit_sum(11), 36);
        assert_eq!(Solution::factorial_digit_sum(100), 648);
    }
}
