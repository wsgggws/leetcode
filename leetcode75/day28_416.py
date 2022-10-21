from typing import List


class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        """
             1   5   11  5
         0   T   T   T   F
         1   T   T   T   F
         2   F   F   F   F
         3   F   F   F   F
         4   F   F   F   F
         5   F   T   T   F
         6   F   T   T   F
         7   F   F   F   F
         8   F   F   F   F
         9   F   F   F   F
        10   F   F   F   F
        11   F   F   T   F

        dp[row][col]: 表示前col个元素是否可以构成row这个数字
        dp[row][col] = dp[row][col-1] or dp[row-nums[col]][col-1]
        状态压缩: col 只与 col-1 有关
        dp[row] = dp[row] or dp[row-nums[col]], row 从后往前, 从前往后不行是因为 col 会覆盖 col-1
        """
        total = sum(nums)
        if total % 2 > 0 or len(nums) <= 1:
            return False
        target = total // 2
        dp = [False for _ in range(target + 1)]
        dp[0] = True
        if nums[0] <= target:
            dp[nums[0]] = True
        for col in range(1, len(nums)):
            for row in range(target, 0, -1):
                dp[row] = dp[row] or (row >= nums[col] and dp[row - nums[col]])
            if dp[target] is True:
                return True
        return False


# class Solution:
#     def canPartition(self, nums: List[int]) -> bool:
#         """
#              1   5   11  5
#          0   T   T   T   F
#          1   T   T   T   F
#          2   F   F   F   F
#          3   F   F   F   F
#          4   F   F   F   F
#          5   F   T   T   F
#          6   F   T   T   F
#          7   F   F   F   F
#          8   F   F   F   F
#          9   F   F   F   F
#         10   F   F   F   F
#         11   F   F   T   F

#         dp[row][col]: 表示前col个元素是否可以构成row这个数字
#         dp[row][col] = dp[row][col-1] or dp[row-nums[col]][col-1]
#         """
#         total = sum(nums)
#         if total % 2 > 0 or len(nums) <= 1:
#             return False
#         target = total // 2
#         dp = [[False for _ in nums] for _ in range(total + 1)]
#         dp[0][0] = dp[nums[0]][0] = True
#         for col in range(1, len(nums)):
#             for row in range(target + 1):
#                 dp[row][col] = dp[row][col - 1] or (row >= nums[col] and dp[row - nums[col]][col - 1])
#             if dp[target][col] is True:
#                 return True
#         return False


# class Solution:
#     def canPartition(self, nums: List[int]) -> bool:
#         """
#                     (11,0)
#             (10,1)            (11,1)
#          (5,2)   (10,2)     (6,2)     (11,2)
#         -   (5,3)       -  (6,3)   (0,3) (11,3)
#           (0,4)
#         超时的原因在于重复计数子问题 2**N
#         记录 (target, index) 的解避免重复计算
#         """
#         total = sum(nums)
#         if total % 2 > 0 or len(nums) <= 1:
#             return False
#         memory = dict()
#         return self.search(nums, total // 2, 0, memory)

#     def search(self, nums: List[int], target: int, index: int, memory: dict) -> bool:
#         if target == 0:
#             return True
#         if target < 0:
#             return False
#         if index >= len(nums):
#             return False
#         if memory.get((target, index)) in (True, False):
#             return memory[(target, index)]
#         got = self.search(nums, target - nums[index], index + 1, memory)
#         memory[(target - nums[index], index + 1)] = got
#         not_got = self.search(nums, target, index + 1, memory)
#         memory[(target, index + 1)] = not_got
#         return got or not_got


# class Solution:
#     def canPartition(self, nums: List[int]) -> bool:
#         # 暴力搜索 Timeout
#         total = sum(nums)
#         if total % 2 > 0 or len(nums) <= 1:
#             return False
#         nums.sort()
#         return self.search(nums, total // 2, 0)

#     def search(self, nums: List[int], target: int, index: int):
#         if target == 0:
#             return True
#         if target < 0:
#             return False
#         if index >= len(nums):
#             return False
#         return self.search(nums, target - nums[index], index + 1) or self.search(nums, target, index + 1)


if __name__ == "__main__":
    # assert Solution().canPartition(nums=[1, 5, 11, 5]) is True
    # assert Solution().canPartition(nums=[1, 2, 3, 5]) is False
    # assert Solution().canPartition(nums=[9, 5]) is False
    assert Solution().canPartition(nums=[14, 9, 8, 4, 3, 2]) is True
