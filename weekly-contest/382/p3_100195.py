class Solution:
    def flowerGame(self, n: int, m: int) -> int:
        return n * m // 2


if __name__ == "__main__":
    assert Solution().flowerGame(3, 2) == 3
    assert Solution().flowerGame(1, 1) == 0
