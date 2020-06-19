// 704. Binary Search
// Given a sorted (in ascending order) integer array nums of n elements and a target value, write a function to search target in nums. If target exists, then return its index, otherwise return -1.

// Example 1:

// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4

// Example 2:

// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1

// Note:

// You may assume that all elements in nums are unique.
// n will be in the range [1, 10000].
// The value of each element in nums will be in the range [-9999, 9999].

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Some(x) = nums.iter().position(|&x| x == target) {
            x as i32
        } else {
            -1
        }
    }

    // 二分搜索法
    pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0i32, nums.len() as i32 -1);
        while start <= end {
            let mid = (start + end) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        assert_eq!(Solution::search(vec![1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![1, 0, 3, 5, 9, 12], 8), -1);
        assert_eq!(Solution::search(vec![1, 0, 3, 5, 9, 12], 12), 5);
        assert_eq!(Solution::search(vec![1, 0, 3, 5, 9, 12], 1), 0);
    }

    #[test]
    fn search_2_test() {
        assert_eq!(Solution::search_2(vec![1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search_2(vec![1, 0, 3, 5, 9, 12], 8), -1);
        assert_eq!(Solution::search_2(vec![1, 0, 3, 5, 9, 12], 12), 5);
        assert_eq!(Solution::search_2(vec![1, 0, 3, 5, 9, 12], 1), 0);
    }
}
