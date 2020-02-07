// 64. Minimum Path Sum
// Medium

// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.

// Note: You can only move either down or right at any point in time.

// Example:

// Input:
// [
//   [1,3,1],
//   [1,5,1],
//   [4,2,1]
// ]
// Output: 7
// Explanation: Because the path 1→3→1→1→1 minimizes the sum.

pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; col]; row];
        for i in 0..row {
            for j in 0..col {
                if i == 0 && j == 0 {
                    // 第一步
                    dp[i][j] = grid[i][j];
                } else if i == 0 && j > 0 {
                    // 只能从右走
                    dp[i][j] = dp[i][j-1] + grid[i][j];
                } else if i > 0 && j == 0 {
                    // 只能从上走
                    dp[i][j] = dp[i-1][j] + grid[i][j];
                } else {
                    // 从右或者从上走, 选择小的
                    dp[i][j] = i32::min(dp[i-1][j], dp[i][j-1]) + grid[i][j];
                }
            }
        }
        dp[row-1][col-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_path_sum_test() {
        assert_eq!(
            Solution::min_path_sum(
                vec![
                 vec![1,3,1],
                 vec![1,5,1],
                 vec![4,2,1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(
                vec![
                 vec![4,2,1],
                 vec![1,5,1]]),
            8
        );
        assert_eq!(
            Solution::min_path_sum(
                vec![
                 vec![4,2,1]]),
            7
        );
    }
}
