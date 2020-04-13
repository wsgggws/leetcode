// 494. Target Sum
// Medium

// You are given a list of non-negative integers, a1, a2, ..., an, and a target, S. Now you have 2 symbols + and -. For each integer, you should choose one from + and - as its new symbol.

// Find out how many ways to assign symbols to make sum of integers equal to target S.

// Example 1:
// Input: nums is [1, 1, 1, 1, 1], S is 3.
// Output: 5
// Explanation:

// -1+1+1+1+1 = 3
// +1-1+1+1+1 = 3
// +1+1-1+1+1 = 3
// +1+1+1-1+1 = 3
// +1+1+1+1-1 = 3

// There are 5 ways to assign symbols to make the sum of nums be target 3.
// Note:
// The length of the given array is positive and will not exceed 20.
// The sum of elements in the given array will not exceed 1000.
// Your output answer is guaranteed to be fitted in a 32-bit integer.

pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if sum < s || (sum + s) % 2 != 0 {
            return 0;
        }
        let w = (sum + s) / 2;
        let mut dp: Vec<i32> = vec![0; (w + 1) as usize];
        dp[0] = 1;
        for num in nums {
            for i in (num..=w).rev() {
                dp[i as usize] = dp[i as usize] + dp[(i - num) as usize];
            }
        }
        dp[w as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_target_sum_ways_test() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}
