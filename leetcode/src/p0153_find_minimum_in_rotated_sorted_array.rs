// 153. Find Minimum in Rotated Sorted Array
// Medium

// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.

// (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).

// Find the minimum element.

// You may assume no duplicate exists in the array.

// Example 1:

// Input: [3,4,5,1,2]
// Output: 1
// Example 2:

// Input: [4,5,6,7,0,1,2]
// Output: 0

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] <= nums[right as usize] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_min_test() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
}
