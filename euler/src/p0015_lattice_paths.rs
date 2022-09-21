// Lattice paths
// Problem 15
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.


// How many such routes are there through a 20×20 grid?

pub struct Solution {}

impl Solution {
    pub fn lattice_paths(n: usize) -> u64 {
        if n > 20 {
            panic!("Plz set n <= 20");
        }
        let mut grid = [[0; 21]; 21];
        // grid[n][n] 表示从(0,0) => (n,n)能走的频数
        // 使用递推，由于只能向右或者向下走，故grid[n][n] = grid[n-1][n] + grid[n][n-1], 并注意边缘
        for i in 0..=n {
            for j in 0..=n {
                if i == 0 && j == 0{
                    grid[i][j] = 1u64;
                } else if i == 0 && j > 0 {
                    grid[i][j] = grid[i][j-1];
                } else if i > 0 && j == 0 {
                    grid[i][j] = grid[i-1][j];
                } else {
                    grid[i][j] = grid[i-1][j] + grid[i][j-1];
                }
            }
        }
        grid[n][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lattice_paths_test() {
        assert_eq!(Solution::lattice_paths(0), 1);
        assert_eq!(Solution::lattice_paths(1), 2);
        assert_eq!(Solution::lattice_paths(2), 6);
        assert_eq!(Solution::lattice_paths(3), 20);
        assert_eq!(Solution::lattice_paths(19), 35345263800);
        assert_eq!(Solution::lattice_paths(20), 137846528820);
    }
}
