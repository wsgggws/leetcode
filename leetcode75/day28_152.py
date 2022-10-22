from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        """
        状态压缩
        """
        if len(nums) < 1:
            raise KeyError
        maxs = dp_maxs = dp_mins = nums[0]
        for i in range(1, len(nums)):
            pre_dp_maxs = dp_maxs
            dp_maxs = max(dp_maxs * nums[i], dp_mins * nums[i], nums[i])
            # notes, 使用上次的 dp_maxs, 而非刚计算出来的
            dp_mins = min(pre_dp_maxs * nums[i], dp_mins * nums[i], nums[i])
            maxs = max(maxs, dp_maxs)
        return maxs


# class Solution:
#     def maxProduct(self, nums: List[int]) -> int:
#         """

#         -2      (-2)
#         3       (3, -6)
#         2       (2, 6, -12)
#         -1      (-1, -2, -6, 12)
#         7       (7, -7, -14, -42, 84)
#         => 84
#         如何求最大乘积， 正*正，负*负
#                             max             min
#         -2    (,,2)         -2 (-2)          -2 (-2)
#         3     (-6,-6,3)     3  (3)           -6 (-2,3)
#         2     (6,-12,2)     6  (3,2)         -12(-2,3,2)
#         -1    (-6,12,-1)    12 (-2,3,2,-1)   -6 (3,2,-1)
#         7     (84,-42,7)    84 (-2,3,2,-1,7) -42(3,2,-1,7)
#         => 84
#         dp_maxs[i]: 到数据i所构成连续子数据的最大乘积
#         dp_mins[i]: 到数据i所构成连续子数据的最小乘积
#         dp_maxs[i] = max(dp_maxs[i-1]*nums[i], dp_mins[i-1]*nums[i], nums[i])
#         dp_mins[i] = min(dp_maxs[i-1]*nums[i], dp_mins[i-1]*nums[i], nums[i])
#         max(dp_maxs)
#         """
#         if len(nums) < 1:
#             raise KeyError
#         dp_maxs = [0 for _ in nums]
#         dp_mins = [0 for _ in nums]
#         dp_maxs[0] = dp_mins[0] = nums[0]
#         for i in range(1, len(nums)):
#             dp_maxs[i] = max(dp_maxs[i - 1] * nums[i], dp_mins[i - 1] * nums[i], nums[i])
#             dp_mins[i] = min(dp_maxs[i - 1] * nums[i], dp_mins[i - 1] * nums[i], nums[i])
#         return max(dp_maxs)


# class Solution:
#     def maxProduct(self, nums: List[int]) -> int:
#         """
#         -2      (-2)
#         3       (3, -6)
#         2       (2, 6, -12)
#         -1      (-1, -2, -6, 12)
#         7       (7, -7, -14, -42, 84)
#         => 84
#         """
#         if len(nums) < 1:
#             raise KeyError
#         maxs = nums[0]
#         starts = set([nums[0]])
#         for num in nums[1:]:
#             next_starts = set([num])
#             for start in starts:
#                 next_starts.add(num * start)
#             maxs = max(*next_starts, maxs)
#             starts = next_starts
#         return maxs


if __name__ == "__main__":
    assert Solution().maxProduct(nums=[2, 3, -2, 4]) == 6
    assert Solution().maxProduct(nums=[-2, 0, -1]) == 0
    assert Solution().maxProduct(nums=[-2, 3, 2, -1, 7]) == 84
    assert Solution().maxProduct(nums=[-3, -4, -2]) == 12
