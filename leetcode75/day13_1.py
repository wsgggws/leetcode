from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # O(n*n)
        # for i in range(len(nums)):
        #     for j in range(i + 1, len(nums)):
        #         if nums[i] + nums[j] == target:
        #             return [i, j]

        # O(n)
        nums: List[tuple[int, int]] = sorted([(num, index) for index, num in enumerate(nums)])
        left, right = 0, len(nums) - 1
        while left < right:
            num = nums[left][0] + nums[right][0]
            if target == num:
                break
            elif target > num:
                left += 1
            else:
                right -= 1
        return [nums[left][1], nums[right][1]]


if __name__ == "__main__":
    assert Solution().twoSum(nums=[2, 7, 11, 15], target=9) == [0, 1]
    assert Solution().twoSum(nums=[3, 2, 4], target=6) == [1, 2]
    assert Solution().twoSum(nums=[3, 3], target=6) == [0, 1]
