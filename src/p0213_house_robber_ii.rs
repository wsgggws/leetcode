// 213. House Robber II
// Medium

// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

// Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.

// Example 1:

// Input: [2,3,2]
// Output: 3
// Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2),
//              because they are adjacent houses.
// Example 2:

// Input: [1,2,3,1]
// Output: 4
// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//              Total amount you can rob = 1 + 3 = 4.

pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        } else {
            return i32::max(
                Solution::rob_198(nums[0..n - 1].to_vec()),
                Solution::rob_198(nums[1..n].to_vec()),
            );
        }
    }
    fn rob_198(nums: Vec<i32>) -> i32 {
        let (mut pre2, mut pre1) = (0, 0);
        for i in 0..nums.len() {
            // dp[i] = max(dp[i-2]+nums[i], dp[i-1]);
            let cur = i32::max(pre2 + nums[i], pre1);
            pre2 = pre1;
            pre1 = cur;
        }
        pre1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rob_test() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![2, 3, 2, 3]), 6);
        assert_eq!(Solution::rob(vec![2, 3, 2, 3, 5]), 8);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
