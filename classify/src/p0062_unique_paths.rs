// 62. Unique Paths
// Medium

// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

// The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

// How many possible unique paths are there?

// Above is a 7 x 3 grid. How many possible unique paths are there?

// Note: m and n will be at most 100.

// Example 1:

// Input: m = 3, n = 2
// Output: 3
// Explanation:
// From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
// 1. Right -> Right -> Down
// 2. Right -> Down -> Right
// 3. Down -> Right -> Right
// Example 2:

// Input: m = 7, n = 3
// Output: 28

pub struct Solution {}

impl Solution {
    // 利用数学方法 组合 来计算
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // 总移动次数为 m+n-2
        let total_steps = (m + n - 2) as i64;
        // 向下移动次数为 m-1
        let down_steps = (m - 1) as i64;
        let mut result = 1_i64;
        // 结果为组合数C(m+n-2, m-1)
        for i in 1..=down_steps {
            result = result * (total_steps - down_steps + i) / i;
        }
        result as i32
    }

    // 使用动态规划
    pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; n as usize]; m as usize];
        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i-1][j] + dp[i][j-1]
            }
        }
        dp[m as usize- 1][n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_paths_test() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(23, 12), 193536720);
    }

    #[test]
    fn unique_paths_dp_test() {
        assert_eq!(Solution::unique_paths_dp(3, 2), 3);
        assert_eq!(Solution::unique_paths_dp(7, 3), 28);
        assert_eq!(Solution::unique_paths_dp(23, 12), 193536720);
    }
}
