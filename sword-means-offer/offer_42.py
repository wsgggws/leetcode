# 剑指 Offer 42. 连续子数组的最大和
# 输入一个整型数组，数组中的一个或连续多个整数组成一个子数组。求所有子数组的和的最大值。

# 要求时间复杂度为O(n)。


# 示例1:

# 输入: nums = [-2,1,-3,4,-1,2,1,-5,4]
# 输出: 6
# 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。


# 提示：

# 1 <= arr.length <= 10^5
# -100 <= arr[i] <= 100

from typing import List


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        """
        0. 暴力 (i..(i..=)
        1. 递归？
        2. dp[i][j] = max(dp[i][j-1] + nums[j], nums[j])
        3. dp[j] = max(dp[j-1] + nums[j], nums[j])
        4. current = max(current + nums[j], nums[j]); ans = max(current, ans)
        """
        # # 0. 暴力 (i..(i..=)
        # if len(nums) < 1:
        #     raise TypeError
        # ans = nums[0]
        # for i in range(0, len(nums)):
        #     sum_of = 0
        #     for j in range(i, len(nums)):
        #         sum_of += nums[j]
        #         ans = max(ans, sum_of)
        # return ans

        # # 2. dp[i][j] = max(dp[i][j-1] + nums[j], nums[j])
        # dp = [[0 for _ in nums] for _ in nums]

        # for i in range(len(nums)):
        #     dp[i][i] = nums[i]
        #     for j in range(i+1, len(nums)):
        #         dp[i][j] = max(dp[i][j-1] + nums[j], nums[j])

        # ans = nums[0]
        # for i in range(len(nums)):
        #     for j in range(i, len(nums)):
        #         ans = max(ans, dp[i][j])
        # return ans

        # # 3. dp[j] = max(dp[j-1] + nums[j], nums[j])
        # dp = [0 for _ in nums]
        # dp[0] = nums[0]
        # for j in range(1, len(nums)):
        #     dp[j] = max(dp[j-1] + nums[j], nums[j])
        # return max(dp)

        # 4. current = max(current + nums[j], nums[j]); ans = max(current, ans)
        current = nums[0]
        ans = nums[0]
        for j in range(1, len(nums)):
            current = max(current + nums[j], nums[j])
            ans = max(current, ans)
        return ans


if __name__ == "__main__":
    s = Solution()
    assert s.maxSubArray([-2, 1]) == 1
    assert s.maxSubArray([-2, -1]) == -1
    assert s.maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6
