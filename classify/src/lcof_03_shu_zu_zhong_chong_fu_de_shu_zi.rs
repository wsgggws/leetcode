// 剑指 Offer 03. 数组中重复的数字
// 找出数组中重复的数字。


// 在一个长度为 n 的数组 nums 里的所有数字都在 0～n-1 的范围内。数组中某些数字是重复的，但不知道有几个数字重复了，也不知道每个数字重复了几次。请找出数组中任意一个重复的数字。

// 示例 1：

// 输入：
// [2, 3, 1, 0, 2, 5, 3]
// 输出：2 或 3


// 限制：

// 2 <= n <= 100000

pub struct Solution {}

impl Solution {
    // 1. 排序后查找 O(nlogn) O(1)
    // 2. 利用HashSet O(n) O(n)

    // O(n) O(1)
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut array = nums.clone();
        for i in 0..nums.len() {
            while array[i] != i as i32 {
                let j = array[i] as usize;
                if array[i] == array[j] {
                    return array[i];
                }
                array.swap(i, j);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_repeat_number_test() {
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0]), -1);
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 3, 0]), 3);
    }
}
