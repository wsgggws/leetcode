// 47. Permutations II
// Medium

// Given a collection of numbers that might contain duplicates, return all possible unique permutations.

// Example:

// Input: [1,1,2]
// Output:
// [
//   [1,1,2],
//   [1,2,1],
//   [2,1,1]
// ]

pub struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut visit: Vec<bool> = vec![false; nums.len()];
        let mut cur_vec: Vec<i32> = vec![];
        let mut nums = nums.clone();
        nums.sort();
        Solution::dfs(&mut ans, &mut visit, &nums, &mut cur_vec, 0);
        ans
    }
    fn dfs(
        ans: &mut Vec<Vec<i32>>,
        visit: &mut Vec<bool>,
        nums: &Vec<i32>,
        cur_vec: &mut Vec<i32>,
        cur_index: usize,
    ) {
        if cur_index == nums.len() {
            ans.push(cur_vec.clone());
            return;
        }
        for i in 0..nums.len() {
            if !visit[i] {
                if i > 0 && nums[i] == nums[i - 1] && !visit[i - 1] {
                    continue;
                }
                visit[i] = true;
                cur_vec.push(nums[i]);
                Solution::dfs(ans, visit, nums, &mut cur_vec.clone(), cur_index + 1);
                cur_vec.pop();
                visit[i] = false;
            }
        }
    }

    pub fn permute_unique_by_swap(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut visit: Vec<bool> = vec![false; nums.len()];
        let mut array = nums.clone();
        array.sort();
        Solution::dfs_by_swap(&mut ans, &mut array, &mut visit, 0);
        ans
    }
    fn dfs_by_swap(
        ans: &mut Vec<Vec<i32>>,
        array: &mut Vec<i32>,
        visit: &mut Vec<bool>,
        cur_index: usize,
    ) {
        if cur_index == array.len() {
            ans.push(array.clone());
            return;
        }
        for i in cur_index..array.len() {
            if i > 0 && array[i] == array[i - 1] && !visit[i - 1] {
                continue;
            }
            visit[i] = true;
            array.swap(cur_index, i);
            Solution::dfs_by_swap(ans, array, visit, cur_index + 1);
            array.swap(cur_index, i);
            visit[i] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_unique_test() {
        assert_eq!(Solution::permute_unique(vec![1]), vec![vec![1],]);
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn permute_unique_by_swap_test() {
        assert_eq!(Solution::permute_unique_by_swap(vec![1]), vec![vec![1],]);
        assert_eq!(
            Solution::permute_unique_by_swap(vec![1, 2, 1]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]
        );
        assert_eq!(
            Solution::permute_unique_by_swap(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2]
            ]
        );
    }
}
