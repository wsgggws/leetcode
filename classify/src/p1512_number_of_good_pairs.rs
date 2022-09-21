// 1512. Number of Good PairsMy SubmissionsBack to Contest
// Difficulty:Easy
// Given an array of integers nums.

// A pair (i,j) is called good if nums[i] == nums[j] and i < j.

// Return the number of good pairs.

 

// Example 1:

// Input: nums = [1,2,3,1,1,3]
// Output: 4
// Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
// Example 2:

// Input: nums = [1,1,1,1]
// Output: 6
// Explanation: Each pair in the array are good.
// Example 3:

// Input: nums = [1,2,3]
// Output: 0
 

// Constraints:

// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

pub struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] == nums[j] {
                    result += 1;
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
    fn num_identical_pairs_test() {
        assert_eq!(Solution:: num_identical_pairs(vec![1, 2, 3]), 0);
        assert_eq!(Solution:: num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution:: num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }
}
