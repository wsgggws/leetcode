// 118. Pascal's Triangle
// Easy

// Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.

// In Pascal's triangle, each number is the sum of the two numbers directly above it.

// Example:

// Input: 5
// Output:
// [
//      [1],
//     [1,1],
//    [1,2,1],
//   [1,3,3,1],
//  [1,4,6,4,1]
// ]

pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }
        if num_rows == 1 {
            return vec![vec![1]];
        }
        if num_rows == 2 {
            return vec![vec![1], vec![1, 1]];
        }
        let mut result = Vec::new();
        result.push(vec![1]);
        result.push(vec![1, 1]);
        for row in 2..num_rows {
            let mut temp = Vec::new();
            temp.push(1);
            for index in 1..row {
                temp.push(
                    result[(row - 1) as usize][(index - 1) as usize]
                        + result[(row - 1) as usize][index as usize],
                );
            }
            temp.push(1);
            result.push(temp);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_test() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);

        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
