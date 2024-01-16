from typing import List


class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        win = nums[0]
        count = 1
        for num in nums[1:]:
            if win == num:
                count += 1
            elif win != num:
                count -= 1
                if count < 0:
                    win = num
                    count = 1
        return win


if __name__ == "__main__":
    assert Solution().majorityElement(nums=[3, 2, 3]) == 3
    assert Solution().majorityElement(nums=[2, 2, 1, 1, 1, 2, 2]) == 2
