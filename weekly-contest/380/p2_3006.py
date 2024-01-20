from typing import List


class Solution:
    def beautifulIndices(self, s: str, a: str, b: str, k: int) -> List[int]:
        a_indexes = self._find_all_indexes(s, a)
        b_indexes = self._find_all_indexes(s, b)
        ans = []
        for a_index in a_indexes:
            if self._find_abs_in_k(a_index, b_indexes, k):
                ans.append(a_index)
        return ans

    def _find_abs_in_k(self, a_index, b_indexes, k):
        for b_index in b_indexes:
            if abs(a_index - b_index) <= k:
                return True
        return False

    def _find_all_indexes(self, text: str, substring: str):
        indexes = []
        start = 0

        while True:
            try:
                index = text.index(substring, start)
                indexes.append(index)
                start = index + 1
            except ValueError:
                break

        return indexes


if __name__ == "__main__":
    assert Solution().beautifulIndices(s="isawsquirrelnearmysquirrelhouseohmy", a="my", b="squirrel", k=15) == [16, 33]
    assert Solution().beautifulIndices(s="abcd", a="a", b="a", k=4) == [0]
