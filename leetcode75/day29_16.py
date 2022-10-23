from typing import List


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        """
        1. i,j,k 暴力破解
        2. 双指针
        -4, -1, 1, 2
        i    s-> <-e
        =, return
        >, s+1
        <, e-1
        ><需要考虑记录最接近的值(N*N)
        """
        lens = len(nums)
        if lens < 3:
            raise KeyError
        nums.sort()
        ans = 0
        diff_mins = 100000
        for i in range(lens - 2):
            start = i + 1
            end = lens - 1
            while start < end:
                three_sum = nums[i] + nums[start] + nums[end]
                if three_sum == target:
                    return target
                else:
                    if three_sum < target:
                        start += 1
                    else:
                        end -= 1
                    diff = abs(three_sum - target)
                    if diff < diff_mins:
                        diff_mins = diff
                        ans = three_sum
        return ans


if __name__ == "__main__":
    assert Solution().threeSumClosest(nums=[-1, 2, 1, -4], target=1) == 2
    assert Solution().threeSumClosest(nums=[0, 0, 0], target=1) == 0
