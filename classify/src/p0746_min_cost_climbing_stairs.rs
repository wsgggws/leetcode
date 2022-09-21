// 746. Min Cost Climbing Stairs
// Easy

// On a staircase, the i-th step has some non-negative cost cost[i] assigned (0 indexed).

// Once you pay the cost, you can either climb one or two steps. You need to find minimum cost to reach the top of the floor, and you can either start from the step with index 0, or the step with index 1.

// Example 1:
// Input: cost = [10, 15, 20]
// Output: 15
// Explanation: Cheapest is start on cost[1], pay that cost and go to the top.
// Example 2:
// Input: cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
// Output: 6
// Explanation: Cheapest is start on cost[0], and only step on 1s, skipping cost[3].
// Note:
// cost will have a length in the range [2, 1000].
// Every cost[i] will be an integer in the range [0, 999].

pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; cost.len()];
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..cost.len() {
            dp[i] = cost[i] + i32::min(dp[i-1], dp[i-2]);
        }
        i32::min(dp[cost.len()-2], dp[cost.len()-1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_cost_climbing_stairs_test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
    }
}
