// 34. Find First and Last Position of Element in Sorted Array
// Medium

// Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.

// Your algorithm's runtime complexity must be in the order of O(log n).

// If the target is not found in the array, return [-1, -1].

// Example 1:

// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]
// Example 2:

// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]

pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 可理解为两次查找左值, 如[2, 2], 2 => 先查找最左边的2, 再查找3可以插入[2, 2]最左哪个位置
        let left = Solution::find_index(&nums, target);
        let right = Solution::find_index(&nums, target + 1) - 1;
        // target大于最右边的值, 或者小于最左边的值
        if left == nums.len() as i32 || nums[left as usize] != target {
            return vec![-1, -1];
        }
        vec![left, right]
    }

    fn find_index(nums: &Vec<i32>, target: i32) -> i32 {
        // 注意r的取值
        let (mut l, mut r) = (0, nums.len() as i32);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m as usize] >= target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn search_range_test() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 5),
            vec![0, 0]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 4),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }
}
