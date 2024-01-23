from collections import Counter
from typing import List


class Solution:
    def countOfPairs(self, n: int, x: int, y: int) -> List[int]:
        distances = [[n * 2 + 1 for _ in range(n)] for _ in range(n)]
        for i in range(n):
            if i + 1 < n:
                distances[i][i + 1] = 1
            if i - 1 >= 0:
                distances[i][i - 1] = 1
        if x != y:
            distances[x - 1][y - 1] = 1
            distances[y - 1][x - 1] = 1
        for k in range(n):
            for i in range(n):
                for j in range(n):
                    if i != j and distances[i][j] > distances[i][k] + distances[k][j]:
                        distances[i][j] = distances[i][k] + distances[k][j]
        counter = Counter()
        for i in range(n):
            for j in range(n):
                if i != j:
                    counter[distances[i][j]] += 1
        ans = []
        for i in range(n):
            ans.append(counter[i + 1])
        return ans


if __name__ == "__main__":
    assert Solution().countOfPairs(n=3, x=1, y=3) == [6, 0, 0]
    assert Solution().countOfPairs(n=4, x=1, y=1) == [6, 4, 2, 0]
    assert Solution().countOfPairs(n=5, x=2, y=4) == [10, 8, 2, 0, 0]
    assert Solution().countOfPairs(n=6, x=2, y=6) == [12, 14, 4, 0, 0, 0]
    assert Solution().countOfPairs(n=1, x=1, y=1) == [0]
