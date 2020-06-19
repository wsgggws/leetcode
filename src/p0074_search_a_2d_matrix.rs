// 74. Search a 2D Matrix
// Medium

// Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:

// Integers in each row are sorted from left to right.
// The first integer of each row is greater than the last integer of the previous row.
// Example 1:

// Input:
// matrix = [
//   [1,   3,  5,  7],
//   [10, 11, 16, 20],
//   [23, 30, 34, 50]
// ]
// target = 3
// Output: true
// Example 2:

// Input:
// matrix = [
//   [1,   3,  5,  7],
//   [10, 11, 16, 20],
//   [23, 30, 34, 50]
// ]
// target = 13
// Output: false

pub struct Solution {}

impl Solution {
    // 直接使用二分查找模板二
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let nums = matrix.into_iter().flatten().collect::<Vec<i32>>();
        let (mut start, mut end) = (0usize, nums.len());
        while start < end {
            let mid = start + (end - start) / 2;
            if nums[mid] >= target {
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        if start < nums.len() && nums[start] == target {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_matrix_test() {
        assert_eq!(
            Solution::search_matrix(vec![
                vec![1,   3,  5,  7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50]
            ], 3), true);

        assert_eq!(
            Solution::search_matrix(vec![
                vec![1,   3,  5,  7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50]
            ], 13), false);
    }
}
