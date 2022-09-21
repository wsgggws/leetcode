from typing import List


class Solution:
    def runningSum(self, nums: List[int]) -> List[int]:
        result = []
        sum = 0
        for num in nums:
            sum += num
            result.append(sum)
        return result


if __name__ == "__main__":
    s = Solution()
    assert s.runningSum([1, 2, 3, 4]) == [1, 3, 6, 10]
    assert s.runningSum([1, 1, 1, 1, 1]) == [1, 2, 3, 4, 5]
    assert s.runningSum([3, 1, 2, 10, 1]) == [3, 4, 6, 16, 17]
