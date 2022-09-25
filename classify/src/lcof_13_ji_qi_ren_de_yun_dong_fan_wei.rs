// 剑指 Offer 13. 机器人的运动范围
// 难度
// 中等

// 地上有一个m行n列的方格，从坐标 [0,0] 到坐标 [m-1,n-1] 。一个机器人从坐标 [0, 0] 的格子开始移动，它每次可以向左、右、上、下移动一格（不能移动到方格外），也不能进入行坐标和列坐标的数位之和大于k的格子。例如，当k为18时，机器人能够进入方格 [35, 37] ，因为3+5+3+7=18。但它不能进入方格 [35, 38]，因为3+5+3+8=19。请问该机器人能够到达多少个格子？



// 示例 1：

// 输入：m = 2, n = 3, k = 1
// 输出：3
// 示例 2：

// 输入：m = 3, n = 1, k = 0
// 输出：1
// 提示：

// 1 <= n,m <= 100
// 0 <= k <= 20

pub struct Solution {}

use std::collections::VecDeque;
impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visit: Vec<Vec<bool>> = vec![vec![false; n as usize]; m as usize];

        let mut count = 1;
        queue.push_back((0, 0));
        visit[0][0] = true;
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            for &dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let next_i = i + dir.0;
                let next_j = j + dir.1;
                if Solution::is_valid(next_i, next_j, m, n, k) && !visit[next_i as usize][next_j as usize] {
                    queue.push_back((next_i, next_j));
                    visit[next_i as usize][next_j as usize] = true;
                    count += 1;
                }
            }
        }

        count
    }
    fn is_valid(i: i32, j: i32, m: i32, n: i32, k: i32) -> bool {
        if i < 0 || i >= m || j < 0 || j >= n || Solution::get_dot_sum(i) + Solution::get_dot_sum(j) > k {
            false
        } else {
            true
        }
    }

    fn get_dot_sum(i: i32) -> i32 {
        let mut sum = 0;
        let mut i = i;
        while i > 0 {
            sum += i % 10;
            i /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_count_test() {
        assert_eq!(Solution::moving_count(2, 3, 1), 3);
        assert_eq!(Solution::moving_count(3, 1, 0), 1);
    }
}
