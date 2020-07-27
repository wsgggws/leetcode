// 90. Subsets II
// Medium

// Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).

// Note: The solution set must not contain duplicate subsets.

// Example:

// Input: [1,2,2]
// Output:
// [
//   [2],
//   [1],
//   [1,2,2],
//   [2,2],
//   [1,2],
//   []
// ]

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut visit: Vec<bool> = vec![false; nums.len()];
        let mut cur_vex: Vec<i32> = vec![];
        let nums_counts: Vec<(i32, usize)> = Solution::get_nums_counts(&nums);
        Solution::dfs(&mut ans, &mut visit, &nums_counts, &mut cur_vex);
        ans
    }

    fn get_nums_counts(nums: &Vec<i32>) -> Vec<(i32, usize)> {
        let mut hash_map: HashMap<i32, usize> = HashMap::new();
        for &num in nums.iter() {
            let counter = hash_map.entry(num).or_insert(0);
            *counter += 1;
        }
        hash_map
            .iter()
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<(i32, usize)>>()
    }

    fn dfs(
        ans: &mut Vec<Vec<i32>>,
        visit: &mut Vec<bool>,
        nums_counts: &Vec<(i32, usize)>,
        cur_vex: &mut Vec<i32>,
    ) {
        ans.push(cur_vex.clone());
        for i in 0..nums_counts.len() {
            // 保证后面进来的元素大于等于前面的元素
            if cur_vex.len() > 0 && nums_counts[i].0 < cur_vex[cur_vex.len() - 1] {
                continue;
            }
            if !visit[i] {
                visit[i] = true;
                // 每个元素可以先1~count个
                for count in 1..=nums_counts[i].1 {
                    for _ in 0..count {
                        cur_vex.push(nums_counts[i].0);
                    }
                    Solution::dfs(ans, visit, nums_counts, &mut cur_vex.clone());
                    for _ in 0..count {
                        cur_vex.pop();
                    }
                }
                visit[i] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_with_dup_test() {
        let empty: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(Solution::subsets_with_dup(vec![]), empty);

        let mut result = Solution::subsets_with_dup(vec![1, 2, 2]);
        result.sort();
        let mut expect_result = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ];
        expect_result.sort();
        assert_eq!(result, expect_result);

        let mut result = Solution::subsets_with_dup(vec![4, 1, 1]);
        result.sort();
        let mut expect_result = vec![
            vec![],
            vec![4],
            vec![1],
            vec![1, 4],
            vec![1, 1],
            vec![1, 1, 4]
        ];
        expect_result.sort();
        assert_eq!(result, expect_result);
    }
}
