from typing import List


class Solution:
    def pivotIndex(self, nums: List[int]) -> int:
        total = sum(nums)
        left = 0
        for index, num in enumerate(nums):
            if left * 2 + nums[index] == total:
                return index
            left += num
        return -1


if __name__ == "__main__":
    s = Solution()
    assert s.pivotIndex([1, 7, 3, 6, 5, 6]) == 3
    assert s.pivotIndex([1, 2, 3]) == -1
    assert s.pivotIndex([2, 1, -1]) == 0
