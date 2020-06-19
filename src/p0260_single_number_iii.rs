// 260. Single Number III
// Medium

// Given an array of numbers nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once.

// Example:

// Input:  [1,2,1,3,2,5]
// Output: [3,5]
// Note:

// The order of the result is not important. So in the above example, [5, 3] is also correct.
// Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // 先计算a^b, 然后根据最后一个1来区分a或者b
        let a_xor_b = nums.iter().fold(0, |result, num| result ^ num);
        let mut result = vec![a_xor_b; 2];
        let the_last_diff_1 = a_xor_b & (a_xor_b - 1) ^ a_xor_b;
        for i in 0..nums.len() {
            match the_last_diff_1 & nums[i] {
                0 => result[0] ^= nums[i],
                _ => result[1] ^= nums[i],
            }
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_test() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    }
}
