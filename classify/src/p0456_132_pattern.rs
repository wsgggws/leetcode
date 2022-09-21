// 456. 132 Pattern
// Medium

// Given a sequence of n integers a1, a2, ..., an, a 132 pattern is a subsequence ai, aj, ak such that i < j < k and ai < ak < aj. Design an algorithm that takes a list of n numbers as input and checks whether there is a 132 pattern in the list.

// Note: n will be less than 15,000.

// Example 1:
// Input: [1, 2, 3, 4]

// Output: False

// Explanation: There is no 132 pattern in the sequence.
// Example 2:
// Input: [3, 1, 4, 2]

// Output: True

// Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
// Example 3:
// Input: [-1, 3, 2, 0]

// Output: True

// Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].

pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut num3 = std::i32::MAX;
        for j in 0..nums.len() {
            num3 = i32::min(num3, nums[j]);
            if num3 == nums[j] {
                continue;
            }
            for k in (j..nums.len()).rev() {
                if num3 < nums[k] && nums[j] > nums[k] {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find132pattern_test() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
        assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    }
}
