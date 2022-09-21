# 输入一个非负整数数组，把数组里所有数字拼接起来排成一个数，打印能拼接出的所有数字中最小的一个。

# 示例 1:

# 输入: [10,2]
# 输出: "102"
# 示例 2:

# 输入: [3,30,34,5,9]
# 输出: "3033459"
#

# 提示:

# 0 < nums.length <= 100
# 说明:


# 输出结果可能非常大，所以你需要返回一个字符串而不是整数
# 拼接起来的数字可能会有前导 0，最后结果不需要去掉前导 0
import functools
from typing import List


class Solution:
    def minNumber(self, nums: List[int]) -> str:
        """通过比较 {x}{y} {y}{x} 进行排序即可"""
        sorted_nums = sorted(nums, key=functools.cmp_to_key(lambda x, y: 1 if f"{x}{y}" > f"{y}{x}" else -1))
        print(sorted_nums)
        return "".join(map(lambda num: str(num), sorted_nums))


if __name__ == "__main__":
    solution = Solution()
    assert solution.minNumber([10, 2]) == "102"
    assert solution.minNumber([3, 30, 34, 5, 9]) == "3033459"
