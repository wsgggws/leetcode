# 请从字符串中找出一个最长的不包含重复字符的子字符串，计算该最长子字符串的长度。

# 示例 1:

# 输入: "abcabcbb"
# 输出: 3
# 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
# 示例 2:

# 输入: "bbbbb"
# 输出: 1
# 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
# 示例 3:

# 输入: "pwwkew"
# 输出: 3
# 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
#      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
#

# 提示：

# s.length <= 40000


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        """
        abcabcbb
        123?
        maps = {a: 0, b: 1, c: 2}
        --
        abcabcbb
         bca
           3 # cur_index(a) - old_index(a) = 3 - 0 = 3
        maps = {a: 0, b: 1, c: 2}
        maps = {a: 3, b: 1, c: 2}
        """
        maps = {}
        maxs = 0
        current = 0
        start_index = 0
        for index, char in enumerate(s):
            if char in maps and maps[char] >= start_index:
                current = index - maps[char]
                start_index = maps[char] + 1
            else:
                current += 1
            maps[char] = index
            maxs = max(maxs, current)
        return maxs


if __name__ == "__main__":
    s = Solution()
    assert s.lengthOfLongestSubstring("abcabcbb") == 3
    assert s.lengthOfLongestSubstring("bbbbb") == 1
    assert s.lengthOfLongestSubstring("pwwkew") == 3
    assert s.lengthOfLongestSubstring("") == 0
    assert s.lengthOfLongestSubstring(" ") == 1
    assert s.lengthOfLongestSubstring("abcba") == 3
