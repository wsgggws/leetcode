// 695. Max Area of Island
// Medium

// Given a non-empty 2D array grid of 0's and 1's, an island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

// Find the maximum area of an island in the given 2D array. (If there is no island, the maximum area is 0.)

// Example 1:

// [[0,0,1,0,0,0,0,1,0,0,0,0,0],
//  [0,0,0,0,0,0,0,1,1,1,0,0,0],
//  [0,1,1,0,1,0,0,0,0,0,0,0,0],
//  [0,1,0,0,1,1,0,0,1,0,1,0,0],
//  [0,1,0,0,1,1,0,0,1,1,1,0,0],
//  [0,0,0,0,0,0,0,0,0,0,1,0,0],
//  [0,0,0,0,0,0,0,1,1,1,0,0,0],
//  [0,0,0,0,0,0,0,1,1,0,0,0,0]]
// Given the above grid, return 6. Note the answer is not 11, because the island must be connected 4-directionally.
// Example 2:

// [[0,0,0,0,0,0,0,0]]
// Given the above grid, return 0.
// Note: The length of each dimension in the given grid does not exceed 50.

pub struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid[0].len(), grid.len());
        if m == 0 {
            return 0;
        }
        let mut max_area = 0;
        let mut visit: Vec<Vec<bool>> = vec![vec![false; n as usize]; m as usize];
        for i in 0..m {
            for j in 0..n {
                max_area = i32::max(
                    max_area,
                    Solution::dfs(&grid, &mut visit, i as i32, j as i32),
                );
            }
        }
        max_area
    }

    fn dfs(grid: &Vec<Vec<i32>>, visit: &mut Vec<Vec<bool>>, i: i32, j: i32) -> i32 {
        let (n, m) = (grid[0].len() as i32, grid.len() as i32);
        if i < 0
            || i >= m
            || j < 0
            || j >= n
            || grid[i as usize][j as usize] == 0
            || visit[i as usize][j as usize]
        {
            return 0;
        }
        let direction: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
        visit[i as usize][j as usize] = true;
        let mut area = 1;
        for dir in 0..4 {
            area += Solution::dfs(grid, visit, i + direction[dir][0], j + direction[dir][1]);
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area_of_island_test() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
            0
        );
    }
}
