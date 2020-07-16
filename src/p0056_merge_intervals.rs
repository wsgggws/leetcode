// 56. Merge Intervals
// Medium

// Given a collection of intervals, merge all overlapping intervals.

// Example 1:

// Input: [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
// Example 2:

// Input: [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
// NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.

pub struct Solution {}

use std::cmp::Ordering;
impl Solution {
    fn cmp(a: &Vec<i32>, b: &Vec<i32>) -> Ordering {
        if a[0] < b[0] {
            return Ordering::Less;
        } else if a[0] == b[0] && a[1] < b[1] {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals.clone();
        intervals.sort_by(|a, b| Solution::cmp(a, b));
        let mut result: Vec<Vec<i32>> = vec![];
        let mut i = 0;
        while i < intervals.len() {
            let left = intervals[i][0];
            let mut right = intervals[i][1];
            let mut j = i + 1;
            while j < intervals.len() && intervals[j][0] >= left && intervals[j][0] <= right {
                right = i32::max(right, intervals[j][1]);
                j += 1;
            }
            result.push(vec![left, right]);
            i = j;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_test() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![5, 6]]),
            vec![vec![1, 4], vec![5, 6]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 3]]),
            vec![vec![1, 4]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![2, 3],
                vec![4, 5],
                vec![6, 7],
                vec![8, 9],
                vec![1, 10]
            ]),
            vec![vec![1, 10]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 2], vec![3, 5]]),
            vec![vec![0, 5]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![5, 5],
                vec![1, 3],
                vec![3, 5],
                vec![4, 6],
                vec![1, 1],
                vec![3, 3],
                vec![5, 6],
                vec![3, 3],
                vec![2, 4],
                vec![0, 0]
            ]),
            vec![vec![0, 0], vec![1, 6]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![1, 3],
                vec![0, 2],
                vec![2, 3],
                vec![4, 6],
                vec![4, 5],
                vec![5, 5],
                vec![0, 2],
                vec![3, 3]
            ]),
            vec![vec![0, 3], vec![4, 6]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![0, 0],
                vec![1, 2],
                vec![5, 5],
                vec![2, 4],
                vec![3, 3],
                vec![5, 6],
                vec![5, 6],
                vec![4, 6],
                vec![0, 0],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5]
            ]),
            vec![vec![0, 6]]
        );
    }
}
