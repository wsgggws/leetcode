// 75. Sort Colors
// Medium

// Given an array with n objects colored red, white or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white and blue.

// Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.

// Note: You are not suppose to use the library's sort function for this problem.

// Example:

// Input: [2,0,2,1,1,0]
// Output: [0,0,1,1,2,2]
// Follow up:

// A rather straight forward solution is a two-pass algorithm using counting sort.
// First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
// Could you come up with a one-pass algorithm using only constant space?

pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        for index in 0..nums.len() {
            if nums[index] == 0 {
                nums.swap(index, i);
                i += 1;
            }
        }
        for index in 0..nums.len() {
            if nums[index] == 1 {
                nums.swap(index, i);
                i += 1;
            }
        }
    }
    // 双指针
    pub fn sort_colors_pointer(nums: &mut Vec<i32>) {
        let (mut cur, mut left, mut right) = (0i32, 0i32, nums.len() as i32 - 1);
        // 这里right可能出现负数，所以必须使用i32类型, 在索引时又得使用 as usize
        while cur <= right {
            if nums[cur as usize] == 0 {
                nums.swap(cur as usize, left as usize);
                cur += 1;
                left += 1;
            } else if nums[cur as usize] == 2 {
                nums.swap(cur as usize, right as usize);
                right -= 1;
            } else{
                cur += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_colors_test() {
        let mut colors = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut colors);
        assert_eq!(colors, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn sort_colors_pointer_test() {
        let mut colors = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors_pointer(&mut colors);
        assert_eq!(colors, vec![0, 0, 1, 1, 2, 2]);
    }
}
