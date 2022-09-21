// 120. Triangle
// Medium

// Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.

// For example, given the following triangle

// [
//      [2],
//     [3,4],
//    [6,5,7],
//   [4,1,8,3]
// ]
// The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11).

// Note:

// Bonus point if you are able to do this using only O(n) extra space, where n is the total number of rows in the triangle.

pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        // 自底向上写代码更快
        let mut dp = triangle.clone();
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..triangle[i].len() {
                dp[i][j] = triangle[i][j] + i32::min(dp[i + 1][j], dp[i + 1][j + 1])
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_total_test() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
