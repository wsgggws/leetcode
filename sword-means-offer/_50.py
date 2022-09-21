# 在字符串 s 中找出第一个只出现一次的字符。如果没有，返回一个单空格。 s 只包含小写字母。

# 示例 1:

# 输入：s = "abaccdeff"
# 输出：'b'
# 示例 2:

# 输入：s = ""
# 输出：' '
#

# 限制：

# 0 <= s 的长度 <= 50000


# from collections import Counter


class Solution:
    # def firstUniqChar(self, s: str) -> str:
    #     """ """
    #     # O(N+N) = O(2N)
    #     counter = Counter(s)
    #     for char in s:
    #         if counter[char] == 1:
    #             return char
    #     return " "
    def firstUniqChar(self, s: str) -> str:
        """
        O(N)
        python3.6 dict 是按插入时间有序的特性，or OrderSet
        aabbac
        maps = {a:a}
        maps = {c:c}
        duplicated = {a,b}
        """
        maps = {}
        duplicated = set()

        for char in s:
            if char in maps:
                maps.pop(char)
                duplicated.add(char)
            else:
                if char not in duplicated:
                    maps[char] = char

        for char in maps:
            return char
        return " "


if __name__ == "__main__":
    s = Solution()
    assert s.firstUniqChar("abaccdefff") == "b"
    assert s.firstUniqChar("aabbac") == "c"
    assert s.firstUniqChar("") == " "
