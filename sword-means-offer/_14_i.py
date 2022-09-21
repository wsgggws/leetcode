# 剑指 Offer 14- II. 剪绳子 II
# 给你一根长度为 n 的绳子，请把绳子剪成整数长度的 m 段（m、n都是整数，n>1并且m>1），
# 每段绳子的长度记为 k[0],k[1]...k[m - 1] 。请问 k[0]*k[1]*...*k[m - 1] 可能的最大乘积是多少？例如，当绳子的长度是8时，我们把它剪成长度分别为2、3、3的三段，此时得到的最大乘积是18。

# 答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。


# 示例 1：

# 输入: 2
# 输出: 1
# 解释: 2 = 1 + 1, 1 × 1 = 1
# 示例 2:

# 输入: 10
# 输出: 36
# 解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36


# 提示：

# 2 <= n <= 1000


class Solution:
    # def cuttingRope(self, n: int) -> int:
    #     """
    #     递归    ans = max(self._recursive_cutting(n - i) * i, ans)
    #     DP      dp[n] = max(dp[n-i] * i, dp[n])
    #     Greedy  ans *= 3, n -= 3, return ans * n
    #     """
    #     if n <= 3:
    #         return 1 * (n - 1)
    #     return self._recursive_cutting(n)

    # def _recursive_cutting(self, n: int) -> int:
    #     if n <= 3:
    #         return n
    #     ans = 0
    #     for i in range(2, n // 2 + 1):
    #         ans = max(ans, self._recursive_cutting(n - i) * i)
    #     return ans

    # def cuttingRope(self, n: int) -> int:
    #     """
    #     DP      dp[n] = max(dp[n-i] * i, dp[n])
    #     Greedy  ans *= 3, n -= 3, return ans * n
    #     """
    #     if n <= 3:
    #         return 1 * (n - 1)
    #     dp = [0 for _ in range(n + 1)]
    #     dp[0] = 0
    #     dp[1] = 1
    #     dp[2] = 2
    #     dp[3] = 3
    #     for i in range(4, n + 1):
    #         max_value = 0
    #         for j in range(2, i // 2 + 1):
    #             max_value = max(dp[i - j] * j, max_value)
    #         dp[i] = max_value
    #     # note should to mod at last only
    #     return dp[n] % 1000000007

    def cuttingRope(self, n: int) -> int:
        """
        DP      dp[n] = max(dp[n-i] * i, dp[n])
        Greedy  ans *= 3, n -= 3, return ans * n
        """
        if n <= 3:
            return 1 * (n - 1)
        ans = 1
        while n > 4:
            ans = (ans * 3) % 1000000007
            n -= 3
        return ans * n % 1000000007


if __name__ == "__main__":
    s = Solution()
    assert s.cuttingRope(2) == 1
    assert s.cuttingRope(3) == 2
    assert s.cuttingRope(4) == 4
    assert s.cuttingRope(5) == 6
    assert s.cuttingRope(6) == 9
    assert s.cuttingRope(7) == 12
    assert s.cuttingRope(8) == 18
    assert s.cuttingRope(9) == 27
    assert s.cuttingRope(10) == 36
