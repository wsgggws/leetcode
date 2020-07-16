// 674. Longest Continuous Increasing Subsequence
// Easy

// Given an unsorted array of integers, find the length of longest continuous increasing subsequence (subarray).

// Example 1:
// Input: [1,3,5,4,7]
// Output: 3
// Explanation: The longest continuous increasing subsequence is [1,3,5], its length is 3. 
// Even though [1,3,5,7] is also an increasing subsequence, it's not a continuous one where 5 and 7 are separated by 4. 
// Example 2:
// Input: [2,2,2,2,2]
// Output: 1
// Explanation: The longest continuous increasing subsequence is [2], its length is 1. 
// Note: Length of the array will not exceed 10,000.

pub struct Solution {}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut result = 1;
        let mut cur_count = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                cur_count += 1;
                result = i32::max(result, cur_count);
            } else {
                cur_count = 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_length_of_lcis_test() {
        assert_eq!(Solution::find_length_of_lcis(vec![]), 0);
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
        assert_eq!(Solution::find_length_of_lcis(vec![2, 3, 4, 5, 6]), 5);
    }
}
