// 350. Intersection of Two Arrays II
// Easy

// Given two arrays, write a function to compute their intersection.

// Example 1:

// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2,2]
// Example 2:

// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [4,9]
// Note:

// Each element in the result should appear as many times as it shows in both arrays.
// The result can be in any order.
// Follow up:

// What if the given array is already sorted? How would you optimize your algorithm?
// What if nums1's size is small compared to nums2's size? Which algorithm is better?
// What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let hash_map1 = Solution::get_hash_map(nums1);
        let hash_map2 = Solution::get_hash_map(nums2);
        let mut result: Vec<i32> = vec![];

        for (&k, &v) in hash_map1.iter() {
            if let Some(&value) = hash_map2.get(&k) {
                for _ in 0..i32::min(v, value) {
                    result.push(k);
                }
            }
        }
        // 提交时取注释此行，题目要求解可以是任意序的
        result.sort();
        result
    }

    fn get_hash_map(nums: Vec<i32>) -> HashMap<i32, i32> {
        let mut hash_map = HashMap::new();
        for &num in nums.iter() {
            let counter = hash_map.entry(num).or_insert(0);
            *counter += 1;
        }
        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_test() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }
}
