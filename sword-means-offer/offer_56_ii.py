from collections import Counter
from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        counter = Counter(nums)
        for value, count in counter.items():
            if count < 3:
                return value


if __name__ == "__main__":
    s = Solution()
    assert s.singleNumber([3, 4, 3, 3]) == 4
    assert s.singleNumber([9, 1, 7, 9, 7, 9, 7]) == 1
