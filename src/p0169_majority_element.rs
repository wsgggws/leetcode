// 169. Majority Element
// Easy

// Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.

// You may assume that the array is non-empty and the majority element always exist in the array.

// Example 1:

// Input: [3,2,3]
// Output: 3
// Example 2:

// Input: [2,2,1,1,1,2,2]
// Output: 2

pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority = nums[0];
        let mut count = 1;
        for num in &nums[1..] {
            if count == 0 {
                majority = *num;
            }

            if majority == *num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        majority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn majority_element_test() {
        assert_eq!(Solution::majority_element(vec![1, 1, 2]), 1);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
