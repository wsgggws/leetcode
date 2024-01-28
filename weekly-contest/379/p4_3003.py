# https://leetcode.cn/problems/maximize-the-number-of-partitions-after-operations/description/


class Solution:
    def maxPartitionsAfterOperations(self, s: str, k: int) -> int:
        # TODO:
        ...


if __name__ == "__main__":
    assert Solution().maxPartitionsAfterOperations(s="accca", k=2) == 3
    assert Solution().maxPartitionsAfterOperations(s="aabaab", k=3) == 1
    assert Solution().maxPartitionsAfterOperations(s="xxyz", k=1) == 4
