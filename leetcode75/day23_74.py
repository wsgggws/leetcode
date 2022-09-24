from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        if len(matrix) == 0 or len(matrix[0]) == 0:
            raise KeyError
        rows, cols = len(matrix), len(matrix[0])
        row, col = 0, cols - 1
        while row < rows and col >= 0:
            if matrix[row][col] == target:
                return True
            elif matrix[row][col] > target:
                col -= 1
            else:
                row += 1
        return False


if __name__ == "__main__":
    assert (
        Solution().searchMatrix(
            matrix=[
                [1, 3, 5, 7],
                [10, 11, 16, 20],
                [23, 30, 34, 60],
            ],
            target=3,
        )
        is True
    )
    assert (
        Solution().searchMatrix(
            matrix=[
                [1, 3, 5, 7],
                [10, 11, 16, 20],
                [23, 30, 34, 60],
            ],
            target=13,
        )
        is False
    )
