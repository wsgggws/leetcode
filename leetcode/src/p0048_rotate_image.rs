// 48. Rotate Image
// Medium

// You are given an n x n 2D matrix representing an image.

// Rotate the image by 90 degrees (clockwise).

// Note:

// You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.

// Example 1:

// Given input matrix = 
// [
//   [1,2,3],
//   [4,5,6],
//   [7,8,9]
// ],

// rotate the input matrix in-place such that it becomes:
// [
//   [7,4,1],
//   [8,5,2],
//   [9,6,3]
// ]
// Example 2:

// Given input matrix =
// [
//   [ 5, 1, 9,11],
//   [ 2, 4, 8,10],
//   [13, 3, 6, 7],
//   [15,14,12,16]
// ], 

// rotate the input matrix in-place such that it becomes:
// [
//   [15,13, 2, 5],
//   [14, 3, 4, 1],
//   [12, 6, 8, 9],
//   [16, 7,10,11]
// ]



//  even:
//
//  x x o o
//  x x o o
//  o o o o
//  o o o o
//
//  odd:
//
//  x x o o o
//  x x o o o
//  x x o o o
//  o o o o o
//  o o o o o
// i,j ->  j,n-i 
//   |      |
// n-j,i <- n-i,n-j

pub struct Solution {}


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {

        let (len, n) = (matrix.len(), matrix.len()-1);

        for i in 0..len/2 {
            for j in 0..(len+1)/2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n-j][i];
                matrix[n-j][i] = matrix[n-i][n-j];
                matrix[n-i][n-j] = matrix[j][n-i];
                matrix[j][n-i] = temp;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_test() {
        let mut test_matrix_1 = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];
        Solution::rotate(&mut test_matrix_1);
        assert_eq!(
            test_matrix_1,
            vec![
                vec![7,4,1],
                vec![8,5,2],
                vec![9,6,3]
            ]
        );

        let mut test_matrix_2 = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16]
        ];
        Solution::rotate(&mut test_matrix_2);
        assert_eq!(
            test_matrix_2,
            vec![
                vec![15,13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7,10,11]
            ]
        );
    }
}
