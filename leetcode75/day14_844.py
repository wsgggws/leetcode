from typing import List


class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        return self.handler(s) == self.handler(t)

    def handler(self, text: str) -> str:
        stack: List[str] = []
        left = 0
        while left < len(text):
            if text[left] == "#":
                if stack:
                    stack.pop()
            else:
                stack.append(text[left])
            left += 1
        return "".join(stack)


if __name__ == "__main__":
    assert Solution().backspaceCompare(s="ab#c", t="ad#c") is True
    assert Solution().backspaceCompare(s="ab##", t="c#d#") is True
    assert Solution().backspaceCompare(s="a#c", t="b") is False
    assert Solution().backspaceCompare(s="a##c", t="b") is False
    assert Solution().backspaceCompare(s="##a#c", t="c") is True
    assert Solution().backspaceCompare(s="bxj##tw", t="bxo#j##tw") is True
