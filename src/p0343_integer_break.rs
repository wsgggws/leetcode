// 343. Integer Break
// Medium

// Given a positive integer n, break it into the sum of at least two positive integers and maximize the product of those integers. Return the maximum product you can get.

// Example 1:

// Input: 2
// Output: 1
// Explanation: 2 = 1 + 1, 1 × 1 = 1.
// Example 2:

// Input: 10
// Output: 36
// Explanation: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.
// Note: You may assume that n is not less than 2 and not larger than 58.

pub struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![1; (n + 1) as usize];
        for i in 2..=n {
            for j in 1..i {
                dp[i as usize] = i32::max(
                    dp[i as usize],
                    i32::max(j * dp[(i - j) as usize], j * (i - j)),
                )
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_break_test() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
