// 560. Subarray Sum Equals K
// Medium

// Given an array of integers and an integer k, you need to find the total number of continuous subarrays whose sum equals to k.

// Example 1:

// Input:nums = [1,1,1], k = 2
// Output: 2


// Constraints:

// The length of the array is in range [1, 20,000].
// The range of numbers in the array is [-1000, 1000] and the range of the integer k is [-1e7, 1e7].

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut presums = vec![0; nums.len() + 1];
        let mut sum = 0;
        for i in 0..nums.len() {
            presums[i] = sum;
            sum += nums[i];
        }
        presums[nums.len()] = sum;

        let mut count = 0;
        for i in 0..=nums.len() {
            for j in i + 1..=nums.len() {
                if presums[j] - presums[i] == k {
                    count += 1;
                }
            }
        }
        count
    }
    // TODO, 可以使用HashMap进行优化成O(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarray_sum_test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }
}
