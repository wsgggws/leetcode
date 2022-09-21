// 377. Combination Sum IV
// Medium

// Given an integer array with all positive numbers and no duplicates, find the number of possible combinations that add up to a positive integer target.

// Example:

// nums = [1, 2, 3]
// target = 4

// The possible combination ways are:
// (1, 1, 1, 1)
// (1, 1, 2)
// (1, 2, 1)
// (1, 3)
// (2, 1, 1)
// (2, 2)
// (3, 1)

// Note that different sequences are counted as different combinations.

// Therefore the output is 7.
 

// Follow up:
// What if negative numbers are allowed in the given array?
// How does it change the problem?
// What limitation we need to add to the question to allow negative numbers?

// Credits:
// Special thanks to @pbrother for adding this problem and creating all test cases.

pub struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; target as usize + 1];
        dp[0] = 1;
        for k in 1..=target {
            for &num in nums.iter() {
                if k >= num {
                    dp[k as usize] += dp[(k - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination_sum4_test() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }
}
