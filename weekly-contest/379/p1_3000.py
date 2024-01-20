from typing import List


class Solution:
    def areaOfMaxDiagonal(self, dimensions: List[List[int]]) -> int:
        max_diagonal = 0
        max_area = 0
        for x, y in dimensions:
            diagonal = x * x + y * y
            if diagonal > max_diagonal:
                max_diagonal = diagonal
                max_area = x * y
            if diagonal == max_diagonal:
                max_area = max(max_area, x * y)
        return max_area


if __name__ == "__main__":
    assert Solution().areaOfMaxDiagonal([[1, 1], [2, 2], [3, 3]]) == 9
    assert Solution().areaOfMaxDiagonal(dimensions=[[9, 3], [8, 6]]) == 48
    assert Solution().areaOfMaxDiagonal(dimensions=[[3, 4], [4, 3]]) == 12
