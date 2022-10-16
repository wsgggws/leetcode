from collections import defaultdict
from typing import List


class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        digraph = defaultdict(list)
        in_degree = [0 for _ in range(numCourses)]
        # 构建有向图
        for dependcy in prerequisites:
            # 0: [1, 2]
            digraph[dependcy[1]].append(dependcy[0])
            in_degree[dependcy[0]] += 1
        # 广度优先搜索
        result = []
        queue = [index for index in range(numCourses) if in_degree[index] == 0]
        while queue:
            size = len(queue)
            for start_index in range(size):
                start_node = queue[start_index]
                result.append(start_node)
                for end_node in digraph[start_node]:
                    in_degree[end_node] -= 1
                    if in_degree[end_node] == 0:
                        queue.append(end_node)
            queue = queue[size:]
        # 判断是否可习完
        if len(result) == numCourses:
            return result
        return []


if __name__ == "__main__":
    ans = Solution().findOrder(numCourses=2, prerequisites=[[1, 0]])
    assert sorted(ans) == sorted([0, 1])

    ans = Solution().findOrder(numCourses=4, prerequisites=[[1, 0], [2, 0], [3, 1], [3, 2]])
    assert sorted(ans) == sorted([0, 2, 1, 3])

    ans = Solution().findOrder(numCourses=1, prerequisites=[])
    assert sorted(ans) == sorted([0])

    ans = Solution().findOrder(numCourses=2, prerequisites=[])
    assert sorted(ans) == sorted([0, 1])
