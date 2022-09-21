# 剑指 Offer 49. 丑数
# 我们把只包含质因子 2、3 和 5 的数称作丑数（Ugly Number）。求按从小到大的顺序的第 n 个丑数。


# 示例:

# 输入: n = 10
# 输出: 12
# 解释: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 是前 10 个丑数。
# 说明:

# 1 是丑数。
# n 不超过1690。


class Solution:
    def nthUglyNumber(self, n: int) -> int:
        """
        (2, 3, 5) * 丑数 = 丑数
        1
            2 * 1
        min 3 * 1  =>  2
            5 * 1
        1 2
            2 * 2
        min 3 * 1  => 3
            5 * 1

        uglys[index(ugly_2)] * 2
        uglys[index(ugly_3)] * 3  min => 下一个丑数, min (ugly_2, ugly_3, ugly_5) + 1
        uglys[index(ugly_5)] * 5
        """
        uglys = [1]
        ugly_2, ugly_3, ugly_5 = 0, 0, 0
        for _ in range(n):
            a = 2 * uglys[ugly_2]
            b = 3 * uglys[ugly_3]
            c = 5 * uglys[ugly_5]
            mins = min([a, b, c])
            if a == mins:
                ugly_2 += 1
            if b == mins:
                ugly_3 += 1
            if c == mins:
                ugly_5 += 1
            uglys.append(mins)
        return uglys[n - 1]


if __name__ == "__main__":
    s = Solution()
    assert s.nthUglyNumber(1) == 1
    assert s.nthUglyNumber(10) == 12
    assert s.nthUglyNumber(11) == 15
    assert s.nthUglyNumber(1690) == 2123366400
