// 202. Happy Number
// Easy

// Write an algorithm to determine if a number is "happy".

// A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.

// Example: 

// Input: 19
// Output: true
// Explanation: 
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1

pub struct Solution {}


use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut nums: HashSet<i32> = HashSet::new();
        let mut temp = n;
        nums.insert(n);
        loop {
            let result = Solution::calc(temp);
            if result == 1 { return true; }
            if !nums.insert(result) { return false; }
            temp = result;
        }
    }
    fn calc(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;
        while n > 0 {
            let current = n % 10;
            result += current * current;
            n /= 10;
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_happy_test() {
        assert_eq!(Solution::is_happy(1), true);
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(18), false);
    }
}
