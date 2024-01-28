class Solution:
    def countKeyChanges(self, s: str) -> int:
        ans = 0
        index = 0
        s = s.lower()
        pre_char = s[0]
        while index < len(s):
            if s[index] != pre_char:
                ans += 1
                pre_char = s[index]
            index += 1
        return ans


if __name__ == "__main__":
    assert Solution().countKeyChanges("abc") == 2
    assert Solution().countKeyChanges("aAbBcC") == 2
    assert Solution().countKeyChanges("AaAaAaaA") == 0
