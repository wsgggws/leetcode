// 238. Product of Array Except Self
// Medium

// Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].

// Example:

// Input:  [1,2,3,4]
// Output: [24,12,8,6]
// Note: Please solve it without division and in O(n).

// Follow up:
// Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)

pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result: Vec<i32> = vec![1; n];
        let mut left = 1;
        for index in 1..n {
            left *= nums[index - 1];
            result[index] *= left;
        }
        let mut right = 1;
        for index in (0..=n-2).rev() {
            right *= nums[index + 1];
            result[index] *= right;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_except_self_test() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4, 5]), vec![120, 60, 40, 30, 24]);
    }
}
