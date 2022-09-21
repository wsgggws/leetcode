// 73. Set Matrix Zeroes
// Medium

// Given a m x n matrix, if an element is 0, set its entire row and column to 0. Do it in-place.

// Example 1:

// Input: 
// [
//   [1,1,1],
//   [1,0,1],
//   [1,1,1]
// ]
// Output: 
// [
//   [1,0,1],
//   [0,0,0],
//   [1,0,1]
// ]
// Example 2:

// Input: 
// [
//   [0,1,2,0],
//   [3,4,5,2],
//   [1,3,1,5]
// ]
// Output: 
// [
//   [0,0,0,0],
//   [0,4,5,0],
//   [0,3,1,0]
// ]
// Follow up:

// A straight forward solution using O(mn) space is probably a bad idea.
// A simple improvement uses O(m + n) space, but still not the best solution.
// Could you devise a constant space solution?

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    // 空间复杂度O(n+m), 使用原数组的0行0列即可实现O(1)的要求
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }

        let mut set_i: HashSet<usize> = HashSet::new();
        let mut set_j: HashSet<usize> = HashSet::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    set_i.insert(i);
                    set_j.insert(j);
                }
            }
        }
        for &index in set_i.iter() {
            for j in 0..matrix[0].len() {
                matrix[index][j] = 0
            }
        }
        for &index in set_j.iter() {
            for i in 0..matrix.len() {
                matrix[i][index] = 0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_zeroes_test() {
        let mut matrix = vec![
            vec![1,1,1],
            vec![1,0,1],
            vec![1,1,1]
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![1,0,1],
            vec![0,0,0],
            vec![1,0,1]
        ]);

        let mut matrix = vec![
            vec![0,1,2,0],
            vec![3,4,5,2],
            vec![1,3,1,5]
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![0,0,0,0],
            vec![0,4,5,0],
            vec![0,3,1,0]
        ]);
    }
}
