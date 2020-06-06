// 1. Two Sum
// Easy

// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 暴力枚举法
        // let len = nums.len();
        // for i in 0..len {
        //     for j in i+1..len {
        //         if nums[i] + nums[j] == target {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }
        // vec![]

        // 利用HashMap, 一次遍历
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            // 这里必须得使用&符号, other_index也必须使用*号
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, index);
                }
                Some(other_index) => return vec![*other_index as i32, index as i32],
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
