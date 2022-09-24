from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        """
        [                   )
        4, 5, 6, 7, 0, 1, 2
                 |
        左边有序，右边有可能有序,即至少有一边是有序的 1234567, 7012345
        0. target == nums[mid]:
            return mid
        1. 左边有序 nums[0] < nums[mid]
            target 是不是在左边？
                是: [left, mid)  # nums[left] <= target < nums[mid]
                否: [mid+1, right)
        2. 右边有序
            target 是不是在右边？
                是: [mid+1, right)  # nums[mid] < target <= nums[right-1]
                否: [left, mid)
        """
        left, right = 0, len(nums)
        # [)
        while left < right:
            mid = left + (right - left) // 2
            if target == nums[mid]:
                return mid
            elif nums[0] < nums[mid]:
                if nums[left] <= target < nums[mid]:
                    right = mid
                else:
                    left = mid + 1
            else:
                if nums[mid] < target <= nums[right - 1]:
                    left = mid + 1
                else:
                    right = mid
        return -1


if __name__ == "__main__":
    assert Solution().search(nums=[4, 5, 6, 7, 0, 1, 2], target=0) == 4
    assert Solution().search(nums=[4, 5, 6, 7, 0, 1, 2], target=3) == -1
    assert Solution().search(nums=[4, 5, 6, 7, 0, 1, 2], target=4) == 0
    assert Solution().search(nums=[4, 5, 6, 7, 0, 1, 2], target=2) == 6
    assert Solution().search(nums=[4, 5, 6, 7, 0, 1, 2], target=7) == 3
    assert Solution().search(nums=[1], target=0) == -1
