from collections import Counter


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        """滑动窗口
            s      s        s      s      s       s
            pwwkew pwwkew pwwkew pwwkew pwwkew pwwkew
            e       e       e       e       e       e

        max 1       2       1       2       3       3
        => 3
        """
        maxs = start = end = 0
        counter = Counter()
        while end < len(s):
            if counter[s[end]] <= 0:
                counter[s[end]] += 1
            else:
                while s[start] != s[end]:
                    counter[s[start]] -= 1
                    start += 1
                start += 1
            maxs = max(maxs, end - start + 1)
            end += 1
        return maxs


if __name__ == "__main__":
    assert Solution().lengthOfLongestSubstring("abcabcbb") == 3
    assert Solution().lengthOfLongestSubstring("bbbb") == 1
    assert Solution().lengthOfLongestSubstring("pwwkew") == 3
    assert Solution().lengthOfLongestSubstring("tmmzuxt") == 5
