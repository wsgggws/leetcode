// 283. Move Zeroes
// Easy

// Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// Example:

// Input: [0,1,0,3,12]
// Output: [1,3,12,0,0]
// Note:

// You must do this in-place without making a copy of the array.
// Minimize the total number of operations.

pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut cur_index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                cur_index += 1;
            } else {
                let mut j = i;
                while j < nums.len() && nums[j] == 0 {
                    j += 1;
                }
                if j == nums.len() {
                    break;
                } else {
                    nums[cur_index] = nums[j];
                    nums[j] = 0;
                    cur_index += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_zeroes_test() {
        let mut array = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut array);
        assert_eq!(array, vec![1, 3, 12, 0, 0]);

        let mut array = vec![0, 0, 0, 3, 12];
        Solution::move_zeroes(&mut array);
        assert_eq!(array, vec![3, 12, 0, 0, 0]);

        let mut array = vec![1, 0, 0, 0, 0];
        Solution::move_zeroes(&mut array);
        assert_eq!(array, vec![1, 0, 0, 0, 0]);

        let mut array = vec![0, 0, 0, 0, 1];
        Solution::move_zeroes(&mut array);
        assert_eq!(array, vec![1, 0, 0, 0, 0]);

        let mut array = vec![0, 0, 1, 0, 1];
        Solution::move_zeroes(&mut array);
        assert_eq!(array, vec![1, 1, 0, 0, 0]);
    }
}
