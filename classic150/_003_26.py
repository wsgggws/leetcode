from typing import List


class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        index = cur = 0
        while cur + 1 < len(nums):
            # 找到一个与当前不相同的索引
            while cur + 1 < len(nums) and nums[cur + 1] == nums[cur]:
                cur = cur + 1
            # 找到后，原地赋值，索引往后移动，元素个数加一
            # 找不到的 case 表示后面几位一样，不需要操作，由外层 while 结束
            if cur + 1 < len(nums):
                nums[index + 1] = nums[cur + 1]
                cur += 1
                index += 1
        return index + 1


if __name__ == "__main__":
    nums = [1, 1, 2]
    assert Solution().removeDuplicates(nums=nums) == 2
    assert nums[:2] == [1, 2]

    nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
    assert Solution().removeDuplicates(nums=nums) == 5
    assert nums[:5] == [0, 1, 2, 3, 4]

    nums = [1]
    assert Solution().removeDuplicates(nums=nums) == 1
    assert nums[:1] == [1]

    nums = [1, 1]
    assert Solution().removeDuplicates(nums=nums) == 1
    assert nums[:1] == [1]

    nums = [1, 2, 3, 4]
    assert Solution().removeDuplicates(nums=nums) == 4
    assert nums[:4] == [1, 2, 3, 4]
    # print(nums)
    # assert 1 == 2
