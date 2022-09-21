# 给定一个含有 n 个正整数的数组和一个正整数 target 。

# 找出该数组中满足其和 ≥ target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，
# 并返回其长度。如果不存在符合条件的子数组，返回 0 。

#

# 示例 1：

# 输入：target = 7, nums = [2,3,1,2,4,3]
# 输出：2
# 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
# 示例 2：

# 输入：target = 4, nums = [1,4,4]
# 输出：1
# 示例 3：

# 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
# 输出：0
#

# 提示：

# 1 <= target <= 109
# 1 <= nums.length <= 105
# 1 <= nums[i] <= 105

from typing import List


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        """
        [2] 3 1 2 4 3 N
        [2 3 1 2] 4 3 Y 4
        2 [3 1 2] 4 3 N
        2 [3 1 2 4] 3 Y 4
        2 3 [1 2 4] 3 Y 3
        2 3 1 [2 4] 3 N
        2 3 1 [2 4 3] Y 3
        2 3 1 2 [4 3] Y 2
        2 3 1 2 4 [3] N
        => 2
        """
        left_position, right_position = 0, 0
        sum_of_l2r = 0
        min_length = len(nums) + 1

        while right_position < len(nums):
            sum_of_l2r += nums[right_position]
            if sum_of_l2r >= target:
                while sum_of_l2r - nums[left_position] >= target:
                    sum_of_l2r -= nums[left_position]
                    left_position += 1
                # 1 个元素肯定是最小长度
                if left_position == right_position:
                    return 1
                min_length = min(min_length, right_position - left_position + 1)
            right_position += 1

        if min_length == len(nums) + 1:
            return 0
        return min_length


if __name__ == "__main__":
    s = Solution()
    assert s.minSubArrayLen(7, [2, 3, 1, 2, 4, 3]) == 2
    assert s.minSubArrayLen(4, [1, 4]) == 1
    assert s.minSubArrayLen(5, [1, 4]) == 2
    assert s.minSubArrayLen(4, [1, 4, 4]) == 1
    assert s.minSubArrayLen(11, [1, 1, 1, 1, 1, 1, 1, 1]) == 0
