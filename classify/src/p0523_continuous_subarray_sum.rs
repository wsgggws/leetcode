// 523. Continuous Subarray Sum
// Medium

// Given a list of non-negative numbers and a target integer k, write a function to check if the array has a continuous subarray of size at least 2 that sums up to a multiple of k, that is, sums up to n*k where n is also an integer.

// Example 1:

// Input: [23, 2, 4, 6, 7],  k=6
// Output: True
// Explanation: Because [2, 4] is a continuous subarray of size 2 and sums up to 6.
// Example 2:

// Input: [23, 2, 6, 4, 7],  k=6
// Output: True
// Explanation: Because [23, 2, 6, 4, 7] is an continuous subarray of size 5 and sums up to 42.

// Constraints:

// The length of the array won't exceed 10,000.
// You may assume the sum of all the numbers is in the range of a signed 32-bit integer.

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        // 0 = 0 * k
        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] == 0 && nums[i + 1] == 0 {
                return true;
            }
        }
        // 如果没有找到连续两个值为0，当k=0时，必为false
        if k == 0 {
            return false;
        }
        if k == 1 || k == -1 {
            return true;
        }
        // a[i] + a[i+1] + a[j] = n1k + q
        // a[i] + a[i+1] + a[j] + a[j+1] + a[n] = n2k + q
        // => a[j+1] + a[n] = (n2-n1)k
        let mut hash_map = HashMap::new();
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if k != 0 {
                sum = sum % k;
            }
            if let Some(&index) = hash_map.get(&sum) {
                if i as i32 > index + 1 {
                    return true;
                }
            } else {
                hash_map.insert(sum, i as i32);
            }
        }
        nums.iter().sum::<i32>() % k == 0 || false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_subarray_sum_test() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
        assert_eq!(Solution::check_subarray_sum(vec![23, 0, 0, 4, 7], 0), true);
        assert_eq!(Solution::check_subarray_sum(vec![0, 0], -1), true);
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], -6), true);
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 0), false);
        assert_eq!(Solution::check_subarray_sum(vec![1, 1], -1), true);
        assert_eq!(Solution::check_subarray_sum(vec![1, 1], -2), true);
        assert_eq!(Solution::check_subarray_sum(vec![1, 2, 3], 6), true);
    }
}
