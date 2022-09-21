# 剑指 Offer 16. 数值的整数次方
# 实现 pow(x, n) ，即计算 x 的 n 次幂函数（即，xn）。不得使用库函数，同时不需要考虑大数问题。


# 示例 1：

# 输入：x = 2.00000, n = 10
# 输出：1024.00000
# 示例 2：

# 输入：x = 2.10000, n = 3
# 输出：9.26100
# 示例 3：

# 输入：x = 2.00000, n = -2
# 输出：0.25000
# 解释：2-2 = 1/22 = 1/4 = 0.25


# 提示：

# -100.0 < x < 100.0
# -231 <= n <= 231-1
# -104 <= xn <= 104


class Solution:
    # def myPow(self, x: float, n: int) -> float:
    #     """递归, (暴力因为 n<= 2**31-1,所以忽略)"""
    #     if n == 0:
    #         return 1
    #     if n < 0:
    #         x = 1.0 / x
    #         n = -n
    #     if n % 2 == 0:
    #         return self.myPow(x * x, n // 2)
    #     else:
    #         return x * self.myPow(x * x, n // 2)

    def myPow(self, x: float, n: int) -> float:
        """迭代
        2**10
        (2**2)*5
        (2**2) * ((2**2) * (2**2)) ** 2
        [(2**2) * ((2**2)**2)] * [(2**2) * ((2**2)**2)]
        """
        if n < 0:
            x = 1.0 / x
            n = -n
        ans = 1
        radix = x
        while n:
            if n % 2 == 1:
                ans *= radix
            n //= 2
            radix *= radix
        return ans


if __name__ == "__main__":
    s = Solution()
    assert abs(s.myPow(2.00000, 1) - 2.00000) < 10e-5
    assert abs(s.myPow(2.00000, 10) - 1024.0) < 10e-5
    assert abs(s.myPow(2.10000, 3) - 9.261) < 10e-5
    assert abs(s.myPow(2.00000, -2) - 0.25000) < 10e-5
    assert abs(s.myPow(0.00000, 0) - 1.00000) < 10e-5
    assert abs(s.myPow(0.00000, 1) - 0) < 10e-5
    # ?
    # assert abs(s.myPow(0.00000, -1) - 0) < 10e-5
