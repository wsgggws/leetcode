// 217. Contains Duplicate
// Easy

// Given an array of integers, find if the array contains any duplicates.

// Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.

// Example 1:

// Input: [1,2,3,1]
// Output: true
// Example 2:

// Input: [1,2,3,4]
// Output: false
// Example 3:

// Input: [1,1,1,3,3,4,3,2,4,2]
// Output: true

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let sub: HashSet<_> = nums.iter().cloned().collect();
        sub.len() != nums.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_test() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 3, 4]), true);
    }
}
