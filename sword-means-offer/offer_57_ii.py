# 剑指 Offer 57 - II. 和为s的连续正数序列
# 输入一个正整数 target ，输出所有和为 target 的连续正整数序列（至少含有两个数）。

# 序列内的数字由小到大排列，不同序列按照首个数字从小到大排列。


# 示例 1：

# 输入：target = 9
# 输出：[[2,3,4],[4,5]]
# 示例 2：

# 输入：target = 15
# 输出：[[1,2,3,4,5],[4,5,6],[7,8]]


# 限制：

# 1 <= target <= 10^5

from typing import List


class Solution:
    def findContinuousSequence(self, target: int) -> List[List[int]]:
        """
        暴力不推荐
        双指针
         l  r
        [1, 2, 3, 4, 5, 6, 7, 8] (sum_of(l..=r) < target) => r++
        ...
         l           r
        [1, 2, 3, 4, 5, 6, 7, 8] (sum_of(l..=r) = target) => ans.append(), l++, r++
            l           r
        [1, 2, 3, 4, 5, 6, 7, 8] (sum_of(l..=r) > target) => l++
        ...
                  l     r
        [1, 2, 3, 4, 5, 6, 7, 8] (sum_of(l..=r) > target) => l++
        """
        # 暴力不推荐, 但应该首先想到
        # ans = []
        # border = target // 2 + target % 2
        # for i in range(1, border + 1):
        #     sum_of = i
        #     for j in range(i + 1, border + 1):
        #         sum_of += j
        #         if sum_of == target:
        #             ans.append(list(range(i, j + 1)))
        #             break
        #         if sum_of > target:
        #             break
        # return ans
        # 双指针
        ans = []
        border = target // 2 + target % 2
        left, right = 1, 2
        while left <= border and right <= border and left < right:
            sum_of = (left + right) * (right - left + 1) // 2
            if sum_of == target:
                ans.append(list(range(left, right + 1)))
                left += 1
                right += 1
            elif sum_of > target:
                left += 1
            else:
                right += 1
        return ans


if __name__ == "__main__":
    s = Solution()
    assert s.findContinuousSequence(9) == [[2, 3, 4], [4, 5]]
    assert s.findContinuousSequence(15) == [[1, 2, 3, 4, 5], [4, 5, 6], [7, 8]]
