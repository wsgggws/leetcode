class Solution:
    def climbStairs(self, n: int) -> int:
        f1, f2 = 1, 1
        for _ in range(n):
            f1, f2 = f2, f1 + f2
        return f1


if __name__ == "__main__":
    assert Solution().climbStairs(0) == 1
    assert Solution().climbStairs(1) == 1
    assert Solution().climbStairs(2) == 2
    assert Solution().climbStairs(3) == 3
    assert Solution().climbStairs(4) == 5
