# 剑指 Offer 10- I. 斐波那契数列
# 写一个函数，输入 n ，求斐波那契（Fibonacci）数列的第 n 项（即 F(N)）。斐波那契数列的定义如下：

# F(0) = 0,   F(1) = 1
# F(N) = F(N - 1) + F(N - 2), 其中 N > 1.
# 斐波那契数列由 0 和 1 开始，之后的斐波那契数就是由之前的两数相加而得出。

# 答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。


# 示例 1：

# 输入：n = 2
# 输出：1
# 示例 2：

# 输入：n = 5
# 输出：5


# 提示：

# 0 <= n <= 100


class Solution:
    NUMBER = 1000000007

    def fib(self, n: int) -> int:
        # 递归会重复计算
        # fib(99)             fib(98)
        # fib(98) + fib(97)   fib(97) + fib(96)
        # 97+96   +  97+96     96+95  + 95+94
        # ...
        # if 0 <= n <= 1:
        #     return n
        # return (self.fib(n-1) + self.fib(n-2)) % self.NUMBER

        # 使用 dp 数组缓存每个值
        # dp = [0, 1]
        # for _ in range(2, n+1):
        #     dp.append((dp[-1] + dp[-2]) % self.NUMBER)
        # return dp[n]

        # 只使用两个常量进行优化
        # f0, f1      # 初始值
        #     f0, f1
        # 哪上当进行一次迭代时，明显 f1 表示的是前两数之和 (f0+f1),
        # 而 f0 表示的是上一次迭代的 f1, 即可同时赋值完成一次迭代 f0, f1 = f1, f0 + f1
        # if 0 <= n <= 1:
        #     return n
        # f0, f1 = 0, 1
        # for _ in range(2, n + 1):
        #     f0, f1 = f1, (f0 + f1) % self.NUMBER
        # return f1

        # 或许用 3 个变量更容易理解；）
        if 0 <= n <= 1:
            return n
        f0, f1, f2 = 0, 1, 1
        for _ in range(2, n + 1):
            f2 = (f0 + f1) % self.NUMBER
            f0 = f1
            f1 = f2
        return f2


if __name__ == "__main__":
    s = Solution()
    assert s.fib(0) == 0
    assert s.fib(1) == 1
    assert s.fib(2) == 1
    assert s.fib(5) == 5
    assert s.fib(100) == 687995182
