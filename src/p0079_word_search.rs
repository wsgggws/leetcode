// 79. Word Search
// Medium

// Given a 2D board and a word, find if the word exists in the grid.

// The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.

// Example:

// board =
// [
//   ['A','B','C','E'],
//   ['S','F','C','S'],
//   ['A','D','E','E']
// ]

// Given word = "ABCCED", return true.
// Given word = "SEE", return true.
// Given word = "ABCB", return false.

// Constraints:

// board and word consists only of lowercase and uppercase English letters.
// 1 <= board.length <= 200
// 1 <= board[i].length <= 200
// 1 <= word.length <= 10^3

pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let row = board.len();
        if row == 0 {
            return false;
        }
        let col = board[0].len();
        if col == 0 {
            return false;
        }
        if word.len() > row * col {
            return false;
        }

        let mut ans = false;
        let word_chars: Vec<char> = word.chars().collect();
        for i in 0..row {
            for j in 0..col {
                if board[i][j] == word_chars[0] {
                    let mut visit: Vec<Vec<bool>> = vec![vec![false; col]; row];
                    visit[i][j] = true;
                    Solution::dfs(&board, &mut visit, i, j, &word_chars, 0, &mut ans);
                }
                if ans {
                    return true
                }
            }
        }
        ans
    }
    fn dfs(board: &Vec<Vec<char>>, visit: &mut Vec<Vec<bool>>, i: usize, j: usize, word_chars: &Vec<char>, cur_index: usize, ans: &mut bool) {
        if *ans {
            return;
        }
        if cur_index + 1 == word_chars.len() {
            *ans = true;
            return;
        }
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let next_i: i32 = i as i32 + dir.0;
            let next_j: i32 = j as i32 + dir.1;
            if Solution::is_valid(next_i, next_j, visit.len(), visit[0].len())
                && !visit[next_i as usize][next_j as usize]
                && word_chars[cur_index + 1] == board[next_i as usize][next_j as usize]
            {
                visit[next_i as usize][next_j as usize] = true;
                Solution::dfs(board, visit, next_i as usize, next_j as usize, word_chars, cur_index + 1, ans);
                visit[next_i as usize][next_j as usize] = false;

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
    fn exist_test() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_owned()
            ),
            true
        );

        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_owned()
            ),
            true
        );

        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_owned()
            ),
            false
        );
    }
}
