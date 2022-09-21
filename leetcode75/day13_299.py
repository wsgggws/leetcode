from typing import List


class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        bulls, cows = 0, 0
        # bulls
        eq_indexes: List[int] = []
        for i in range(len(secret)):
            if secret[i] == guess[i]:
                bulls += 1
                eq_indexes.append(i)
        # cows
        secret: List[str] = sorted([char for i, char in enumerate(secret) if i not in eq_indexes])
        guess: List[str] = sorted([char for i, char in enumerate(guess) if i not in eq_indexes])
        i, j = 0, 0
        while i < len(secret) and j < len(guess):
            if secret[i] == guess[j]:
                cows += 1
                i += 1
                j += 1
            elif secret[i] < guess[j]:
                i += 1
            else:
                j += 1
        return f"{bulls}A{cows}B"


if __name__ == "__main__":
    assert Solution().getHint(secret="1807", guess="7810") == "1A3B"
    assert Solution().getHint(secret="1123", guess="0111") == "1A1B"
