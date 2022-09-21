# 剑指 Offer 57. 和为s的两个数字
# 输入一个递增排序的数组和一个数字s，在数组中查找两个数，使得它们的和正好是s。如果有多对数字的和等于s，则输出任意一对即可。


# 示例 1：

# 输入：nums = [2,7,11,15], target = 9
# 输出：[2,7] 或者 [7,2]
# 示例 2：

# 输入：nums = [10,26,30,31,47,60], target = 40
# 输出：[10,30] 或者 [30,10]


# 限制：

# 1 <= nums.length <= 10^5
# 1 <= nums[i] <= 10^6

from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # 暴力不推荐
        # 用set
        # tmp_set = set(nums)
        # ans = []
        # for num in nums:
        #     if num != target - num and (target - num) in tmp_set:
        #         ans = [num, target - num]
        #         break
        # return ans

        # 用二分搜索
        # ans = []
        # for num in nums:
        #     if num != target - num and self._binary_search(target-num, nums):
        #         ans = [num, target - num]
        #         break
        # return ans

        # 用双指针
        left, right = 0, len(nums) - 1
        ans = []
        while left < right:
            sum_of = nums[left] + nums[right]
            if sum_of > target:
                right -= 1
            elif sum_of < target:
                left += 1
            else:
                ans = [nums[left], nums[right]]
                break
        return ans

    # def _binary_search(self, target, nums):
    #     # [)
    #     left, right = 0, len(nums)
    #     while left < right:
    #         mid = left + (right - left) // 2
    #         if nums[mid] == target:
    #             return True
    #         elif nums[mid] > target:
    #             right = mid
    #         else:
    #             left = mid + 1
    #     return False


if __name__ == "__main__":
    s = Solution()
    s.twoSum([2, 7, 11, 15], 9) == [2, 7]
    s.twoSum([10, 26, 30, 31, 47, 60], 40) == [10, 30]
    # ?
    s.twoSum([2, 7, 11, 15], 10) == []
