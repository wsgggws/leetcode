from typing import List


class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        steps = len(cost)
        dp = [0 for _ in range(steps + 1)]
        dp[0] = 0
        dp[1] = 0
        for i in range(2, steps + 1):
            dp[i] = min(dp[i - 2] + cost[i - 2], dp[i - 1] + cost[i - 1])
        return dp[steps]


if __name__ == "__main__":
    assert Solution().minCostClimbingStairs([10, 15, 20]) == 15
    assert Solution().minCostClimbingStairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1]) == 6
