from typing import List

# class Solution:
#     def rob(self, nums: List[int]) -> int:
#         """
#         2   7    9   3   1

#         2
#             7
#                 max(2+9,7)
#                     不取9，取9
#         """
#         # dp[i]: 0-i 所能获得的最大金钱
#         # dp[i] = max(dp[i-2]+nums[i], dp[i-1])
#         if len(nums) <= 1:
#             return nums[0]
#         dp = [0 for _ in nums]
#         dp[0] = nums[0]
#         dp[1] = max(nums[0], nums[1])
#         for i in range(2, len(nums)):
#             dp[i] = max(dp[i - 2] + nums[i], dp[i - 1])
#         return dp[len(nums) - 1]


class Solution:
    def rob(self, nums: List[int]) -> int:
        if len(nums) <= 1:
            return nums[0]
        # 状态压缩
        dp_0 = nums[0]
        dp_1 = max(nums[0], nums[1])
        for i in range(2, len(nums)):
            dp_0, dp_1 = dp_1, max(dp_0 + nums[i], dp_1)
        return dp_1


if __name__ == "__main__":
    assert Solution().rob([1]) == 1
    assert Solution().rob([1, 2, 3, 1]) == 4
    assert Solution().rob([2, 7, 9, 3, 1]) == 12
