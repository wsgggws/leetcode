class Solution:
    def calculate(self, s: str) -> int:
        """
        stack: 最后只保留需要相加的数字
        *: stack[-1] * num
        /: stack[-1] // num
        -: -num
        +: num
        return sum(stack)
        """
        stack = []
        s = s.strip()
        num = 0
        pre_sign = "+"
        for i in range(len(s)):
            if s[i].isdigit():
                num = num * 10 + int(s[i])
            if s[i] in "+-*/" or i == len(s) - 1:
                if pre_sign == "+":
                    stack.append(num)
                elif pre_sign == "-":
                    stack.append(-num)
                elif pre_sign == "*":
                    stack[-1] = stack[-1] * num
                elif pre_sign == "/":
                    # note! -3 // 2 == -2, 所以对负数需要转成正数处理后再变成负数
                    stack[-1] = stack[-1] // num if stack[-1] > 0 else -((-stack[-1]) // num)
                num = 0
                pre_sign = s[i]
        return sum(stack)


if __name__ == "__main__":
    assert Solution().calculate(s="3+2*2") == 7
    assert Solution().calculate(s=" 3/2 ") == 1
    assert Solution().calculate(s=" 3+5 / 2 ") == 5
    assert Solution().calculate(s="14-3/2") == 13
