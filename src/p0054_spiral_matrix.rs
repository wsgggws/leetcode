// 54. Spiral Matrix
// Medium

// Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.

// Example 1:

// Input:
// [
//  [ 1, 2, 3 ],
//  [ 4, 5, 6 ],
//  [ 7, 8, 9 ]
// ]
// Output: [1,2,3,6,9,8,7,4,5]
// Example 2:

// Input:
// [
//   [1, 2, 3, 4],
//   [5, 6, 7, 8],
//   [9,10,11,12]
// ]
// Output: [1,2,3,4,8,12,11,10,9,5,6,7]

pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {

        let row = matrix.len() as i32;
        if row == 0 {
            return vec![];
        }
        let col = matrix[0].len() as i32;
        if col == 0 {
            return vec![];
        }

        let mut visit: Vec<Vec<bool>> = vec![vec![false; col as usize]; row as usize];
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let (mut cur_i, mut cur_j) = (0, 0);
        let mut ans: Vec<i32> = vec![matrix[0][0]];
        visit[0][0] = true;

        let mut steps = 1;
        let mut loops = 0;
        while steps < row * col {
            let mut next_i = cur_i + dirs[loops % 4].0;
            let mut next_j = cur_j + dirs[loops % 4].1;
            if Solution::is_valid(next_i, next_j, row, col) && !visit[next_i as usize][next_j as usize] {
                // 朝着某个方向走完
                while Solution::is_valid(next_i, next_j, row, col) && !visit[next_i as usize][next_j as usize] {
                    ans.push(matrix[next_i as usize][next_j as usize]);
                    visit[next_i as usize][next_j as usize] = true;
                    steps += 1;

                    cur_i = next_i;
                    cur_j = next_j;
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

    fn is_valid(next_i: i32, next_j: i32, row: i32, col: i32) -> bool {
        if next_i < 0 || next_i >= row || next_j < 0 || next_j >= col {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_order_test() {
        assert_eq!(
            Solution::spiral_order(vec![]),
            vec![]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![]]),
            vec![]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![3], vec![2]]),
            vec![3, 2]
        );
    }
}
