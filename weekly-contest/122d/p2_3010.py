from typing import List, Optional


class Solution:
    def canSortArray(self, nums: List[int]) -> bool:
        return self._can_up_order(nums)

    # def _can_down_order(self, nums: Optional[List[int]] = None) -> bool:
    #     if nums is None:
    #         nums = []
    #     for i in range(0, len(nums)):
    #         for j in range(i + 1, len(nums)):
    #             if nums[i] < nums[j]:
    #                 if self._has_equal_one_bit(nums[i], nums[j]):
    #                     nums[i], nums[j] = nums[j], nums[i]
    #                 else:
    #                     return False
    #     return True

    def _can_up_order(self, nums: Optional[List[int]] = None) -> bool:
        if nums is None:
            nums = []
        for i in range(0, len(nums)):
            for j in range(i + 1, len(nums)):
                if nums[i] > nums[j]:
                    if self._has_equal_one_bit(nums[i], nums[j]):
                        nums[i], nums[j] = nums[j], nums[i]
                    else:
                        return False
        return True

    def _has_equal_one_bit(self, x: int, y: int) -> bool:
        return self._get_one_bit_count(x) == self._get_one_bit_count(y)

    def _get_one_bit_count(self, number: int) -> int:
        cnt = 0
        while number:
            if number & 1:
                cnt += 1
            number >>= 1
        return cnt


if __name__ == "__main__":
    assert Solution().canSortArray(nums=[8, 4, 2, 30, 15]) is True
    assert Solution().canSortArray(nums=[1, 2, 3, 4, 5]) is True
    assert Solution().canSortArray(nums=[3, 16, 8, 4, 2]) is False
