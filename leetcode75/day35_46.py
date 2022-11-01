from itertools import permutations
from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        """4. 使用 itertools"""
        return [list(item) for item in permutations(nums)]


# class Solution:
#     def permute(self, nums: List[int]) -> List[List[int]]:
#         """3. 搜索+

#                 1                   2                   3
#             2       3
#         3               2
#         """
#         self.ans = []
#         visit = [False for _ in nums]
#         self.dfs(nums, visit, [])
#         return self.ans

#     def dfs(self, nums, visit, item):
#         if len(item) == len(nums):
#             self.ans.append(item)
#             return
#         for i in range(len(nums)):
#             if visit[i] is False:
#                 visit[i] = True
#                 self.dfs(nums, visit, item + [nums[i]])
#                 visit[i] = False


# class Solution:
#     def permute(self, nums: List[int]) -> List[List[int]]:
#         """2. 迭代
#         先解决子问题, 推导出大问题
#         dp[0] = [[1]]
#         # dp[i] 是前一项 dp[i-1] 每个 item 的每个位置插入 nums[i]
#         dp[1] = [
#             item[:index] + nums[1] + item[index:]
#             for item in dp[0]
#                 for index in range(len(item) + 1)
#         ]
#         """
#         # dp = [[] for _ in nums]
#         # dp[0] = [[nums[0]]]
#         # for i in range(1, len(nums)):
#         #     for sub in dp[i - 1]:
#         #         for j in range(len(sub) + 1):
#         #             dp[i].append(sub[:j] + [nums[i]] + sub[j:])
#         # return dp[len(nums) - 1]

#         # 空间优化, dp[i] 只与 dp[i-1]相关
#         dp_0 = [[nums[0]]]
#         for i in range(1, len(nums)):
#             dp_1 = []
#             for sub in dp_0:
#                 for j in range(len(sub) + 1):
#                     dp_1.append(sub[:j] + [nums[i]] + sub[j:])
#             dp_0 = dp_1
#         return dp_0


# class Solution:
#     def permute(self, nums: List[int]) -> List[List[int]]:
#         """1. 递归
#         要解决大问题，先解决子问题
#         如果我解决了 permute(nums[1:]), 如何得到 premute(nums) 的答案呢？
#         for sub in permute(nums[1:]):
#            for i in range(len(sub) + 1):
#               sub[:i] + nums[0] + sub[i:]
#         即将 nums[0] 插在每个item的每个位置
#         """
#         if len(nums) == 1:
#             return [[nums[0]]]
#         per_permute = self.permute(nums[1:])
#         ans = []
#         for sub in per_permute:
#             for i in range(len(sub) + 1):
#                 ans.append(sub[:i] + [nums[0]] + sub[i:])
#         return ans


if __name__ == "__main__":
    assert sorted(Solution().permute(nums=[1, 2, 3])) == sorted(
        [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
    )
    assert sorted(Solution().permute(nums=[0, 1])) == sorted([[0, 1], [1, 0]])
    assert Solution().permute(nums=[1]) == [[1]]
