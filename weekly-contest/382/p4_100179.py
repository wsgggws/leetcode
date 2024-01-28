from typing import List


class Solution:
    def minOrAfterOperations(self, nums: List[int], k: int) -> int:
        # TODO:
        ...


if __name__ == "__main__":
    assert Solution().minOrAfterOperations(nums=[3, 5, 3, 2, 7], k=2) == 3
    assert Solution().minOrAfterOperations(nums=[7, 3, 15, 14, 2, 8], k=4) == 2
    assert Solution().minOrAfterOperations(nums=[10, 7, 10, 3, 9, 14, 9, 4], k=1) == 15
