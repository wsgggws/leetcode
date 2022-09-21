// 41. First Missing Positive
// Hard

// Given an unsorted integer array, find the smallest missing positive integer.

// Example 1:

// Input: [1,2,0]
// Output: 3
// Example 2:

// Input: [3,4,-1,1]
// Output: 2
// Example 3:

// Input: [7,8,9,11,12]
// Output: 1
// Note:

// Your algorithm should run in O(n) time and uses constant extra space.

pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        for i in 0..nums.len() {
            while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[i] != nums[nums[i] as usize - 1] {
                let j = nums[i] as usize - 1;
                nums.swap(i, j);
            }
        }
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_missing_positive_test() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![1, 2]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
