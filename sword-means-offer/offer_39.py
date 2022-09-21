# 剑指 Offer 39. 数组中出现次数超过一半的数字
# 数组中有一个数字出现的次数超过数组长度的一半，请找出这个数字。


# 你可以假设数组是非空的，并且给定的数组总是存在多数元素。


# 示例 1:

# 输入: [1, 2, 3, 2, 2, 2, 5, 4, 2]
# 输出: 2


# 限制：

# 1 <= 数组长度 <= 50000

from typing import List


class Solution:
    # def majorityElement(self, nums: List[int]) -> int:
    #     return sorted(nums)[len(nums) // 2]

    def majorityElement(self, nums: List[int]) -> int:
        ans = nums[0]
        count = 1
        for i in range(1, len(nums)):
            if nums[i] == ans:
                count += 1
            else:
                count -= 1
                if count == 0:
                    ans = nums[i]
                    count = 1
        return ans


if __name__ == "__main__":
    s = Solution()
    assert s.majorityElement([2]) == 2
    assert s.majorityElement([2, 1, 2]) == 2
    assert s.majorityElement([1, 2, 3, 2, 2, 2, 5, 4, 2]) == 2
