// 416. Partition Equal Subset Sum
// Medium

// Given a non-empty array containing only positive integers, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.

// Note:

// Each of the array element will not exceed 100.
// The array size will not exceed 200.

// Example 1:

// Input: [1, 5, 11, 5]

// Output: true

// Explanation: The array can be partitioned as [1, 5, 5] and [11].

// Example 2:

// Input: [1, 2, 3, 5]

// Output: false

// Explanation: The array cannot be partitioned into equal sum subsets.

pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let w = sum / 2;
        let mut dp: Vec<bool> = vec![false; (w + 1) as usize];
        dp[0] = true;
        for num in nums {
            for i in (num..=w).rev() {
                dp[i as usize] = dp[i as usize] || dp[(i - num) as usize];
            }
        }
        dp[w as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_partition_test() {
        assert_eq!(Solution::can_partition(vec![1, 2, 7]), false);
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5, 7]), true);
    }
}
