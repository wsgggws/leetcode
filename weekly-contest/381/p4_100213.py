from typing import List


class Solution:
    def countOfPairs(self, n: int, x: int, y: int) -> List[int]:
        # TODO
        # distances = {}
        # for i in range(1, n + 1):
        #     for j in range(i + 1, n + 1):
        #         distances[(i, j)] = j - i
        #         distances[(j, i)] = j - i
        # if x != y:
        #     for i in range(y, n + 1):
        #         distances[(x, i)] = 1 + (i - y)
        #         distances[(i, x)] = 1 + (i - y)
        #     for i in range(x, 0, -1):
        #         distances[(y, i)] = 1 + (x - i)
        #         distances[(i, y)] = 1 + (x - i)
        # print("[/Users/hj.tian/github/leetcode/weekly-contest/381/p2_100188.py:10] distances: ", distances)
        # ans = []
        # for i in range(1, n + 1):
        #     count = sum(1 if value == i else 0 for value in distances.values())
        #     ans.append(count)
        # print("[/Users/hj.tian/github/leetcode/weekly-contest/381/p2_100188.py:18] ans: ", ans)
        # return ans
        ...


if __name__ == "__main__":
    assert Solution().countOfPairs(n=3, x=1, y=3) == [6, 0, 0]
    assert Solution().countOfPairs(n=4, x=1, y=1) == [6, 4, 2, 0]
    assert Solution().countOfPairs(n=5, x=2, y=4) == [10, 8, 2, 0, 0]
    assert Solution().countOfPairs(n=6, x=2, y=6) == [12, 14, 4, 0, 0, 0]
    assert Solution().countOfPairs(n=1, x=1, y=1) == [0]
