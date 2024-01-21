from typing import List


class Solution:
    def minimumCost(self, nums: List[int]) -> int:
        ans = nums[0]
        nums = nums[1:]
        x1 = min(nums)
        nums.remove(x1)
        ans += x1
        x2 = min(nums)
        ans += x2
        nums.remove(x2)
        return ans


if __name__ == "__main__":
    assert Solution().minimumCost(nums=[1, 2, 3, 12]) == 6
    assert Solution().minimumCost(nums=[10, 3, 1, 1]) == 12
    assert Solution().minimumCost(nums=[5, 4, 3]) == 12
