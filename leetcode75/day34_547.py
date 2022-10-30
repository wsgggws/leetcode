from collections import defaultdict
from typing import List


class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        """
        1. 构建有向图 edges:
            {
                0: [1],
                1: [0],
                2: [],
            }
        2. 广度优先搜索次数即为省份数量
        """
        n = len(isConnected)
        if n < 1 or len(isConnected[0]) < 1:
            raise KeyError
        edges = defaultdict(list)
        for i in range(n):
            for j in range(n):
                if i != j and isConnected[i][j] == 1:
                    edges[i].append(j)
        visit = [False for _ in range(n)]
        ans = 0
        for i in range(n):
            if visit[i] is False:
                self.bfs(edges, i, visit)
                ans += 1
        return ans

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
    assert Solution().findCircleNum(isConnected=[[1, 1, 0], [1, 1, 0], [0, 0, 1]]) == 2
    assert Solution().findCircleNum(isConnected=[[1, 0, 0], [0, 1, 0], [0, 0, 1]]) == 3
