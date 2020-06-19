// 201. Bitwise AND of Numbers Range
// Medium

// Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.

// Example 1:

// Input: [5,7]
// Output: 4
// Example 2:

// Input: [0,1]
// Output: 0

pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        (m..n).fold(n, |result, x| result & x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_bitwise_and_test() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
}
