from typing import List


class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        rows, cols = len(matrix), len(matrix[0])
        dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        visit = [[False for _ in range(cols)] for _ in range(rows)]
        cur_i, cur_j = 0, 0
        steps, loops = 0, 0
        ans = []
        while steps < rows * cols:
            while 0 <= cur_i < rows and 0 <= cur_j < cols and visit[cur_i][cur_j] is False:
                ans.append(matrix[cur_i][cur_j])
                visit[cur_i][cur_j] = True
                steps += 1
                cur_i = cur_i + dirs[loops % 4][0]
                cur_j = cur_j + dirs[loops % 4][1]
            cur_i = cur_i - dirs[loops % 4][0]
            cur_j = cur_j - dirs[loops % 4][1]
            loops += 1
            cur_i = cur_i + dirs[loops % 4][0]
            cur_j = cur_j + dirs[loops % 4][1]
        return ans


if __name__ == "__main__":
    assert Solution().spiralOrder(matrix=[[1, 2, 3], [4, 5, 6], [7, 8, 9]]) == [1, 2, 3, 6, 9, 8, 7, 4, 5]
    assert Solution().spiralOrder(matrix=[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]) == [
        1,
        2,
        3,
        4,
        8,
        12,
        11,
        10,
        9,
        5,
        6,
        7,
    ]
