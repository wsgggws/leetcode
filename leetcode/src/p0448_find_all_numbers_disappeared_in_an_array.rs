// 448. Find All Numbers Disappeared in an Array
// Easy

// Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others appear once.

// Find all the elements of [1, n] inclusive that do not appear in this array.

// Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

// Example:

// Input:
// [4,3,2,7,8,2,3,1]

// Output:
// [5,6]

pub struct Solution {}


impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let value = i32::abs(nums[i]) as usize - 1_usize;
            nums[value] = -i32::abs(nums[value]);
        }
        let mut result = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 { result.push((i+1_usize) as i32); }
        }
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_disappeared_numbers_test() {
        assert_eq!(Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![5, 6]);
    }
}
