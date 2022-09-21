# 剑指 Offer 12. 矩阵中的路径
# 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。

# 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。


# 例如，在下面的 3×4 的矩阵中包含单词 "ABCCED"（单词中的字母已标出）。


# 示例 1：

# 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
# 输出：true
# 示例 2：

# 输入：board = [["a","b"],["c","d"]], word = "abcd"
# 输出：false


# 提示：

# 1 <= board.length <= 200
# 1 <= board[i].length <= 200
# board 和 word 仅由大小写英文字母组成


from typing import List


class Solution:

    DIRECTIONS = ((1, 0), (-1, 0), (0, 1), (0, -1))

    def exist(self, board: List[List[str]], word: str) -> bool:
        """尝试搜索 + 回溯, 搜索模板题"""
        visited = [[False for _ in range(len(board[0]))] for _ in range(len(board))]
        for i in range(len(board)):
            for j in range(len(board[0])):
                if self._dfs(board, word, visited, i, j, 0):
                    return True
        return False

    def _dfs(
        self, board: List[List[str]], word: str, visited: List[List[bool]], i: int, j: int, word_position: int
    ) -> bool:
        if (
            i < 0
            or i >= len(board)
            or j < 0
            or j >= len(board[0])
            or visited[i][j] is True
            or board[i][j] != word[word_position]
        ):
            return False
        elif word_position == len(word) - 1:
            return True

        visited[i][j] = True
        for direction in self.DIRECTIONS:
            next_i, next_j = i + direction[0], j + direction[1]
            if self._dfs(board, word, visited, next_i, next_j, word_position + 1):
                return True
        visited[i][j] = False
        return False


if __name__ == "__main__":
    s = Solution()
    assert s.exist(board=[["a", "b", "c", "e"], ["s", "f", "c", "s"], ["a", "d", "e", "e"]], word="abcced") is True
    assert s.exist(board=[["a", "b"], ["c", "d"]], word="abcd") is False
