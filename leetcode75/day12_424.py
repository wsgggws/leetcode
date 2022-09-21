from collections import Counter


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        left, right = 0, min(len(s), k + 1)
        counter = Counter(s[: k + 1])
        while right < len(s):
            counter[s[right]] += 1
            max_count = counter.most_common(1)[0][1]
            if right - left + 1 - max_count > k:
                counter[s[left]] -= 1
                left += 1
            right += 1
        return right - left


if __name__ == "__main__":
    assert Solution().characterReplacement(s="ABAB", k=2) == 4
    assert Solution().characterReplacement(s="AABABBA", k=1) == 4
    assert Solution().characterReplacement(s="A", k=0) == 1
    assert Solution().characterReplacement(s="A", k=1) == 1
