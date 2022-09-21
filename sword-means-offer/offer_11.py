# 剑指 Offer 11. 旋转数组的最小数字
# 把一个数组最开始的若干个元素搬到数组的末尾，我们称之为数组的旋转。

# 给你一个可能存在 重复 元素值的数组 numbers ，它原来是一个升序排列的数组，并按上述情形进行了一次旋转。请返回旋转数组的最小元素。例如，数组 [3,4,5,1,2] 为 [1,2,3,4,5] 的一次旋转，该数组的最小值为1。

# 示例 1：

# 输入：[3,4,5,1,2]
# 输出：1
# 示例 2：

# 输入：[2,2,2,0,1]
# 输出：0


from typing import List


class Solution:
    # def minArray(self, numbers: List[int]) -> int:
    #     return min(numbers)

    def minArray(self, numbers: List[int]) -> int:
        """
        从右往左找一个数，这个数字比左边数值小
        [left mid right]
        mid > right => 这个数字在 [mid+1, right]
        mid < right => 这个数字在 [left, mid]
        mid = right => 这个数字在 [left, right-1]
        """
        left, right = 0, len(numbers) - 1
        while left < right:
            mid = left + (right - left) // 2
            if numbers[mid] > numbers[right]:
                left = mid + 1
            elif numbers[mid] < numbers[right]:
                right = mid
            else:
                right -= 1
        return numbers[left]  # or numbers[right]


if __name__ == "__main__":
    s = Solution()
    assert s.minArray([3]) == 3
    assert s.minArray([3, 4, 5, 1, 2]) == 1
    assert s.minArray([2, 2, 2, 0, 1]) == 0
    assert s.minArray([1, 2, 3, 4, 5]) == 1
    assert s.minArray([1, 1, 1, 1, 1]) == 1
    assert s.minArray([1, 0, 1, 1, 1]) == 0
    assert s.minArray([1, 1, 1, 0, 1]) == 0
    assert s.minArray([0, 0, 1, 1, 1]) == 0
    assert s.minArray([0, 1, 1, 1, 1]) == 0
