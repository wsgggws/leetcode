from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        """
        3. 优化双指针

        s
        ABCBAC, ABC
        e
        """
        target = Counter(t)
        record = Counter()
        source = [(char, index) for index, char in enumerate(s) if char in t]
        min_start, min_end = 0, len(s) + 1
        start = end = 0
        while end < len(source) and start + len(t) <= len(source):
            record[source[end][0]] += 1
            # 满足了条件,覆盖全部t的位置
            if record >= target:
                # 移动 start, 直到不满点
                while record >= target:
                    record[source[start][0]] -= 1
                    start += 1
                if source[end][1] - source[start - 1][1] < min_end - min_start:
                    min_start, min_end = source[start - 1][1], source[end][1]
            end += 1
        if min_end == len(s) + 1:
            return ""
        return s[min_start : min_end + 1]


# class Solution:
#     def minWindow(self, s: str, t: str) -> str:
#         """
#         2. 双指针

#         s
#         ADOBECODEBANC
#         e

#         s
#         ADOBECODEBANC  # e走到覆盖全部t的位置
#              e

#          s
#         ADOBECODEBANC  # s走到不覆盖全部t的位置, 计算长度并更新最小长度
#              e
#               s                        s
#         ADOBECODEBANC     -> ADOBECODEBANC
#                   e                      e
#         (0,5) -> (5, 10) -> (9, 12)
#         => s[9:12+1]
#         """
#         target = Counter(t)
#         record = Counter()
#         min_start, min_end = 0, len(s) + 1
#         start = end = 0
#         while end < len(s) and start + len(t) <= len(s):
#             if s[end] in t:
#                 record[s[end]] += 1
#                 # 满足了条件,覆盖全部t的位置
#                 if all(record[char] >= target[char] for char in target):
#                     # 移动 start, 直到不满点
#                     while all(record[char] >= target[char] for char in target):
#                         record[s[start]] -= 1
#                         start += 1
#                     if end - (start - 1) < min_end - min_start:
#                         min_start, min_end = start - 1, end
#             end += 1
#         if min_end == len(s) + 1:
#             return ""
#         return s[min_start : min_end + 1]


# class Solution:
#     def minWindow(self, s: str, t: str) -> str:
#         """
#         1. 暴力枚举
#             a. 找到起始点，只要字母在t中都可以成为起始点
#             b. 往右走，直到覆盖t, 更新最小长度
#         """
#         # 构建起始点
#         starts = []
#         for i in range(len(s)):
#             if s[i] in t and i + len(t) <= len(s):
#                 starts.append(i)
#         mins = len(s) + 1
#         mins_start = mins_end = 0
#         flag = False
#         for start in starts:
#             end = start
#             target = Counter(t)
#             record = Counter()
#             while end < len(s):
#                 if s[end] in t:
#                     record[s[end]] += 1
#                 if all(record[char] >= target[char] for char in target.keys()) and end - start + 1 < mins:
#                     # 记录搜索到了，更新最小长度
#                     mins = end - start + 1
#                     mins_start, mins_end = start, end
#                     flag = True
#                     break
#                 end += 1
#         if flag is False:
#             return ""
#         return s[mins_start : mins_end + 1]


if __name__ == "__main__":
    assert Solution().minWindow(s="ADOBECODEBANC", t="ABC") == "BANC"
    assert Solution().minWindow(s="a", t="a") == "a"
    assert Solution().minWindow(s="a", t="aa") == ""
    assert Solution().minWindow(s="a", t="b") == ""
