from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        # [)
        left, right = 0, len(nums)
        while left < right:
            mid = left + (right - left) // 2
            if nums[mid] >= target:
                right = mid
            elif nums[mid] < target:
                left = mid + 1
        return left


if __name__ == "__main__":
    assert Solution().search(nums=[-1, 0, 3, 3, 3, 3], target=3) == 2
