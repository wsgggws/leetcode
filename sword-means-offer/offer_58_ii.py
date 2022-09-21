class Solution:
    def reverseLeftWords(self, s: str, n: int) -> str:
        return s[n:] + s[:n]


if __name__ == "__main__":
    s = Solution()
    assert s.reverseLeftWords("abcdefg", 2) == "cdefgab"
    assert s.reverseLeftWords("lrloseumgh", 6) == "umghlrlose"
