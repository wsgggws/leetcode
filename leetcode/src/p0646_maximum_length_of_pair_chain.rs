// 646. Maximum Length of Pair Chain
// Medium

// You are given n pairs of numbers. In every pair, the first number is always smaller than the second number.

// Now, we define a pair (c, d) can follow another pair (a, b) if and only if b < c. Chain of pairs can be formed in this fashion.

// Given a set of pairs, find the length longest chain which can be formed. You needn't use up all the given pairs. You can select pairs in any order.

// Example 1:
// Input: [[1,2], [2,3], [3,4]]
// Output: 2
// Explanation: The longest chain is [1,2] -> [3,4]
// Note:
// The number of given pairs will be in the range [1, 1000].

pub struct Solution {}

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let n = pairs.len();
        if n == 0 {
            return 0;
        }
        let mut pairs = pairs;
        pairs.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut dp: Vec<i32> = vec![1; n];
        for i in 1..n {
            for j in 0..i {
                if pairs[j][1] < pairs[i][0] {
                    dp[i] = i32::max(dp[i], dp[j] + 1);
                }
            }
        }
        let mut result = 0;
        for i in 0..n {
            result = i32::max(result, dp[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest_chain_test() {
        assert_eq!(Solution::find_longest_chain(vec![]), 0);
        assert_eq!(Solution::find_longest_chain(vec![vec![1, 2]]), 1);
        assert_eq!(
            Solution::find_longest_chain(vec![vec![2, 3], vec![1, 2], vec![3, 4]]),
            2
        );
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }
}
