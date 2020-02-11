// 1091. Shortest Path in Binary Matrix
// Medium

// In an N by N square grid, each cell is either empty (0) or blocked (1).

// A clear path from top-left to bottom-right has length k if and only if it is composed of cells C_1, C_2, ..., C_k such that:

// Adjacent cells C_i and C_{i+1} are connected 8-directionally (ie., they are different and share an edge or corner)
// C_1 is at location (0, 0) (ie. has value grid[0][0])
// C_k is at location (N-1, N-1) (ie. has value grid[N-1][N-1])
// If C_i is located at (r, c), then grid[r][c] is empty (ie. grid[r][c] == 0).
// Return the length of the shortest such clear path from top-left to bottom-right.  If such a path does not exist, return -1.

// Example 1:

// Input: [[0,1],[1,0]]

// Output: 2

// Example 2:

// Input: [[0,0,0],[1,1,0],[1,1,0]]

// Output: 4

// Note:

// 1 <= grid.length == grid[0].length <= 100
// grid[r][c] is 0 or 1

pub struct Solution {}

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }
        if n == 1 && grid[0][0] == 0 {
            return 1;
        }

        let direction: Vec<Vec<i32>> = vec![
            vec![1, -1],
            vec![1, 0],
            vec![1, 1],
            vec![0, -1],
            vec![0, 1],
            vec![-1, -1],
            vec![-1, 0],
            vec![-1, 1],
        ];
        let mut visit: Vec<Vec<bool>> = vec![vec![false; n]; n];

        let mut queue: VecDeque<_> = VecDeque::new();

        queue.push_back((0, 0));
        visit[0][0] = true;

        let mut steps = 0;
        while !queue.is_empty() {
            steps += 1;
            let mut size = queue.len();
            while size > 0 {
                let (cur_x, cur_y) = match queue.pop_front() {
                    Some((cur_x, cur_y)) => (cur_x, cur_y),
                    None => (0, 0)
                };
                for i in 0..8 {
                    let next_x = cur_x + direction[i][0];
                    let next_y = cur_y + direction[i][1];
                    if next_x < 0
                        || next_x >= (n as i32)
                        || next_y < 0
                        || next_y >= (n as i32)
                        || visit[next_x as usize][next_y as usize]
                        || grid[next_x as usize][next_y as usize] == 1
                    {
                        continue;
                    }
                    if (next_x as usize) == n - 1usize && (next_y as usize) == n - 1usize {
                        return steps + 1;
                    }
                    queue.push_back((next_x, next_y));
                    visit[next_x as usize][next_y as usize] = true;
                }
                size -= 1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_path_binary_matrix_test() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![1, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 1]
            ]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            4
        );
        // 不能够走到终点
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 1, 0]
            ]),
            -1
        );
        assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0]]), 1);
    }
}
