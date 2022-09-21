from typing import List


class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        if image[sr][sc] == color:
            return image
        rows, colums = len(image), len(image[0])
        ans = [[image[row][colum] for colum in range(colums)] for row in range(rows)]
        self.DFS(ans, sr, sc, image[sr][sc], color, rows, colums)
        return ans

    def DFS(self, ans: List[List[int]], sr: int, sc: int, old_color: int, new_color: int, rows: int, colums: int):
        ans[sr][sc] = new_color
        for direct in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
            next_sr, next_sc = sr + direct[0], sc + direct[1]
            if 0 <= next_sr < rows and 0 <= next_sc < colums and ans[next_sr][next_sc] == old_color:
                self.DFS(ans, next_sr, next_sc, old_color, new_color, rows, colums)


if __name__ == "__main__":
    assert Solution().floodFill(image=[[1, 1, 1], [1, 1, 0], [1, 0, 1]], sr=1, sc=1, color=2) == [
        [2, 2, 2],
        [2, 2, 0],
        [2, 0, 1],
    ]
    assert Solution().floodFill([[0, 0, 0], [0, 0, 0]], sr=0, sc=0, color=2) == [
        [2, 2, 2],
        [2, 2, 2],
    ]

    assert Solution().floodFill([[0, 0, 0], [0, 0, 0]], sr=0, sc=0, color=0) == [
        [0, 0, 0],
        [0, 0, 0],
    ]
