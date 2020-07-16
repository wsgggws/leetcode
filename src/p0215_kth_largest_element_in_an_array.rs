// 215. Kth Largest Element in an Array
// Medium

// Find the kth largest element in an unsorted array. Note that it is the kth largest element in the sorted order, not the kth distinct element.

// Example 1:

// Input: [3,2,1,5,6,4] and k = 2
// Output: 5
// Example 2:

// Input: [3,2,3,1,2,4,5,5,6] and k = 4
// Output: 4
// Note:
// You may assume k is always valid, 1 ≤ k ≤ array's length.

pub struct Solution {}

impl Solution {
    // 直接使用排序算法
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut array = nums.clone();
        array.sort_by(|a, b| b.cmp(a));
        *array.iter().nth(k as usize - 1).unwrap()
    }
    // TODO, 使用快速排序
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_kth_largest_test() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
