class Solution:
    def fib(self, n: int) -> int:
        f1, f2 = 0, 1
        for _ in range(n):
            f1, f2 = f2, f1 + f2
        return f1


if __name__ == "__main__":
    assert Solution().fib(0) == 0
    assert Solution().fib(1) == 1
    assert Solution().fib(2) == 1
    assert Solution().fib(3) == 2
    assert Solution().fib(4) == 3
