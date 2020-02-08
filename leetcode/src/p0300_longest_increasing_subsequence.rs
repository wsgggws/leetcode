// 300. Longest Increasing Subsequence
// Medium

// Given an unsorted array of integers, find the length of longest increasing subsequence.

// Example:

// Input: [10,9,2,5,3,7,101,18]
// Output: 4 
// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4. 
// Note:

// There may be more than one LIS combination, it is only necessary for you to return the length.
// Your algorithm should run in O(n2) complexity.
// Follow up: Could you improve it to O(n log n) time complexity?

pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![0; n];
        for i in 0..n {
            let mut maxs = 1;
            for j in 0..i {
                if nums[i] > nums[j] {
                    maxs = i32::max(maxs, dp[j] + 1);
                }
            }
            dp[i] = maxs
        }
        let mut result = 0;
        for i in 0..n {
            result = i32::max(result, dp[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_lis_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
