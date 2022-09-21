from collections import Counter


class Solution:
    def longestPalindrome(self, s: str) -> int:
        counter = Counter(s)
        ans = 0
        for value in counter.values():
            ans += value // 2 * 2
            if value % 2 and ans % 2 == 0:
                ans += 1
        return ans


if __name__ == "__main__":
    assert Solution().longestPalindrome("abccccdd") == 7
    assert Solution().longestPalindrome("a") == 1
