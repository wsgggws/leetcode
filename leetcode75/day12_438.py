from collections import Counter
from typing import List


class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        slen, plen = len(s), len(p)
        ans = []
        p_counter = Counter(p)
        for i in range(slen - plen + 1):
            if p_counter == Counter(s[i : i + plen]):
                ans.append(i)
        return ans


if __name__ == "__main__":
    assert Solution().findAnagrams(s="cbaebabacd", p="abc") == [0, 6]
    assert Solution().findAnagrams(s="abab", p="ab") == [0, 1, 2]
    assert Solution().findAnagrams(s="ab", p="abab") == []
