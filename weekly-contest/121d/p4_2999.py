class Solution:
    def numberOfPowerfulInt(self, start: int, finish: int, limit: int, s: str) -> int:
        # TODO: 未完成
        # 枚举会超时
        # 组合与排列，但如何排队最高位与 limit 相同但又不在 finish 之内的数呢？似乎数量级也在 10**14 会超时
        ...


if __name__ == "__main__":
    assert Solution().numberOfPowerfulInt(start=1, finish=6000, limit=4, s="124") == 5
    assert Solution().numberOfPowerfulInt(start=15, finish=215, limit=6, s="10") == 2
    assert Solution().numberOfPowerfulInt(start=1000, finish=2000, limit=4, s="3000") == 0
