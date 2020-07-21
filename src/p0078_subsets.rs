// 78. Subsets
// Medium

// Given a set of distinct integers, nums, return all possible subsets (the power set).

// Note: The solution set must not contain duplicate subsets.

// Example:

// Input: nums = [1,2,3]
// Output:
// [
//   [3],
//   [1],
//   [2],
//   [1,2,3],
//   [1,3],
//   [2,3],
//   [1,2],
//   []
// ]

pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut visit: Vec<bool> = vec![false; nums.len()];
        let mut cur_vex: Vec<i32> = vec![];
        Solution::dfs(&mut ans, &mut visit, &nums, &mut cur_vex);
        ans
    }

    fn dfs(
        ans: &mut Vec<Vec<i32>>,
        visit: &mut Vec<bool>,
        nums: &Vec<i32>,
        cur_vex: &mut Vec<i32>,
    ) {
        ans.push(cur_vex.clone());
        for i in 0..nums.len() {
            if !visit[i as usize] {
                if cur_vex.len() > 0 && nums[i] < cur_vex[cur_vex.len() - 1] {
                    continue;
                }
                visit[i as usize] = true;
                cur_vex.push(nums[i]);
                Solution::dfs(ans, visit, nums, &mut cur_vex.clone());
                cur_vex.pop();
                visit[i as usize] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_test() {
        let empty: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(Solution::subsets(vec![]), empty);

        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        );
        assert_eq!(
            Solution::subsets(vec![4, 1, 0]),
            vec![
                vec![],
                vec![4],
                vec![1],
                vec![1, 4],
                vec![0],
                vec![0, 4],
                vec![0, 1],
                vec![0, 1, 4]
            ]
        );
    }
}
