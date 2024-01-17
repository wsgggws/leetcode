from functools import reduce
from typing import List


class Solution:
    def minOperations(self, nums: List[int], k: int) -> int:
        value = reduce(lambda x, y: x ^ y, nums) ^ k
        count = 0
        while value:
            if value & 1:
                count += 1
            value >>= 1
        return count


if __name__ == "__main__":
    assert Solution().minOperations([2, 1, 3, 4], 1) == 2
    assert Solution().minOperations([2, 0, 2, 0], 0) == 0
    assert Solution().minOperations([4], 7) == 2
