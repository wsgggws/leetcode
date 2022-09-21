from typing import List


class Solution:
    def decodeString(self, s: str) -> str:
        stack: List[str] = []
        tmp: List[str] = []
        index = len(s) - 1
        while index >= 0:
            if "a" <= s[index] <= "z" or s[index] == "]":
                stack.append(s[index])
            elif s[index] == "[":
                while True:
                    char = stack.pop()
                    if char == "]":
                        break
                    else:
                        tmp.append(char)
            else:
                if index - 1 >= 0 and index - 2 >= 0 and "0" <= s[index - 1] <= "9" and "1" <= s[index - 2] <= "3":
                    # 三位数字
                    count = int(s[index - 2 : index + 1])
                    index -= 2
                elif index - 1 >= 0 and "1" <= s[index - 1] <= "9":
                    # 两位数字
                    count = int(s[index - 1 : index + 1])
                    index -= 1
                else:
                    count = int(s[index])
                for _ in range(count):
                    stack.extend(tmp[::-1])
                tmp = []
            index -= 1
        return "".join(stack[::-1])


if __name__ == "__main__":
    assert Solution().decodeString(s="3[a]2[bc]") == "aaabcbc"
    assert Solution().decodeString(s="3[a]102[bc]") == "aaa" + 102 * "bc"
    assert Solution().decodeString(s="3[a2[c]]") == "accaccacc"
    assert Solution().decodeString(s="3[a2[cd]]") == "acdcdacdcdacdcd"
    assert Solution().decodeString(s="2[abc]3[cd]ef") == "abcabccdcdcdef"
    assert Solution().decodeString(s="abc3[cd]xyz") == "abccdcdcdxyz"
    assert Solution().decodeString(s="23[a]12[bc]") == "aaaaaaaaaaaaaaaaaaaaaaabcbcbcbcbcbcbcbcbcbcbcbc"
    assert Solution().decodeString(s="3[a10[c]]") == "acccccccccc" * 3
    assert Solution().decodeString(s="3[a1[c]]") == "ac" * 3
