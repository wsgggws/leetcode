// 258. Add Digits
// Easy

// Given a non-negative integer num, repeatedly add all its digits until the result has only one digit.

// Example:

// Input: 38
// Output: 2
// Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2.
//              Since 2 has only one digit, return it.
// Follow up:
// Could you do it without any loop/recursion in O(1) runtime?

pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut result = num;
        while result > 9 {
            result = Solution::calc(result);
        }
        result
    }
    fn calc(num: i32) -> i32 {
        let mut result = 0;
        let mut num = num;
        while num > 0 {
            result += num % 10;
            num /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_digits_test() {
        assert_eq!(Solution::add_digits(0), 0);
        assert_eq!(Solution::add_digits(9), 9);
        assert_eq!(Solution::add_digits(10), 1);
        assert_eq!(Solution::add_digits(38), 2);
    }
}
