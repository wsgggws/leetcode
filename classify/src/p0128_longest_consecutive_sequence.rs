// 128. Longest Consecutive Sequence
// Hard

// Given an unsorted array of integers, find the length of the longest consecutive elements sequence.

// Your algorithm should run in O(n) complexity.

// Example:

// Input: [100, 4, 200, 1, 3, 2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hash_set: HashSet<i32> = nums.clone().into_iter().collect::<HashSet<i32>>();
        let mut result = 0;
        for i in 0..nums.len() {
            if let None = hash_set.get(&nums[i]) {
                continue;
            } else {
                let mut pre = nums[i] - 1;
                let mut next = nums[i] + 1;
                hash_set.remove(&nums[i]);
                while let Some(_value) = hash_set.get(&pre) {
                    hash_set.remove(&pre);
                    pre -= 1;
                } 
                while let Some(_value) = hash_set.get(&next) {
                    hash_set.remove(&next);
                    next += 1;
                } 
                result = i32::max(result, next - pre - 1);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_consecutive_test() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(Solution::longest_consecutive(vec![1]), 1);
        assert_eq!(Solution::longest_consecutive(vec![1, 2]), 2);
        assert_eq!(Solution::longest_consecutive(vec![1, 1, 1]), 1);
    }
}
