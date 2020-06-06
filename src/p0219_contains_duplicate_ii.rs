// 219. Contains Duplicate II
// Easy

// Given an array of integers and an integer k, find out whether there are two distinct indices i and j in the array such that nums[i] = nums[j] and the absolute difference between i and j is at most k.

// Example 1:

// Input: nums = [1,2,3,1], k = 3
// Output: true
// Example 2:

// Input: nums = [1,0,1,1], k = 1
// Output: true
// Example 3:

// Input: nums = [1,2,3,1,2,3], k = 2
// Output: false

pub struct Solution {}

use std::collections::HashMap;
// use std::collections::HashSet;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (index, &num) in nums.iter().enumerate() {
            match map.get(&num) {
                Some(value) => {
                    if index - value <= k as usize {
                        return true;
                    }
                    map.insert(num, index);
                }
                None => {
                    map.insert(num, index);
                }
            }
        }
        false

        // 此方案with k = 3500 不能通过: Time Limit Exceeded
        // if nums.len() == 0_usize || nums.len() == 1_usize || k == 0 { return false; }
        // let k: usize = if k as usize >= nums.len() { nums.len() - 1_usize } else { k as usize };
        // for i in 0..nums.len()-k {
        //     let sub: HashSet<_> = nums[i..=i+k].iter().cloned().collect();
        //     if sub.len() != k+1_usize { return true; }
        // }
        // false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_test() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![], 5), false);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 1], 0), false);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1], 5), false);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2], 1), false);
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 4], 5),
            false
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 1, 3, 4], 1),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}
