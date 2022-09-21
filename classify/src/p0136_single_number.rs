// 136. Single Number
// Easy

// Given a non-empty array of integers, every element appears twice except for one. Find that single one.

// Note:

// Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

// Example 1:

// Input: [2,2,1]
// Output: 1
// Example 2:

// Input: [4,1,2,1,2]
// Output: 4

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // 利用'异或' 位运算
        nums.iter().fold(0, |result, num| result ^ num)

        // let mut result = 0;
        // for num in &nums {
        //     result ^= num;
        // }
        // result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_test() {
        assert_eq!(Solution::single_number(vec![0]), 0);
        assert_eq!(Solution::single_number(vec![1]), 1);
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
