from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        # dp[i]: 第 i 天卖出能够获得的最大利润
        # dp[i] = max(0, dp[i-1]) - prices[i-1] + prices[i]
        # dp[0] = 0
        dp = [0 for _ in range(len(prices))]
        for i in range(1, len(prices)):
            dp[i] = max(0, dp[i - 1]) - prices[i - 1] + prices[i]
        return max(dp, default=0)


if __name__ == "__main__":
    assert Solution().maxProfit([1]) == 0
    assert Solution().maxProfit([7, 1, 5, 3, 6, 4]) == 5
    assert Solution().maxProfit([7, 6, 4, 3, 1]) == 0
