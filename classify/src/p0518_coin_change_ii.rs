// 518. Coin Change 2
// Medium

// You are given coins of different denominations and a total amount of money. Write a function to compute the number of combinations that make up that amount. You may assume that you have infinite number of each kind of coin.

 

// Example 1:

// Input: amount = 5, coins = [1, 2, 5]
// Output: 4
// Explanation: there are four ways to make up the amount:
// 5=5
// 5=2+2+1
// 5=2+1+1+1
// 5=1+1+1+1+1
// Example 2:

// Input: amount = 3, coins = [2]
// Output: 0
// Explanation: the amount of 3 cannot be made up just with coins of 2.
// Example 3:

// Input: amount = 10, coins = [10] 
// Output: 1
 

// Note:

// You can assume that

// 0 <= amount <= 5000
// 1 <= coin <= 5000
// the number of coins is less than 500
// the answer is guaranteed to fit into signed 32-bit integer

pub struct Solution {}

impl Solution {
    // 完全背包
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; amount as usize + 1];
        dp[0] = 1;
        for &cost in coins.iter() {
            for v in 1..=amount as usize {
                if v >= cost as usize {
                    dp[v] += dp[v-cost as usize];
                }
            }
        } 
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_test() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change(3, vec![2]), 0);
        assert_eq!(Solution::change(10, vec![10]), 1);
    }
}
