from collections import Counter
from typing import List


class Solution:
    def topKFrequent(self, words: List[str], k: int) -> List[str]:
        return [_[0] for _ in Counter(sorted(words)).most_common(k)]


if __name__ == "__main__":
    assert Solution().topKFrequent(words=["i", "a", "a", "love", "leetcode", "i", "love", "coding"], k=2) == [
        "a",
        "i",
    ]
    assert Solution().topKFrequent(["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k=4) == [
        "the",
        "is",
        "sunny",
        "day",
    ]
