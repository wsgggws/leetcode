// 326. Power of Three
// Easy

// Given an integer, write a function to determine if it is a power of three.

// Example 1:

// Input: 27
// Output: true
// Example 2:

// Input: 0
// Output: false
// Example 3:

// Input: 9
// Output: true
// Example 4:

// Input: 45
// Output: false
// Follow up:
// Could you do it without using any loop / recursion?

pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n > 0 {
            if n == 1 {
                return true;
            }
            if n % 3 != 0 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_power_of_three_test() {
        assert_eq!(Solution::is_power_of_three(1), true);
        assert_eq!(Solution::is_power_of_three(9), true);
        assert_eq!(Solution::is_power_of_three(10), false);
        assert_eq!(Solution::is_power_of_three(27), true);
    }
}
