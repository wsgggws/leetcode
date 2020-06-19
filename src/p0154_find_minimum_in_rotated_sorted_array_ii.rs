// 154. Find Minimum in Rotated Sorted Array II
// Hard

// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.

// (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).

// Find the minimum element.

// The array may contain duplicates.

// Example 1:

// Input: [1,3,5]
// Output: 1
// Example 2:

// Input: [2,2,2,0,1]
// Output: 0
// Note:

// This is a follow up problem to Find Minimum in Rotated Sorted Array.
// Would allow duplicates affect the run-time complexity? How and why?

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            // 去除两端重复的，就跟153题是一致的
            while left < right && nums[right] == nums[right - 1] {
                right -= 1;
            }
            while left < right && nums[left] == nums[left + 1] {
                left += 1;
            }
            let mid = left + (right - left) / 2;
            if nums[mid] <= nums[right] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_min_test() {
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    }
}
