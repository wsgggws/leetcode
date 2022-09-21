# 剑指 Offer 38. 字符串的排列
# 输入一个字符串，打印出该字符串中字符的所有排列。


# 你可以以任意顺序返回这个字符串数组，但里面不能有重复元素。


# 示例:

# 输入：s = "abc"
# 输出：["abc","acb","bac","bca","cab","cba"]


# 限制：

# 1 <= s 的长度 <= 8


from typing import List


class Solution:
    def permutation(self, s: str) -> List[str]:
        """递归
        n  -> n-1, n-1排列的每一个字符串的每一个位置插入该元素
        """
        if len(s) <= 1:
            return [s]
        eles = self.permutation(s[:-1])
        result = set()
        for ele in eles:
            for i in range(len(ele) + 1):
                result.add(ele[:i] + s[-1] + ele[i:])
        return list(result)

    # def _get_elements(self, ch: str, eles: Set[str]) -> Set[str]:
    #     result: Set[str] = set()
    #     for ele in eles:
    #         for i in range(len(ele) + 1):
    #             result.add(ele[:i] + ch + ele[i:])
    #     return result

    # def permutation(self, s: str) -> List[str]:
    #     """迭代
    #     n = 1
    #     n = 2, n = 1 的排列的每一个字符串的每一个位置插入该元素
    #     """
    #     current = {"", }
    #     for i in range(len(s)):
    #         current = self._get_elements(s[i], current)
    #     return list(current)


if __name__ == "__main__":
    s = Solution()
    assert s.permutation("").sort() == [""].sort()
    assert s.permutation("a").sort() == ["a"].sort()
    assert s.permutation("ab").sort() == ["ab", "ba"].sort()
    assert s.permutation("aab").sort() == ["baa", "aab", "aba"].sort()
