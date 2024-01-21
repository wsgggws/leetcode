"""
https://leetcode.cn/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/description/
"""
from typing import List


class Solution:
    def minimumCost(self, nums: List[int], k: int, dist: int) -> int:
        # TODO
        ...


if __name__ == "__main__":
    assert Solution().minimumCost(nums=[1, 3, 2, 6, 4, 2], k=3, dist=3) == 5
    assert Solution().minimumCost(nums=[10, 1, 2, 2, 2, 1], k=4, dist=3) == 15
    assert Solution().minimumCost(nums=[10, 8, 18, 9], k=3, dist=1) == 36
