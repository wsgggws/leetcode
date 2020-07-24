// 121. Best Time to Buy and Sell Stock
// Easy

// Say you have an array for which the ith element is the price of a given stock on day i.

// If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.

// Note that you cannot sell a stock before you buy one.

// Example 1:

// Input: [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//              Not 7-1 = 6, as selling price needs to be larger than buying price.
// Example 2:

// Input: [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transaction is done, i.e. max profit = 0.

pub struct Solution {}

impl Solution {

    // dp[i] 表示第i天卖出的收益, 则考虑第i天卖或者不卖
    pub fn max_profit_dp(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0; prices.len()];
        dp[0] = 0;
        for i in 1..prices.len() {
            // 第i天卖则需要把第i-1天卖的收获减去，不卖则为负数，直接取0，跟dp[0]取0是类似的
            dp[i] = i32::max(dp[i-1] + prices[i] - prices[i-1], 0);
        }
        *dp.iter().max().unwrap_or(&0)
    }

    // 上面dp算法的空间优化版本, 在遍历时动态更新最大收获
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut result = 0;
        for i in 1..prices.len() {
            current = current + prices[i] - prices[i - 1];
            if current <= 0 {
                current = 0;
            } else {
                result = i32::max(result, current);
            }
        }
        result
    }

    // 贪心解法
    pub fn max_profit_greedy(prices: Vec<i32>) -> i32 {
        let mut buy_price = prices[0];
        let mut profit = 0;
        for &price in prices.iter() {
            buy_price = i32::min(buy_price, price);
            profit = i32::max(profit, price - buy_price);
        }
        profit
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn max_profit_greedy_test() {
        assert_eq!(Solution::max_profit_greedy(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit_greedy(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn max_profit_dp_test() {
        assert_eq!(Solution::max_profit_dp(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit_dp(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
