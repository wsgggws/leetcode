from typing import List


class Solution:
    def singleNumbers(self, nums: List[int]) -> List[int]:
        """
        (1)0001
        (6)0110 ^
        -------
           0111
        说明这两个不同的数，有三位是不一样的
        取最后一位为 1 进行分组，可将数据分成两组
        则这两组中分别时行异或运算可得出两个值
        """
        ans = 0
        for num in nums:
            ans ^= num
        index = 0
        while ans & 1 == 0:
            index += 1
            ans >>= 1
        num1, num2 = 0, 0
        for num in nums:
            if (num >> index) & 1 == 1:
                num1 ^= num
            else:
                num2 ^= num
        return [num1, num2]


if __name__ == "__main__":
    s = Solution()
    assert sorted(s.singleNumbers([4, 1, 4, 6])) == [1, 6]
    assert sorted(s.singleNumbers([1, 2, 10, 4, 1, 4, 3, 3])) == [2, 10]
