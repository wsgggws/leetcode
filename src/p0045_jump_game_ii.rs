// 45. Jump Game II
// Hard

// 2419

// 127

// Add to List

// Share
// Given an array of non-negative integers, you are initially positioned at the first index of the array.

// Each element in the array represents your maximum jump length at that position.

// Your goal is to reach the last index in the minimum number of jumps.

// Example:

// Input: [2,3,1,1,4]
// Output: 2
// Explanation: The minimum number of jumps to reach the last index is 2.
//     Jump 1 step from index 0 to 1, then 3 steps to the last index.
// Note:

// You can assume that you can always reach the last index.

pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // 题目确保总是可达, 初始化到达index需要index步
        let mut dp: Vec<i32> = (0..nums.len() as i32).collect();
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] + j as i32 >= i as i32 {
                    dp[i] = i32::min(dp[i],dp[j] + 1);
                }
            }
        }
        dp[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_test() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
