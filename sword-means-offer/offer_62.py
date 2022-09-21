class Solution:
    def lastRemaining(self, n: int, m: int) -> int:
        if n == 1:
            return 0
        x = self.lastRemaining(n - 1, m)
        return (x + m) % n

    def lastRemaining(self, n: int, m: int) -> int:

        f = [0 for _ in range(n + 1)]
        f[1] = 0
        for i in range(2, n + 1):
            f[i] = (f[i - 1] + m) % i
        return f[n]


if __name__ == "__main__":
    s = Solution()
    assert s.lastRemaining(5, 3) == 3
    assert s.lastRemaining(10, 17) == 2
