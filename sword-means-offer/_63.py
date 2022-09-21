# 假设把某股票的价格按照时间先后顺序存储在数组中，
# 请问买卖该股票一次可能获得的最大利润是多少？
#

# 示例 1:

# 输入: [7,1,5,3,6,4]
# 输出: 5
# 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
#      注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格。
# 示例 2:

# 输入: [7,6,4,3,1]
# 输出: 0
# 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
#

# 限制：

# 0 <= 数组长度 <= 10^5


import sys
from typing import List


class Solution:
    # def maxProfit(self, prices: List[int]) -> int:
    #     # dp[i] 表示第i天卖出所获得的利润
    #     dp = [0 for _ in prices]
    #     for i in range(1, len(prices)):
    #         dp[i] = max(0, dp[i-1]) - prices[i-1] + prices[i]
    #     return max(dp, default=0)

    # def maxProfit(self, prices: List[int]) -> int:
    #     # 优化: current 表示第i天卖出所获得的利润
    #     current = 0
    #     maxs = 0
    #     for i in range(1, len(prices)):
    #         current = max(0, current) - prices[i-1] + prices[i]
    #         maxs = max(maxs, current)
    #     return maxs

    def maxProfit(self, prices: List[int]) -> int:
        mins = sys.maxsize
        maxs = 0
        for price in prices:
            mins = min(mins, price)
            maxs = max(maxs, price - mins)
        return maxs


if __name__ == "__main__":
    s = Solution()
    assert s.maxProfit([]) == 0
    assert s.maxProfit([7, 1, 5, 3, 6, 4]) == 5
    assert s.maxProfit([7, 6, 4, 3, 1]) == 0
