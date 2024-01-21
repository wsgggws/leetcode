from typing import List


class Solution:
    def minimumArrayLength(self, nums: List[int]) -> int:
        # TODO: timeout
        nums.sort()
        temp_set = set(nums)
        zero = 0
        while True and len(nums) > 1:
            maxs = nums[-1]
            if maxs == 0:
                zero += 1
                nums.remove(maxs)
            mins = nums[0]
            if mins == 0:
                zero += 1
                nums.remove(mins)
            if maxs > 0 and mins > 0:
                maybe1 = mins % maxs
                maybe2 = maxs % mins
                if mins % maxs == 0:
                    zero += 1
                    nums = nums[1:-1]
                elif maybe2 != 0 and maybe2 not in temp_set:
                    nums = [maybe2] + nums[1:-1]
                    temp_set.add(maybe2)
                else:
                    nums = [maybe1] + nums[1:-1]
                    temp_set.add(maybe1)
        return len(nums) + zero


if __name__ == "__main__":
    assert Solution().minimumArrayLength(nums=[1, 4, 3, 1]) == 1
    assert Solution().minimumArrayLength(nums=[5, 5, 5, 10, 5]) == 2
    assert Solution().minimumArrayLength(nums=[2, 3, 4]) == 1
    assert Solution().minimumArrayLength(nums=[3, 3, 1]) == 1
    assert Solution().minimumArrayLength(nums=[0, 0, 0]) == 3
    assert Solution().minimumArrayLength(nums=[0, 0, 1]) == 3
    assert Solution().minimumArrayLength(nums=[5, 2, 2, 2, 9, 10]) == 1
