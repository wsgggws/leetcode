# https://leetcode.cn/problems/remove-element/?envType=study-plan-v2&id=top-interview-150
from typing import List


class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        start, end = 0, len(nums)
        total = 0
        while start < end:
            # <---------------|---->
            #           找到可以替换的 end, 并往后走一步，记录+1
            while end > start:
                if nums[end - 1] == val:
                    end -= 1
                    total += 1
                else:
                    break
            # <-----|----------|---->
            #   找到可以替换的 start
            # 交换 start, end, 并均往前往后走一步, 记录+1
            while start < end:
                if nums[start] == val:
                    nums[start], nums[end - 1] = nums[end - 1], nums[start]
                    end -= 1
                    start += 1
                    total += 1
                    break
                start += 1
        # 返回总长度 - 记录值，为新长度
        return len(nums) - total


if __name__ == "__main__":
    nums = [1]
    assert Solution().removeElement(nums=nums, val=1) == 0
    assert nums == [1]

    nums = [3, 2, 2, 3]
    assert Solution().removeElement(nums=nums, val=3) == 2
    assert nums == [2, 2, 3, 3]

    nums = [2, 2, 3, 3]
    assert Solution().removeElement(nums=nums, val=3) == 2
    assert nums == [2, 2, 3, 3]

    nums = [3, 3, 3, 3]
    assert Solution().removeElement(nums=nums, val=3) == 0
    assert nums == [3, 3, 3, 3]

    nums = [0, 1, 2, 2, 3, 0, 4, 2]
    assert Solution().removeElement(nums=nums, val=2) == 5
    assert nums[:5] == [0, 1, 4, 0, 3]
