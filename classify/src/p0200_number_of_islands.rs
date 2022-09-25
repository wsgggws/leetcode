// 200. Number of Islands
// Medium

// Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.



// Example 1:

// Input: grid = [
//   ["1","1","1","1","0"],
//   ["1","1","0","1","0"],
//   ["1","1","0","0","0"],
//   ["0","0","0","0","0"]
// ]
// Output: 1
// Example 2:

// Input: grid = [
//   ["1","1","0","0","0"],
//   ["1","1","0","0","0"],
//   ["0","0","1","0","0"],
//   ["0","0","0","1","1"]
// ]
// Output: 3

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let row = grid.len();
        if row == 0 { return 0; }
        let col = grid[0].len();
        if col == 0 { return 0; }

        let mut visit: Vec<Vec<bool>> = vec![vec![false; col]; row];
        let mut count = 0;
        for i in 0..row {
            for j in 0..col {
                if !visit[i][j] && grid[i][j] == '1' {
                    count += 1;
                    Solution::bfs(&grid, &mut visit, i, j);
                }
            }
        }
        count
    }

    fn bfs(grid: &Vec<Vec<char>>, visit: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        visit[i][j] = true;
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for &dir in dirs.iter() {
            let next_i = i as i32 + dir.0;
            let next_j = j as i32 + dir.1;
            if next_i >= 0 && next_i < visit.len() as i32 && next_j >= 0 && next_j < visit[0].len() as i32 {
                if !visit[next_i as usize][next_j as usize] && grid[next_i as usize][next_j as usize] == '1' {
                    Solution::bfs(grid, visit, next_i as usize, next_j as usize);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_islands_test() {
        assert_eq!(Solution::num_islands(vec![]), 0);
        assert_eq!(Solution::num_islands(vec![vec![] ]), 0);
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
