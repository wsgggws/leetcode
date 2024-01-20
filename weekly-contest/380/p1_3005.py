from collections import Counter
from typing import List


class Solution:
    def maxFrequencyElements(self, nums: List[int]) -> int:
        counter = Counter(nums)
        _, times = counter.most_common(1)[0]
        return sum(value for key, value in counter.items() if value == times)


if __name__ == "__main__":
    assert Solution().maxFrequencyElements(nums=[1, 2, 4]) == 3
    assert Solution().maxFrequencyElements(nums=[1, 2, 2, 3, 1, 4]) == 4
    assert Solution().maxFrequencyElements(nums=[1, 2, 3, 4, 5]) == 5
