from typing import List


class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        if len(grid) == 0 or len(grid[0]) == 0:
            raise KeyError
        rows, cols = len(grid), len(grid[0])
        empty, bads = 0, 0
        queue: List[tuple[int, int]] = []
        for row in range(rows):
            for col in range(cols):
                if grid[row][col] == 0:
                    empty += 1
                elif grid[row][col] == 2:
                    queue.append((row, col))
                    bads += 1
        ans = 0
        # 广度优先搜索
        while queue:
            count = len(queue)
            for index in range(count):
                row, col = queue[index]
                for direct in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
                    next_row, next_col = row + direct[0], col + direct[1]
                    if 0 <= next_row < rows and 0 <= next_col < cols and grid[next_row][next_col] == 1:
                        queue.append((next_row, next_col))
                        grid[next_row][next_col] = 2
                        bads += 1
            queue = queue[count:]
            if queue:
                ans += 1
        if empty + bads == rows * cols:
            return ans
        return -1


if __name__ == "__main__":
    assert Solution().orangesRotting(grid=[[2, 1, 1], [1, 1, 0], [0, 1, 1]]) == 4
    assert Solution().orangesRotting(grid=[[2, 1, 1], [0, 1, 1], [1, 0, 1]]) == -1
    assert Solution().orangesRotting(grid=[[0, 2]]) == 0
