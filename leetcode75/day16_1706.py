from typing import List


class Solution:
    def findBall(self, grid: List[List[int]]) -> List[int]:
        rows, cols = len(grid), len(grid[0])
        ans = [-1 for _ in range(cols)]
        for col in range(cols):
            ans[col] = -1
            cur_col = col
            for row in range(rows):
                if grid[row][cur_col] == 1:
                    if cur_col + 1 < cols and grid[row][cur_col + 1] == 1:
                        cur_col += 1
                    else:
                        break
                else:
                    if cur_col - 1 >= 0 and grid[row][cur_col - 1] == -1:
                        cur_col -= 1
                    else:
                        break
            else:
                ans[col] = cur_col
        return ans


if __name__ == "__main__":
    assert Solution().findBall(
        grid=[[1, 1, 1, -1, -1], [1, 1, 1, -1, -1], [-1, -1, -1, 1, 1], [1, 1, 1, 1, -1], [-1, -1, -1, -1, -1]]
    ) == [1, -1, -1, -1, -1]
    assert Solution().findBall(grid=[[-1]]) == [-1]
    assert Solution().findBall(
        grid=[[1, 1, 1, 1, 1, 1], [-1, -1, -1, -1, -1, -1], [1, 1, 1, 1, 1, 1], [-1, -1, -1, -1, -1, -1]]
    ) == [0, 1, 2, 3, 4, -1]
