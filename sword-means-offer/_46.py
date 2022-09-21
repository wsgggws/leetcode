# 给定一个数字，我们按照如下规则把它翻译为字符串：0 翻译成 “a” ，1 翻译成 “b”，……，11 翻译成 “l”，……，25 翻译成 “z”。
# 一个数字可能有多个翻译。请编程实现一个函数，用来计算一个数字有多少种不同的翻译方法。

# 示例 1:

# 输入: 12258
# 输出: 5
# 解释: 12258有5种不同的翻译，分别是"bccfi", "bwfi", "bczi", "mcfi"和"mzi"
#

# 提示：

# 0 <= num < 231


class Solution:
    def translateNum(self, num: int) -> int:
        """
        12 + (25) =>
            1 2 (25)
            12 (25)
        122 + (5) =>
            1 2 2 (5)
            12 2  (5)
            1 22  (5)
        dp[n]: n 个数字所能翻译方法的个数
        =》 dp[n] = dp[n-1] + dp[n-2] # 最后面两个字符 <=25, >=10
                  = dp[n-1] # 否则
        """
        text = str(num)
        dp = [1 for _ in range(len(text) + 1)]
        dp[0] = 1
        dp[1] = 1
        for i in range(2, len(text) + 1):
            if 10 <= int(text[i - 2 : i]) <= 25:
                dp[i] = dp[i - 2] + dp[i - 1]
            else:
                dp[i] = dp[i - 1]
        return dp[len(text)]


if __name__ == "__main__":
    solution = Solution()
    assert solution.translateNum(12258) == 5
    assert solution.translateNum(1) == 1
    assert solution.translateNum(12) == 2
    assert solution.translateNum(102) == 2
