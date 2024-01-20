class Solution:
    def minMovesToCaptureTheQueen(self, a: int, b: int, c: int, d: int, e: int, f: int) -> int:
        # 只有 1 或者 2(抽将也算) 步
        # 与车同行
        if a == e:
            if c != a or not (b < d < f or f < d < b):
                return 1
        # 与车同列
        if b == f:
            if d != b or not (a < c < e or e < c < a):
                return 1
        # 与象同对角线
        if f - e == d - c:
            if b - a != f - e or not (c < a < e or e < a < c):
                return 1
        if f + e == d + c:
            if b + a != f + e or not (c < a < e or e < a < c):
                return 1
        # 其它情况
        return 2


if __name__ == "__main__":
    assert Solution().minMovesToCaptureTheQueen(a=1, b=1, c=8, d=8, e=2, f=3) == 2
    assert Solution().minMovesToCaptureTheQueen(a=5, b=3, c=3, d=4, e=5, f=2) == 1
