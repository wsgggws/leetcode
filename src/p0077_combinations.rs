// 77. Combinations
// Medium

// Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.

// Example:

// Input: n = 4, k = 2
// Output:
// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]

pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k <= 0 || k > n {
            return vec![];
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut visit: Vec<bool> = vec![false; n as usize];
        let mut cur_vex: Vec<i32> = vec![];
        Solution::dfs(&mut ans, &mut visit, n, k, &mut cur_vex, 0);
        ans
    }

    fn dfs(
        ans: &mut Vec<Vec<i32>>,
        visit: &mut Vec<bool>,
        n: i32,
        k: i32,
        cur_vex: &mut Vec<i32>,
        cur_index: usize,
    ) {
        if cur_index == k as usize {
            ans.push(cur_vex.clone());
            return;
        }
        for i in 0..n {
            if !visit[i as usize] && cur_index + 1 <= k as usize {
                if cur_vex.len() > 0 && i + 1 < cur_vex[cur_vex.len() - 1] {
                    continue;
                }
                visit[i as usize] = true;
                cur_vex.push(i + 1);
                Solution::dfs(ans, visit, n, k, &mut cur_vex.clone(), cur_index + 1);
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
    fn combine_test() {
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(4, 5), empty);
        assert_eq!(Solution::combine(4, -2), empty);

        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }
}
