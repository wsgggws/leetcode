// 39. Combination Sum
// Medium

// Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.

// The same repeated number may be chosen from candidates unlimited number of times.

// Note:

// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.
// Example 1:

// Input: candidates = [2,3,6,7], target = 7,
// A solution set is:
// [
//   [7],
//   [2,2,3]
// ]
// Example 2:

// Input: candidates = [2,3,5], target = 8,
// A solution set is:
// [
//   [2,2,2,2],
//   [2,3,3],
//   [3,5]
// ]


// Constraints:

// 1 <= candidates.length <= 30
// 1 <= candidates[i] <= 200
// Each element of candidate is unique.
// 1 <= target <= 500

pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut cur_vec: Vec<i32> = vec![];
        Solution::dfs(&mut ans, &candidates, &mut cur_vec, 0, target);
        ans
    }

    fn dfs(
        ans: &mut Vec<Vec<i32>>,
        candidates: &Vec<i32>,
        cur_vec: &mut Vec<i32>,
        cur_target: i32,
        target: i32,
    ) {
        if cur_target == target {
            ans.push(cur_vec.clone());
            return;
        }
        for i in 0..candidates.len() {
            if cur_target + candidates[i] <= target {
                if cur_vec.len() > 0 && candidates[i] < cur_vec[cur_vec.len() - 1] {
                    continue;
                }
                cur_vec.push(candidates[i]);
                Solution::dfs(
                    ans,
                    candidates,
                    &mut cur_vec.clone(),
                    cur_target + candidates[i],
                    target,
                );
                cur_vec.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination_sum_test() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );

        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(
            Solution::combination_sum(vec![5, 3, 2], 8),
            vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]]
        );
    }
}
