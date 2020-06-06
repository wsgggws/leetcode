// 540. Single Element in a Sorted Array
// Medium

// You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once. Find this single element that appears only once.

// Example 1:

// Input: [1,1,2,3,3,4,4,8,8]
// Output: 2
// Example 2:

// Input: [3,3,7,7,10,11,11]
// Output: 10

// Note: Your solution should run in O(log n) time and O(1) space.

pub struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        while left < right {
            let mut mid = left + (right - left) / 2;
            if mid % 2 == 1 {
                mid -= 1;
            }
            if nums[mid as usize] == nums[mid as usize + 1_usize] {
                left += 2;
            } else {
                right = mid;
            }
        }
        nums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn single_non_duplicate_test() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
