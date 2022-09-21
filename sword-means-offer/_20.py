# 剑指 Offer 20. 表示数值的字符串
# 请实现一个函数用来判断字符串是否表示数值（包括整数和小数）。

# 数值（按顺序）可以分成以下几个部分：

# 若干空格
# 一个 小数 或者 整数
# （可选）一个 'e' 或 'E' ，后面跟着一个 整数
# 若干空格
# 小数（按顺序）可以分成以下几个部分：

# （可选）一个符号字符（'+' 或 '-'）
# 下述格式之一：
# 至少一位数字，后面跟着一个点 '.'
# 至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字
# 一个点 '.' ，后面跟着至少一位数字
# 整数（按顺序）可以分成以下几个部分：

# （可选）一个符号字符（'+' 或 '-'）
# 至少一位数字
# 部分数值列举如下：

# ["+100", "5e2", "-123", "3.1416", "-1E-16", "0123"]
# 部分非数值列举如下：

# ["12e", "1a3.14", "1.2.3", "+-5", "12e+5.4"]


# 示例 1：

# 输入：s = "0"
# 输出：true
# 示例 2：

# 输入：s = "e"
# 输出：false
# 示例 3：

# 输入：s = "."
# 输出：false
# 示例 4：

# 输入：s = "    .1  "
# 输出：true


# 提示：

# 1 <= s.length <= 20
# s 仅含英文字母（大写和小写），数字（0-9），加号 '+' ，减号 '-' ，空格 ' ' 或者点 '.' 。


class Solution:
    def isNumber(self, s: str) -> bool:
        """分而治之
        (part1) [eE] (part2)
        part1 是有符号整数或者小数，part2是有符号整数
        当 part1 是小数时，可分解为
        .在前: 后面是无符号整数
        .在后: 前面是有符号整数
        .在中间：前面是有符号号整数 或者仅符号，后面是无符号整数

        还需要注意: 数值只允许前后有空格，中间有空格均为非数值
        """
        parts = s.strip().lower().split("e")
        if len(parts) > 2:
            return False
        # elif len(parts) == 2:
        #     return () and self.is_integer(parts[1])
        # else:
        #     return self.is_integer(parts[0]) or self.is_decimal(parts[0])

        # 避免反复判断
        first = self.is_integer(parts[0]) or self.is_decimal(parts[0])
        if len(parts) == 1:
            return first
        else:
            return first and self.is_integer(parts[1])

    def is_integer(self, text: str) -> bool:
        if len(text) == 0 or len(text) != len(text.strip()):
            return False
        if text[0] in "+-":
            text = text[1:]
        return self.is_unsigned_integer(text)

    def is_unsigned_integer(self, text: str) -> bool:
        if len(text) == 0 or len(text) != len(text.strip()):
            return False
        return text.isdigit()

    def is_sign(self, text: str) -> bool:
        if len(text) == 0 or len(text) != len(text.strip()):
            return False
        return len(text) == 1 and text in "+-"

    def is_decimal(self, text: str) -> bool:
        if len(text) == 0 or len(text) != len(text.strip()):
            return False
        if "." not in text:
            return False
        elif text.startswith("."):
            return self.is_unsigned_integer(text[1:])
        elif text.endswith("."):
            return self.is_integer(text[:-1])
        else:
            parts = text.split(".")
            return (
                len(parts) == 2
                and (self.is_sign(parts[0]) or self.is_integer(parts[0]))
                and self.is_unsigned_integer(parts[1])
            )


if __name__ == "__main__":
    s = Solution()
    nums = ["+100", "5e2", "-123", "3.1416", "-1E-16", "0123"]
    for num in nums:
        assert s.isNumber(num) is True
    nonums = ["12e", "1a3.14", "1.2.3", "+-5", "12e+5.4"]
    for nonum in nonums:
        assert s.isNumber(nonum) is False
    assert s.isNumber("0") is True
    assert s.isNumber("e") is False
    assert s.isNumber(".") is False
    assert s.isNumber("  .1   ") is True
    assert s.isNumber("+.8") is True
    assert s.isNumber("-.8") is True
    assert s.isNumber("- .8") is False
