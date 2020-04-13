// 453. Minimum Moves to Equal Array Elements
// Easy

// Given a non-empty integer array of size n, find the minimum number of moves required to make all array elements equal, where a move is incrementing n - 1 elements by 1.

// Example:

// Input:
// [1,2,3]

// Output:
// 3

// Explanation:
// Only three moves are needed (remember each move increments two elements):

//     [1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]

pub struct Solution {}

// 每次对 n-1 个数进行 加1 <==> 对1个最大的数 减1
// i.e.
// 1, 2, 3 => 2, 3, 3 => 3, 4, 3 => 4, 4, 4
// <===>
// 1, 2, 3 => 1, 2, 2 => 1, 1, 2 => 1, 1, 1
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i64;
        let min_value: i64 = (*nums.iter().min().unwrap()) as i64;
        let mut sum = 0_i64;
        for num in nums.iter() {
            sum += *num as i64;
        }
        (sum - len * min_value) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_moves_test() {
        assert_eq!(Solution::min_moves(vec![-1, 1]), 2);
        assert_eq!(Solution::min_moves(vec![-1, 0, 1]), 3);
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
        assert_eq!(Solution::min_moves(vec![1, 1, 9]), 8);
        assert_eq!(Solution::min_moves(vec![-100, 0, 100]), 300);
        assert_eq!(Solution::min_moves(vec![1, 1, 2147483647]), 2147483646);
    }
}
