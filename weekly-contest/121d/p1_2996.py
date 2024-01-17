from typing import List


class Solution:
    """https://leetcode.cn/problems/smallest-missing-integer-greater-than-sequential-prefix-sum/"""

    def missingInteger(self, nums: List[int]) -> int:
        prefix_sum = nums[0]
        for i in range(1, len(nums)):
            if nums[i] == nums[i - 1] + 1:
                prefix_sum += nums[i]
            else:
                break
        temp = set(nums)
        while prefix_sum in temp:
            prefix_sum += 1
        return prefix_sum


if __name__ == "__main__":
    assert Solution().missingInteger([1, 2, 3, 2, 5]) == 6
    assert Solution().missingInteger([3, 4, 5, 1, 12, 14, 13]) == 15
