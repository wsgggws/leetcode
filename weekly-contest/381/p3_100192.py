from collections import Counter


class Solution:
    def minimumPushes(self, word: str) -> int:
        counter = Counter(word)
        times = 1
        numbers = 0
        ans = 0
        while len(counter):
            char, count = counter.most_common(1)[0]
            ans += count * times
            numbers += 1
            if numbers == 8:
                times += 1
                numbers = 0
            counter.pop(char)
        return ans


if __name__ == "__main__":
    assert Solution().minimumPushes(word="abcde") == 5
    assert Solution().minimumPushes(word="xycdefghij") == 12
    assert Solution().minimumPushes(word="aabbccddeeffgghhiiiiii") == 24
