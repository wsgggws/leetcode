// 349. Intersection of Two Arrays
// Easy

// Given two arrays, write a function to compute their intersection.

// Example 1:

// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]
// Example 2:

// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]
// Note:

// Each element in the result must be unique.
// The result can be in any order.

pub struct Solution {}


// use std::collections::HashSet;
use std::collections::BTreeSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // let nums_set1: HashSet<i32> = nums1.iter().cloned().collect();
        // let nums_set2: HashSet<i32> = nums2.iter().cloned().collect();
        let nums_set1: BTreeSet<i32> = nums1.iter().cloned().collect();
        let nums_set2: BTreeSet<i32> = nums2.iter().cloned().collect();
        nums_set1.intersection(&nums_set2).cloned().collect::<Vec<i32>>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_test() {
        assert_eq!(Solution::intersection(vec![], vec![2, 2]), vec![]);
        assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}
