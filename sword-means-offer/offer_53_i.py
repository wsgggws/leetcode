# 剑指 Offer 53 - I. 在排序数组中查找数字 I
# 统计一个数字在排序数组中出现的次数。


# 示例 1:

# 输入: nums = [5,7,7,8,8,10], target = 8
# 输出: 2
# 示例 2:

# 输入: nums = [5,7,7,8,8,10], target = 6
# 输出: 0


# 提示：

# 0 <= nums.length <= 105
# -109 <= nums[i] <= 109
# nums 是一个非递减数组
# -109 <= target <= 109


from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        # [)
        left, right = 0, len(nums)
        while left < right:
            mid = left + (right - left) // 2
            if nums[mid] >= target:
                right = mid
            elif nums[mid] < target:
                left = mid + 1
        count = 0
        while left < len(nums) and nums[left] == target:
            count += 1
            left += 1
        return count


if __name__ == "__main__":
    s = Solution()
    assert s.search([5, 7, 7, 8, 8, 10], 8) == 2
    assert s.search([5, 7, 7, 8, 8, 10], 7) == 2
    assert s.search([5, 7, 7, 8, 8, 10], 6) == 0
