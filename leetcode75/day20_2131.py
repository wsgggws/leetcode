from collections import Counter
from typing import List


class Solution:
    def longestPalindrome(self, words: List[str]) -> int:
        """
        1. “两字符相同”的
            a. 有奇数的，取偶数对， 最后加 1 对
            b. 无奇数对，取偶数对
        2. “两字符不同“的取 min(count(AB), count(BA))
        """
        counter1 = Counter([word for word in words if word[0] == word[1]])
        counter2 = Counter([word for word in words if word[0] != word[1]])
        ans = 0
        # 不相等字符的单词
        for word in counter2.keys():
            ans += 2 * min(counter2[word], counter2[word[::-1]])
        # 相等字符的单词
        odd = 0
        for value in counter1.values():
            if value % 2 == 0:
                ans += 2 * value
            else:
                odd = 1
                ans += value // 2 * 2 * 2
        ans += 2 * odd
        return ans


if __name__ == "__main__":
    assert Solution().longestPalindrome(words=["lc", "cl", "gg"]) == 6
    assert Solution().longestPalindrome(words=["ab", "ty", "yt", "lc", "cl", "ab"]) == 8
    assert Solution().longestPalindrome(words=["cc", "ll", "xx"]) == 2
    assert Solution().longestPalindrome(words=["cx", "lc", "cx"]) == 0
    assert (
        Solution().longestPalindrome(
            words=["dd", "aa", "bb", "dd", "aa", "dd", "bb", "dd", "aa", "cc", "bb", "cc", "dd", "cc"]
        )
        == 22
    )
