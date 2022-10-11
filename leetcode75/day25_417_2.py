from typing import List, Set, Tuple


class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        """要求既可流向太平洋也可流向大西洋
        遍历每个边界坐标，深度优先搜索到的点添加，并交集表示即可到达太平洋，又可达大西洋
        """
        if len(heights) == 0 or len(heights[0]) == 0:
            raise KeyError
        rows, cols = len(heights), len(heights[0])
        border_p = [(0, index) for index in range(cols)] + [(index, 0) for index in range(rows)]
        border_a = [(rows - 1, index) for index in range(cols)] + [(index, cols - 1) for index in range(rows)]
        return list(map(list, self.search(border_p, heights) & self.search(border_a, heights)))

    def search(self, starts, heights) -> Set[Tuple[int, int]]:
        visit = set()
        rows, cols = len(heights), len(heights[0])
        for row, col in starts:
            self.dfs(row, col, rows, cols, visit, heights)
        return visit

    def dfs(self, row, col, rows, cols, visit, heights):
        if (row, col) in visit:
            return
        visit.add((row, col))
        for direct in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
            nrow, ncol = row + direct[0], col + direct[1]
            if 0 <= nrow < rows and 0 <= ncol < cols and heights[nrow][ncol] >= heights[row][col]:
                self.dfs(nrow, ncol, rows, cols, visit, heights)


if __name__ == "__main__":
    ans = Solution().pacificAtlantic(
        heights=[
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ]
    )
    assert sorted(ans) == sorted([[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]])
    ans = Solution().pacificAtlantic(heights=[[2, 1], [1, 2]])
    assert sorted(ans) == sorted([[0, 0], [0, 1], [1, 0], [1, 1]])
