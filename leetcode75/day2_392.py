class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        si, ti = 0, 0
        while ti < len(t) and si < len(s):
            if t[ti] == s[si]:
                si += 1
            ti += 1
        return si == len(s)


if __name__ == "__main__":
    assert Solution().isSubsequence("", "ahbgdc") is True
    assert Solution().isSubsequence("abc", "ahbgdc") is True
    assert Solution().isSubsequence("abc", "abc") is True
    assert Solution().isSubsequence("abc", "ab") is False
    assert Solution().isSubsequence("axc", "ahbgdc") is False
