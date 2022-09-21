# 剑指 Offer 10- II. 青蛙跳台阶问题
# 一只青蛙一次可以跳上1级台阶，也可以跳上2级台阶。求该青蛙跳上一个 n 级的台阶总共有多少种跳法。

# 答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。

# 示例 1：

# 输入：n = 2
# 输出：2
# 示例 2：

# 输入：n = 7
# 输出：21
# 示例 3：

# 输入：n = 0
# 输出：1
# 提示：

# 0 <= n <= 100

from typing import Dict


class Solution:
    MOD_NUM = 1000000007
    mem: Dict[int, int] = {}

    # def numWays(self, n: int) -> int:
    #     # 我知道 N-1 有多少种跳法之后，每种跳法再跳 1 步能够达到 N
    #     # 我知道 N-2 有多少种跳法之后，每种跳法再跳 2 步也能够达到 N
    #     # so, F[N] = F[N-1] + F[N-2]
    #     if n <= 1:
    #         return 1
    #     return (self.numWays(n - 1) + self.numWays(n - 2)) % self.MOD_NUM
    # def numWays(self, n: int) -> int:
    #     if n <= 1:
    #         return 1
    #     if self.mem.get(n):
    #         return self.mem[n]
    #     else:
    #         self.mem[n] = (self.numWays(n - 1) + self.numWays(n - 2)) % self.MOD_NUM
    #     return self.mem[n]

    # def numWays(self, n: int) -> int:
    #     dp = [1, 1]
    #     for _ in range(2, n+1):
    #         dp.append((dp[-1] + dp[-2]) % self.MOD_NUM)
    #     return dp[n]
    # def numWays(self, n: int) -> int:
    #     f0, f1 = 1, 1
    #     f2 = 1
    #     for _ in range(2, n+1):
    #         f2 = (f0 + f1) % self.MOD_NUM
    #         f0 = f1
    #         f1 = f2
    #     return f2
    def numWays(self, n: int) -> int:
        f0, f1 = 1, 1
        # f0, f1
        #     (f1) => f0 , (f0+f1)=>f1
        # 这个是利用了 Python 行内赋值的特性
        for _ in range(2, n + 1):
            f0, f1 = f1, (f0 + f1) % self.MOD_NUM
        return f1

    # TODO: 矩阵相乘 O(lgN), 公式O(1)


if __name__ == "__main__":
    s = Solution()
    assert s.numWays(0) == 1
    assert s.numWays(2) == 2
    assert s.numWays(7) == 21
    assert s.numWays(100) == 782204094
