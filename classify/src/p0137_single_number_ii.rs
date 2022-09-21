// 137. Single Number II
// Medium

// Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.

// Note:

// Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

// Example 1:

// Input: [2,2,3,2]
// Output: 3
// Example 2:

// Input: [0,1,0,1,0,1,99]
// Output: 99

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            // 依次计算32位中每一个bit为1的和，余3后，剩余的1即为不同的数字
            let mut sum_1_of_index_i = 0;
            for j in 0..nums.len() {
                sum_1_of_index_i += (nums[j] >> i) & 1;
            }
            result |= (sum_1_of_index_i % 3) << i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_test() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
