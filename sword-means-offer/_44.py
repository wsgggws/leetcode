# 剑指 Offer 44. 数字序列中某一位的数字
# 数字以0123456789101112131415…的格式序列化到一个字符序列中。
# 在这个序列中，第5位（从下标0开始计数）是5，第13位是1，第19位是4，等等。

# 请写一个函数，求任意第n位对应的数字。

# 示例 1：

# 输入：n = 3
# 输出：3
# 示例 2：

# 输入：n = 11
# 输出：0

# 限制：

# 0 <= n < 2^31


class Solution:
    def findNthDigit(self, n: int) -> int:
        """
        0 ignore
        (9 - 1 + 1) = 9 * 1 = 9
        (99 - 10 + 1) = 90 * 2 = 180
        (999 - 100 + 1) = 900 * 3 = 2700
        (9999 - 1000 + 1) = 9000 * 4 = 36000
        bitx: x 位所在这个区间的数字的所有位和，比如x=2, 180 位
        while n - bitx > 0
            n -= bitx
            bitx +=1
        bitx: x 要求的数字所在的区间
        n // bitx: 这个 x 区间的第多少位数字
        number = 10 ** (bitx - 1) + n // bitx - 1
        number[n % bitx]
        """
        bit = 1
        while n - 9 * 10 ** (bit - 1) * bit > 0:
            n -= 9 * 10 ** (bit - 1) * bit
            bit += 1
        # n = 6, bit = 3
        if n % bit == 0:
            number = 10 ** (bit - 1) + n // bit - 1
            return number % 10
        # n = 5, bit = 3
        else:
            number = 10 ** (bit - 1) + n // bit - 1 + 1
            return number // 10 ** (bit - n % bit) % 10


if __name__ == "__main__":
    s = Solution()
    assert s.findNthDigit(0) == 0
    assert s.findNthDigit(3) == 3
    assert s.findNthDigit(11) == 0
    assert s.findNthDigit(19) == 4
    assert s.findNthDigit(189) == 9
    assert s.findNthDigit(188) == 9
    assert s.findNthDigit(187) == 8
    assert s.findNthDigit(190) == 1
    assert s.findNthDigit(191) == 0
    assert s.findNthDigit(192) == 0
