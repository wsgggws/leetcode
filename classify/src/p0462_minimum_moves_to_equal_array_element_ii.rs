// 462. Minimum Moves to Equal Array Elements II
// Medium

// Given a non-empty integer array, find the minimum number of moves required to make all array elements equal, where a move is incrementing a selected element by 1 or decrementing a selected element by 1.

// You may assume the array's length is at most 10,000.

// Example:

// Input:
// [1,2,3]

// Output:
// 2

// Explanation:
// Only two moves are needed (remember each move increments or decrements one element):

// [1,2,3]  =>  [2,2,3]  =>  [2,2,2]

pub struct Solution {}

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1usize {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let (mut i, mut j) = (0usize, nums.len() - 1usize);
        let mut result = 0;
        while i <= j {
            result += nums[j] - nums[i];
            i += 1usize;
            j -= 1usize;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_moves2_test() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        assert_eq!(Solution::min_moves2(vec![2, 1, 3]), 2);
    }
}
