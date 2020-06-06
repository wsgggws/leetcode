// 15. 3Sum
// Medium

// Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.

// Note:

// The solution set must not contain duplicate triplets.

// Example:

// Given array nums = [-1, 0, 1, 2, -1, -4],

// A solution set is:
// [
//   [-1, 0, 1],
//   [-1, -1, 2]
// ]

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums: Vec<i32> = nums;
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        for index in 0..nums.len() {
            if index > 0 && nums[index] == nums[index - 1] {
                continue;
            }
            let mut left = index;
            let mut right = nums.len() - 1;
            while left < right {
                if left == index {
                    left += 1;
                } else if right == index {
                    right -= 1;
                } else if nums[left] + nums[right] + nums[index] == 0 {
                    result.push(vec![nums[index], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    left += 1;
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    right -= 1;
                    continue;
                } else if nums[left] + nums[right] + nums[index] >= 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_test() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
