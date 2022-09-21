# 给定一个数组 A[0,1,…,n-1]，请构建一个数组 B[0,1,…,n-1]，
# 其中 B[i] 的值是数组 A 中除了下标 i 以外的元素的积,
# 即 B[i]=A[0]×A[1]×…×A[i-1]×A[i+1]×…×A[n-1]。不能使用除法。

#

# 示例:

# 输入: [1,2,3,4,5]
# 输出: [120,60,40,30,24]
#

# 提示：

# 所有元素乘积之和不会溢出 32 位整数
# a.length <= 100000


from typing import List


class Solution:
    def constructArr(self, a: List[int]) -> List[int]:
        prefix_muls = [0 for _ in a]
        suffix_muls = [0 for _ in a]

        tmp = 1
        for i in range(len(a)):
            prefix_muls[i] = tmp
            tmp *= a[i]

        tmp = 1
        for i in reversed(range(len(a))):
            suffix_muls[i] = tmp
            tmp *= a[i]

        return [prefix_mul * suffix_mul for prefix_mul, suffix_mul in zip(prefix_muls, suffix_muls)]


if __name__ == "__main__":
    s = Solution()
    assert s.constructArr([]) == []
    assert s.constructArr([12]) == [1]
    assert s.constructArr([1, 2]) == [2, 1]
    assert s.constructArr([1, 2, 3, 4, 5]) == [120, 60, 40, 30, 24]
