from typing import Counter, List


class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        """
        case1: AAABB  =>        AB*AB*A            (max_count - 1) * (n+1) + 1
        case2: AAABBB =>        AB*AB*AB           (max_count - 1) * (n+1) + 2
        case3: AAAAAAAGH =>     AG*AH*A*A*A*A      (max_count - 1) * (n+1) + 1
        case4: AAAAAGHBBBB =>   ABG*ABH*AB*AB      (max_count - 1) * (n+1) + 2
        case5: AABCDEFGH =>     AB*ACDEFGH         (max_count - 1) * (n+1) + 1 ？ （2-1）* （1 ..= 7） + 1 < len(case4)=9
        """
        if len(tasks) < 1:
            raise KeyError
        if n == 0:
            return len(tasks)
        counter = Counter(tasks)
        _, most_count = counter.most_common(1)[0]
        return max(
            len(tasks), (most_count - 1) * (n + 1) + sum(1 if most_count == value else 0 for value in counter.values())
        )


if __name__ == "__main__":
    assert Solution().leastInterval(tasks=["A", "A", "A", "B", "B", "B"], n=2) == 8
    assert Solution().leastInterval(tasks=["A", "A", "A", "B", "B", "B"], n=0) == 6
    assert Solution().leastInterval(tasks=["A", "A", "A", "A", "A", "A", "B", "C", "D", "E", "F", "G"], n=2) == 16
    assert Solution().leastInterval(tasks=["A"], n=2) == 1
    assert Solution().leastInterval(tasks=["A", "A"], n=2) == 4
    assert Solution().leastInterval(tasks=["A", "A", "B", "C"], n=1) == 4
    assert Solution().leastInterval(tasks=["A", "A", "A", "B", "B", "B"], n=1) == 6
