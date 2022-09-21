// 342. Power of Four
// Easy

// Given an integer (signed 32 bits), write a function to check whether it is a power of 4.

// Example 1:

// Input: 16
// Output: true
// Example 2:

// Input: 5
// Output: false
// Follow up: Could you solve it without loops/recursion?

pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n > 0 {
            if n == 1 {
                return true;
            }
            if n % 4 != 0 {
                return false;
            }
            n /= 4;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_power_of_four_test() {
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(4), true);
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(25), false);
    }
}
