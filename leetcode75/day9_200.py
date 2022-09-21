from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        rows, colums = len(grid), len(grid[0])
        ans = 0
        for row in range(rows):
            for colum in range(colums):
                if grid[row][colum] == "1":
                    ans += 1
                    self.DFS(grid, row, colum, rows, colums)
        return ans

    def DFS(self, grid: List[List[str]], row: int, colum: int, rows: int, colums: int):
        grid[row][colum] = "#"
        for direct in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
            next_row, next_colum = row + direct[0], colum + direct[1]
            if 0 <= next_row < rows and 0 <= next_colum < colums and grid[next_row][next_colum] == "1":
                self.DFS(grid, next_row, next_colum, rows, colums)


if __name__ == "__main__":
    assert (
        Solution().numIslands(
            grid=[
                ["1", "1", "1", "1", "0"],
                ["1", "1", "0", "1", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "0", "0", "0"],
            ]
        )
        == 1
    )
    assert (
        Solution().numIslands(
            grid=[
                ["1", "1", "0", "0", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "1", "0", "0"],
                ["0", "0", "0", "1", "1"],
            ]
        )
        == 3
    )
