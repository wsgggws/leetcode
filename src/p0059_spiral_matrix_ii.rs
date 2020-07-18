// 59. Spiral Matrix II
// Medium

// Given a positive integer n, generate a square matrix filled with elements from 1 to n2 in spiral order.

// Example:

// Input: 3
// Output:
// [
//  [ 1, 2, 3 ],
//  [ 8, 9, 4 ],
//  [ 7, 6, 5 ]
// ]

pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let row = n as usize;
        let col = n as usize;

        let mut visit: Vec<Vec<bool>> = vec![vec![false; col]; row];
        let mut ans: Vec<Vec<i32>> = vec![vec![1; col]; row];
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let (mut cur_i, mut cur_j) = (0, 0);
        ans[cur_i][cur_j] = 1;
        visit[0][0] = true;

        let mut steps = 1i32;
        let mut loops = 0;
        while steps < n * n {
            let mut next_i = cur_i as i32 + dirs[loops % 4].0;
            let mut next_j = cur_j as i32 + dirs[loops % 4].1;
            if Solution::is_valid(next_i, next_j, n) && !visit[next_i as usize][next_j as usize] {
                // 朝着某个方向走完
                while Solution::is_valid(next_i, next_j, n)
                    && !visit[next_i as usize][next_j as usize]
                {
                    ans[next_i as usize][next_j as usize] = steps + 1;
                    visit[next_i as usize][next_j as usize] = true;
                    steps += 1;

                    cur_i = next_i as usize;
                    cur_j = next_j as usize;
                    next_i = next_i + dirs[loops % 4].0;
                    next_j = next_j + dirs[loops % 4].1;
                }
                // 改变方向
                loops += 1;
            } else {
                loops += 1;
            }
        }
        ans
    }

    fn is_valid(next_i: i32, next_j: i32, n: i32) -> bool {
        if next_i < 0 || next_i >= n || next_j < 0 || next_j >= n {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_matrix_test() {
        assert_eq!(
            Solution::generate_matrix(1),
            vec![
                vec![1]
            ]
        );
        assert_eq!(
            Solution::generate_matrix(3),
            vec![
                vec![1, 2, 3],
                vec![8, 9, 4],
                vec![7, 6, 5]
            ]
        );
        assert_eq!(
            Solution::generate_matrix(2),
            vec![
                vec![1, 2],
                vec![4, 3]
            ]
        );
        assert_eq!(
            Solution::generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }
}
