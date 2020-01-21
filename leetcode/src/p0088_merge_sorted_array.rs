// 88. Merge Sorted Array
// Easy

// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.

// Note:

// The number of elements initialized in nums1 and nums2 are m and n respectively.
// You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
// Example:

// Input:
// nums1 = [1,2,3,0,0,0], m = 3
// nums2 = [2,5,6],       n = 3

// Output: [1,2,2,3,5,6]

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index_i = m - 1;
        let mut index_j = n - 1;
        let mut index = m + n - 1;
        while index_i >= 0 || index_j >= 0 {
            if index_i < 0 {
                nums1[index as usize] = nums2[index_j as usize];
                index -= 1;
                index_j -= 1;
            } else if index_j < 0 {
                nums1[index as usize] = nums1[index_i as usize];
                index -= 1;
                index_i -= 1;
            } else if nums1[index_i as usize] > nums2[index_j as usize] {
                nums1[index as usize] = nums1[index_i as usize];
                index -= 1;
                index_i -= 1;
            } else {
                nums1[index as usize] = nums2[index_j as usize];
                index -= 1;
                index_j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn mer_test() {
        let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
        assert_eq!(nums2, vec![2, 5, 6]);
    }
}
