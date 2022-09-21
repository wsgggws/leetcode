# 给定一个字符串数组 words，请计算当两个字符串 words[i] 和 words[j] 不包含相同字符时，
# 它们长度的乘积的最大值。假设字符串中只包含英语的小写字母。如果没有不包含相同字符的一对字符串，返回 0。

#

# 示例 1:

# 输入: words = ["abcw","baz","foo","bar","fxyz","abcdef"]
# 输出: 16
# 解释: 这两个单词为 "abcw", "fxyz"。它们不包含相同字符，且长度的乘积最大。
# 示例 2:

# 输入: words = ["a","ab","abc","d","cd","bcd","abcd"]
# 输出: 4
# 解释: 这两个单词为 "ab", "cd"。
# 示例 3:

# 输入: words = ["a","aa","aaa","aaaa"]
# 输出: 0
# 解释: 不存在这样的两个单词。
#

# 提示：

# 2 <= words.length <= 1000
# 1 <= words[i].length <= 1000
# words[i] 仅包含小写字母


from typing import List


class Solution:

    # def maxProduct(self, words: List[str]) -> int:
    #     mul_max = 0
    #     for i in range(len(words)):
    #         for j in range(i + 1, len(words)):
    #             mul_tmp = len(words[i]) * len(words[j])
    #             if mul_tmp > mul_max and not (set(words[i]) & set(words[j])):
    #                 mul_max = mul_tmp
    #     return mul_max

    def maxProduct(self, words: List[str]) -> int:
        """ """
        # 利用二进制 words[i] => nums[i], bc => 0x0110, a => 0x01
        # nums[i]? 每一位与 0 进行或运算， 如上面 bc
        nums = [0 for _ in words]
        for i in range(len(words)):
            for ch in words[i]:
                nums[i] |= 1 << (ord(ch) - ord("a"))
        mul_max = 0
        for i in range(len(words)):
            for j in range(i + 1, len(words)):
                mul_tmp = len(words[i]) * len(words[j])
                if mul_tmp > mul_max and nums[i] & nums[j] == 0:
                    mul_max = mul_tmp
        return mul_max


if __name__ == "__main__":
    s = Solution()
    assert s.maxProduct(["abcw", "baz", "foo", "bar", "fxyz", "abcdef"]) == 16
    assert s.maxProduct(["a", "ab", "abc", "d", "cd", "bcd", "abcd"]) == 4
    assert s.maxProduct(["a", "aa", "aaa", "aaaa"]) == 0
