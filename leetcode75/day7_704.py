from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        # [)
        left, right = 0, len(nums)
        while left < right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid
        return -1


if __name__ == "__main__":
    assert Solution().search(nums=[-1, 0, 3, 5, 9, 12], target=9) == 4
    assert Solution().search([-1, 0, 3, 5, 9, 12], target=2) == -1
