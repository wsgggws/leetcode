// 130. Surrounded Regions
// Medium

// Given a 2D board containing 'X' and 'O' (the letter O), capture all regions surrounded by 'X'.

// A region is captured by flipping all 'O's into 'X's in that surrounded region.

// Example:

// X X X X
// X O O X
// X X O X
// X O X X
// After running your function, the board should be:

// X X X X
// X X X X
// X X X X
// X O X X
// Explanation:

// Surrounded regions shouldn’t be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.

pub struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let row = board.len();
        if row <= 2 {
            return;
        }
        let col = board[0].len();
        if col <= 2 {
            return;
        }
        // 从边框上的'O'出发，dfs搜索并标记为'G'
        for j in 0..col {
            if board[0][j] == 'O' {
                Solution::dfs(board, 0, j);
            }
            if board[row-1][j] == 'O' {
                Solution::dfs(board, row-1, j);
            }
        }
        for i in 0..row {
            if board[i][0] == 'O' {
                Solution::dfs(board, i, 0);
            }
            if board[i][col-1] == 'O' {
                Solution::dfs(board, i, col-1);
            }
        }
        // 而后改变’G'为'O', 剩下的'O'必定是被包围的，改成为'X'
        for i in 0..row {
            for j in 0..col {
                if board[i][j] == 'G' {
                    board[i][j] = 'O';
                } else if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        board[i][j] = 'G';
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let next_i: i32 = i as i32 + dir.0;
            let next_j: i32 = j as i32 + dir.1;
            if Solution::is_valid(next_i, next_j, board.len(), board[0].len())
                && board[next_i as usize][next_j as usize] == 'O'
            {
                board[next_i as usize][next_j as usize] = 'G';
                Solution::dfs(board, next_i as usize, next_j as usize);
            }
        }
    }

    fn is_valid(next_i: i32, next_j: i32, n: usize, m: usize) -> bool {
        if next_i < 0 || next_i >= n as i32 || next_j < 0 || next_j >= m as i32 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X']
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X']
        ]);

        let mut board = vec![
            vec!['X','O','X'],
            vec!['X','O','X'],
            vec!['X','O','X']
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec![
            vec!['X','O','X'],
            vec!['X','O','X'],
            vec!['X','O','X']
        ]);
    }
}
