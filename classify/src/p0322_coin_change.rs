// 322. Coin Change
// Medium

// You are given coins of different denominations and a total amount of money amount. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

// Example 1:

// Input: coins = [1, 2, 5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
// Example 2:

// Input: coins = [2], amount = 3
// Output: -1
// Note:
// You may assume that you have an infinite number of each kind of coin.

pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // dp[i] 表示组成金额i所需要的最小金币数
        // => dp[i] = min(dp[i], dp[i-coin[j]] + 1) | i-coin[j] >= 0
        let mut dp: Vec<i32> = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 0i32..=amount {
            for j in 0..coins.len() {
                if i - coins[j] >= 0 {
                    dp[i as usize] = i32::min(dp[i as usize], dp[(i-coins[j]) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_change_test() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![2], 1), -1);
        assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
    }
}
