// 414. Third Maximum Number
// Easy

// Given a non-empty array of integers, return the third maximum number in this array. If it does not exist, return the maximum number. The time complexity must be in O(n).

// Example 1:
// Input: [3, 2, 1]

// Output: 1

// Explanation: The third maximum is 1.
// Example 2:
// Input: [1, 2]

// Output: 2

// Explanation: The third maximum does not exist, so the maximum (2) is returned instead.
// Example 3:
// Input: [2, 2, 3, 1]

// Output: 1

// Explanation: Note that the third maximum here means the third maximum distinct number.
// Both numbers with value 2 are both considered as second maximum.

pub struct Solution {}


impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max_first = i32::min_value();
        let mut max_second = i32::min_value();
        let mut max_third = i32::min_value();

        let mut is_exist_min_value = false;
        for i in 0..nums.len() {

            if nums[i] == i32::min_value() { is_exist_min_value = true; }

            if nums[i] > max_first { max_third = max_second; max_second = max_first; max_first = nums[i]; }
            else if nums[i] > max_second && nums[i] != max_first { max_third = max_second; max_second = nums[i]; }
            else if nums[i] > max_third && nums[i] != max_second && nums[i] != max_first { max_third = nums[i]; }
        }

        // 存在 max_third 的情况, 第三大值确实是 i32::min_value()
        if is_exist_min_value && (max_second != i32::min_value()) { return max_third; }
        // 存在 max_third 的情况, 第三大值> i32::min_value()
        else if max_third > i32::min_value() { return max_third; }

        // 不存在 max_third 的情况
        else { return max_first; }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn third_max_test() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![2, 1]), 2);
        assert_eq!(Solution::third_max(vec![3, 2, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![2, 2, 2, 2]), 2);
        assert_eq!(Solution::third_max(vec![3, 2, 2, 2]), 3);
        assert_eq!(Solution::third_max(vec![1, 2, 2, 5, 3, 5]), 2);
        assert_eq!(Solution::third_max(vec![-2147483648, 1, 1]), 1);
        assert_eq!(Solution::third_max(vec![3, 2, 2, i32::min_value()]), i32::min_value());
        assert_eq!(Solution::third_max(vec![i32::min_value(), 2, 2, i32::min_value()]), 2);
    }
}
