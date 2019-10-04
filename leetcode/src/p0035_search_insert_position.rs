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
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target <= nums[0] { return 0 as i32; }
        let len = nums.len();
        if target > nums[len-1] { return len as i32; }
        let mut start = 0;
        let mut end = len - 1;
        // 使用二分法进行查找
        while start < end {
            let mid = (start + end) / 2;
            if nums[mid] < target && target <= nums[mid+1] { return (mid+1) as i32; }
            else if target <= nums[mid] { end = mid; }
            else if target > nums[mid+1] { start = mid; }
        }
        start as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_test() {
        assert_eq!(Solution::search_insert(vec![1, 3], 1), 0);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3], 3), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
