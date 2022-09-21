# 剑指 Offer 04. 二维数组中的查找
# 在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序。请完成一个高效的函数，输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。


# 示例:

# 现有矩阵 matrix 如下：

# [
#   [1,   4,  7, 11, 15],
#   [2,   5,  8, 12, 19],
#   [3,   6,  9, 16, 22],
#   [10, 13, 14, 17, 24],
#   [18, 21, 23, 26, 30]
# ]
# 给定 target = 5，返回 true。

# 给定 target = 20，返回 false。


# 限制：

# 0 <= n <= 1000

# 0 <= m <= 1000


from typing import List


class Solution:
    def findNumberIn2DArray(self, matrix: List[List[int]], target: int) -> bool:
        """
        拿题后先模拟，以下模拟选取 5 的过程
        从左下角开始(小于往右走，大于往上左)
                                18 > 5 向上走到3        3 < 5 向右走1步       6 > 5 向上到5  => Found
        1,   4,  7, 11, 15     |1,   4,  7, 11, 15     1, |4,  7, 11, 15     1, 4,  7, 11, 15
        2,   5,  8, 12, 19     |2,   5,  8, 12, 19     2, |5,  8, 12, 19     2, {5},8, 12, 19
        3,   6,  9, 16, 22     |3,   6,  9, 16, 22     3, |6,  9, 16, 22     3, 6,  9, 16, 22
                                ------------------         --------------
        10, 13, 14, 17, 24     10, 13, 14, 17, 24      10, 13, 14, 17, 24    10, 13, 14, 17, 24
        18, 21, 23, 26, 30     18, 21, 23, 26, 30      18, 21, 23, 26, 30    18, 21, 23, 26, 30

        选取 20 的过程
                                18 < 20 向右1步         21 < 20 向上走到 13     13 < 20 向右走2步
        1,   4,  7, 11, 15     1,  | 4,  7, 11, 15     1,   |4,  7, 11, 15     1,   4,  7, |11, 15
        2,   5,  8, 12, 19     2,  | 5,  8, 12, 19     2,   |5,  8, 12, 19     2,   5,  8, |12, 19
        3,   6,  9, 16, 22     3,  | 6,  9, 16, 22     3,   |6,  9, 16, 22     3,   6,  9, |16, 22
        10, 13, 14, 17, 24     10, |13, 14, 17, 24     10,  |13, 14, 17,24     10, 13, 14, |17, 24
                                                             -------------                  ------
        18, 21, 23, 26, 30     18, |21, 23, 26, 30       18, 21, 23, 26, 30      18, 21, 23, 26, 30
                                    --------------

         17 < 24 向右走1步       24 > 20 向上走到 19      19 < 20 向右走一步  => 越界 Not Found
         1,   4,  7, 11, |15     1,   4,  7, 11, |15     1,   4,  7, 11, 15
         2,   5,  8, 12, |19     2,   5,  8, 12, |19     2,   5,  8, 12, 19{}
                                                  --
         3,   6,  9, 16, |22     3,   6,  9, 16, 22      3,   6,  9, 16, 22
         10, 13, 14, 17, |24     10,  13, 14, 17,24      10, 13, 14, 17, 24
                          --
         18, |21, 23, 26, 30     18, 21, 23, 26, 30      18, 21, 23, 26, 30
                                    --------------
        """
        row, col = len(matrix) - 1, 0
        while row >= 0 and col < len(matrix[0]):
            if matrix[row][col] == target:
                return True
            elif matrix[row][col] > target:
                array = [line[col] for line in matrix[: row + 1]]
                step = self._binary_search(array, target)
                if step == 0:
                    row -= 1
                else:
                    row = step
            else:
                step = self._binary_search(matrix[row][col:], target)
                col += step or 1
                # if step == 0:
                #     col += 1
                # else:
                #     col += step
        return False

    def _binary_search(self, array: List[int], target: int) -> int:
        # [) 左闭右开
        left, right = 0, len(array)
        while left < right:
            mid = left + (right - left) // 2
            if array[mid] == target:
                return mid
            elif array[mid] > target:
                right = mid
            else:
                left = mid + 1
        return left - 1


if __name__ == "__main__":
    s = Solution()
    matrix = [[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]]
    assert s._binary_search([1, 2, 3, 10, 18], 5) == 2
    assert s._binary_search([1, 2, 3, 10, 18], 3) == 2
    assert s._binary_search([1, 2, 3, 10, 18], 0) == -1
    assert s.findNumberIn2DArray(matrix, target=5) is True
    assert s.findNumberIn2DArray(matrix, target=20) is False
