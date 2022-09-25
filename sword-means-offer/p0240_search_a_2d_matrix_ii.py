# 240. Search a 2D Matrix II
# Medium

# Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:

# Integers in each row are sorted in ascending from left to right.
# Integers in each column are sorted in ascending from top to bottom.
# Example:

# Consider the following matrix:

# [
#   [1,   4,  7, 11, 15],
#   [2,   5,  8, 12, 19],
#   [3,   6,  9, 16, 22],
#   [10, 13, 14, 17, 24],
#   [18, 21, 23, 26, 30]
# ]
# Given target = 5, return true.

# Given target = 20, return false.


class Solution(object):
    def searchMatrix(self, matrix, target):
        """
        :type matrix: List[List[int]]
        :type target: int
        :rtype: bool
        """
        if matrix is None or len(matrix) == 0 or len(matrix[0]) == 0:
            return False
        index_row = len(matrix) - 1
        while index_row >= 0 and matrix[index_row][0] > target:
            index_row -= 1

        index_col = 0
        while index_col < len(matrix[0]) and index_row >= 0:
            if matrix[index_row][index_col] == target:
                return True
            elif matrix[index_row][index_col] > target:
                index_row -= 1
            else:
                index_col += 1
        return False


if __name__ == "__main__":
    solution = Solution()
    matrix = [[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]]
    assert solution.searchMatrix(matrix, 5) is True
    assert solution.searchMatrix(matrix, 20) is False
