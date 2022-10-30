from collections import defaultdict
from typing import List


class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:

        n = len(stones)
        if n < 1:
            raise KeyError
        edges = defaultdict(list)
        for i in range(n):
            for j in range(i + 1, n):
                if stones[i][0] == stones[j][0] or stones[i][1] == stones[j][1]:
                    edges[i].append(j)
                    # notes, 是双向图
                    edges[j].append(i)
        visit = [False for _ in range(n)]
        connected = 0
        for i in range(n):
            if visit[i] is False:
                self.bfs(edges, i, visit)
                connected += 1
        return n - connected

    def bfs(self, edges, start, visit):
        queue = [start]
        while queue:
            size = len(queue)
            for i in range(size):
                visit[queue[i]] = True
                for city in edges[queue[i]]:
                    if visit[city] is False:
                        queue.append(city)
            queue = queue[size:]


if __name__ == "__main__":
    assert Solution().removeStones(stones=[[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]) == 5
    assert Solution().removeStones(stones=[[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]) == 3
    assert Solution().removeStones(stones=[[0, 0]]) == 0
    assert Solution().removeStones(stones=[[0, 1], [1, 0], [1, 1]]) == 2
