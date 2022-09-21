from typing import List


class Solution:
    def isStraight(self, nums: List[int]) -> bool:
        zeros = 0
        minimum = 100
        for num in nums:
            if num == 0:
                zeros += 1
            elif num < minimum:
                minimum = num
        counts = 0
        while True:
            if minimum in nums:
                counts += 1
            elif zeros > 0:
                counts += 1
                zeros -= 1
            else:
                break
            minimum += 1
        return counts == 5


if __name__ == "__main__":
    s = Solution()
    assert s.isStraight([1, 2, 3, 4, 5]) is True
    assert s.isStraight([1, 2, 3, 4, 6]) is False
    assert s.isStraight([0, 0, 1, 2, 5]) is True
