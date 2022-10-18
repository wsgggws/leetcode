from typing import List


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        """
        dp[i]: 表示换成i金额所需要的最少硬币个数，则
        dp[i] = min(dp[i-coin] + 1 for coin in coins)
        """
        dp = [0 for _ in range(amount + 1)]
        dp[0] = 0
        for i in range(1, amount + 1):
            mins = amount + 1
            for coin in coins:
                if i >= coin:
                    if dp[i - coin] + 1 < mins:
                        mins = dp[i - coin] + 1
            dp[i] = mins
        if dp[amount] == amount + 1:
            return -1
        return dp[amount]


if __name__ == "__main__":
    assert Solution().coinChange(coins=[1, 2, 5], amount=11) == 3
    assert Solution().coinChange(coins=[2], amount=3) == -1
    assert Solution().coinChange(coins=[1], amount=0) == 0
    assert Solution().coinChange(coins=[2, 5, 10, 1], amount=27) == 4
    assert Solution().coinChange(coins=[2], amount=1) == -1
