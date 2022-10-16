from collections import defaultdict
from typing import List


class Solution:
    def numBusesToDestination(self, routes: List[List[int]], source: int, target: int) -> int:
        # 不需要乘坐公交车,步行省钱还锻炼身体
        if source == target:
            return 0
        # 创建有向图
        bus = len(routes)
        visit = [False for _ in routes]
        edges = defaultdict(list)
        starts, ends = [], []
        for i in range(bus):
            # 寻找起始点，终点所在的公交线路
            if source in routes[i]:
                starts.append(i)
                visit[i] = True
            if target in routes[i]:
                ends.append(i)
            # 构建有向图
            for j in range(i + 1, bus):
                if set(routes[i]) & set(routes[j]):
                    edges[i].append(j)
                    edges[j].append(i)
        steps = 1
        if any(target in routes[start] for start in starts):
            return steps
        # 广度优先搜索
        while starts:
            size = len(starts)
            for index in range(size):
                cur_route = starts[index]
                for next_route in edges[cur_route]:
                    if next_route in ends:
                        return steps + 1
                    elif visit[next_route] is False:
                        starts.append(next_route)
                        visit[next_route] = True
            starts = starts[size:]
            steps += 1
        return -1


if __name__ == "__main__":
    assert Solution().numBusesToDestination(routes=[[1, 2, 7], [3, 6, 7]], source=1, target=6) == 2
    assert (
        Solution().numBusesToDestination(
            routes=[[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]], source=15, target=12
        )
        == -1
    )
    assert Solution().numBusesToDestination(routes=[[2], [2, 8]], source=8, target=2) == 1
    assert Solution().numBusesToDestination(routes=[[2], [2, 8]], source=8, target=8) == 0
