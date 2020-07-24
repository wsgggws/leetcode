// 53. Maximum Subarray
// Easy

// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

// Example:

// Input: [-2,1,-3,4,-1,2,1,-5,4],
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
// Follow up:

// If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

pub struct Solution {}

impl Solution {
    pub fn max_sub_array_dp(nums: Vec<i32>) -> i32 {
        // dp[i] 表示 0..=i 的最大值，
        // dp[i] = max(dp[i-1]+nums[i], nums[i]);
        let mut dp: Vec<i32> = vec![0; nums.len()];
        dp[0] = nums[0];
        for i in 1..nums.len() {
            dp[i] = i32::max(nums[i]+dp[i-1], nums[i]);
        }
        *dp.iter().max().unwrap()
    }

    // 空间优化
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_value = i32::min_value();
        let mut current: i32 = 0;
        for i in 0..nums.len() {
            current += nums[i];
            max_value = i32::max(max_value, current);
            if current <= 0 {
                current = 0;
            }
        }
        max_value
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sub_array_test() {
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-1, 0]), 0);
        assert_eq!(Solution::max_sub_array(vec![-8, -2]), -2);
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn max_sub_array_dp_test() {
        assert_eq!(Solution::max_sub_array_dp(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array_dp(vec![-1, 0]), 0);
        assert_eq!(Solution::max_sub_array_dp(vec![-8, -2]), -2);
        assert_eq!(
            Solution::max_sub_array_dp(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array_dp(vec![1, 2, 3, 4]), 10);
    }
}
