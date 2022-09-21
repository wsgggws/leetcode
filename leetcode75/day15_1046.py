import heapq
from typing import List


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        stones = [-stone for stone in stones]
        heapq.heapify(stones)
        while len(stones) > 1:
            value = heapq.heappop(stones) - heapq.heappop(stones)
            if value != 0:
                heapq.heappush(stones, value)
        return -stones[0] if stones else 0


if __name__ == "__main__":
    assert Solution().lastStoneWeight([2, 8, 8, 7, 4, 1, 8, 1]) == 1
