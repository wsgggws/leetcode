// 4. Median of Two Sorted Arrays
// Hard

// There are two sorted arrays nums1 and nums2 of size m and n respectively.

// Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).

// You may assume nums1 and nums2 cannot be both empty.

// Example 1:

// nums1 = [1, 3]
// nums2 = [2]

// The median is 2.0
// Example 2:

// nums1 = [1, 2]
// nums2 = [3, 4]

// The median is (2 + 3)/2 = 2.5

pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let k = (total + 1) / 2;
        if total % 2 == 1 {
            Solution::binary_search_kth_ele(&nums1, 0, &nums2, 0, k) as f64
        } else {
            (
                Solution::binary_search_kth_ele(&nums1, 0, &nums2, 0, k)
                + Solution::binary_search_kth_ele(&nums1, 0, &nums2, 0, k+1)
            ) as f64 / 2.0
        }
    }

    fn binary_search_kth_ele(nums1: &Vec<i32>, nums1_index: usize, nums2: &Vec<i32>, nums2_index: usize, k: usize) -> i32 {
        if nums1_index >= nums1.len() {
            return nums2[nums2_index + k - 1];
        }
        if nums2_index >= nums2.len() {
            return nums1[nums1_index + k - 1];
        }
        if k == 1 {
            return i32::min(nums1[nums1_index], nums2[nums2_index]);
        }
        let nums1_mid = if nums1_index + k / 2 - 1 < nums1.len() { nums1[nums1_index + k / 2 - 1] } else { std::i32::MAX };
        let nums2_mid = if nums2_index + k / 2 - 1 < nums2.len() { nums2[nums2_index + k / 2 - 1] } else { std::i32::MAX };
        if nums1_mid < nums2_mid {
            Solution::binary_search_kth_ele(nums1, nums1_index + k/2, nums2, nums2_index, k - k / 2)
        } else {
            Solution::binary_search_kth_ele(nums1, nums1_index, nums2, nums2_index + k / 2, k - k / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_median_sorted_arrays_test() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
