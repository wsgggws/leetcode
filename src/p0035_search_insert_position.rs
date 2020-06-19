// 35. Search Insert Position
// Easy

// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You may assume no duplicates in the array.

// Example 1:

// Input: [1,3,5,6], 5
// Output: 2
// Example 2:

// Input: [1,3,5,6], 2
// Output: 1
// Example 3:

// Input: [1,3,5,6], 7
// Output: 4
// Example 4:

// Input: [1,3,5,6], 0
// Output: 0

pub struct Solution {}

impl Solution {
    // 直接使用模板2, 左开右闭[l, r]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = start + (end - start) / 2;
            if nums[mid] >= target {
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        // 二分搜索最后start == end, 如果数组里不存在，start就是该值存在的索引
        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_test() {
        // 有，最前面
        assert_eq!(Solution::search_insert(vec![1, 3], 1), 0);
        // 有，最后面
        assert_eq!(Solution::search_insert(vec![1, 3], 3), 1);
        // 有，在中间
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        // 没有，应该在中间
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        // 没有，在最后面
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        // 没有，在最前面
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
