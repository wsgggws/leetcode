class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        dp = [[0 for _ in range(n)] for _ in range(m)]
        for raw in range(m):
            for column in range(n):
                if raw == 0 or column == 0:
                    dp[raw][column] = 1
                else:
                    dp[raw][column] = dp[raw - 1][column] + dp[raw][column - 1]
        return dp[m - 1][n - 1]


if __name__ == "__main__":
    assert Solution().uniquePaths(3, 1) == 1
    assert Solution().uniquePaths(1, 3) == 1
    assert Solution().uniquePaths(3, 2) == 3
    assert Solution().uniquePaths(3, 7) == 28
