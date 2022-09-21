from typing import List


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        index = 0
        while index < min(len(str) for str in strs):
            if len(set(str[: index + 1] for str in strs)) != 1:
                break
            index += 1
        return strs[0][:index]


if __name__ == "__main__":
    assert Solution().longestCommonPrefix(strs=["flower", "flow", "flight"]) == "fl"
    assert Solution().longestCommonPrefix(strs=["flower", "flow", ""]) == ""
    assert Solution().longestCommonPrefix(strs=["dog", "racecar", "car"]) == ""
