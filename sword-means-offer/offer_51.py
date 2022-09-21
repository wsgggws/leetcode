# https://leetcode.cn/problems/shu-zu-zhong-de-ni-xu-dui-lcof/

from typing import List


class Solution:
    """
    1. 暴力法O(N*N)
    2. 归并排序?
       - O(N*log2N)
       - 利用对两个有序的列表进行合并，可方便统计出逆序数。
       - i.e. 7 5 6 4
       - 7 5 | 6 4
       - 7 | 5, 6 | 4
       - 5 7 | 4 6   # (归并5,7时出现逆序数 1 次，归并 6,4时出现逆序数1 次, 共 2 次)
       - 4 5 6 7 # (归并第1个值时，4 比5 小，那么逆序数的数量应该是`中间位置-index(5)` 2 次)
                 # (因为左边的5 7是递增的，都会比4 大)
                 # (归并第2个值时, 会选5，由于选择的是左区间的，不出现逆序数)
                 # (归并第3个值时, 6比7小，那么逆序数量中间位置-index(7) 1次)
                 # (因为左边的包括7以后是递增的，都会比6大)
                 # 由此逆序数为 1 + 1 + 2 + 1 = 5
    """

    ans = 0

    def reversePairs(self, nums: List[int]) -> int:
        # 暴力法
        # count = 0
        # for i in range(len(nums)):
        #     for j in range(i+1, len(nums)):
        #         if nums[i] > nums[j]:
        #             count += 1
        # return count

        # 归并排序
        # [)
        self.ans = 0
        self._merge_sort(nums, 0, len(nums))
        return self.ans

    def _merge_sort(self, nums: List[int], left: int, right: int):
        # [left, mid) [mid, right)
        if left + 1 >= right:
            return
        mid = left + (right - left) // 2
        self._merge_sort(nums, left, mid)
        self._merge_sort(nums, mid, right)
        self._merge(nums, left, mid, right)

    def _merge(self, nums: List[int], left: int, mid: int, right: int):
        temp = nums[left:right]

        i = left
        j = mid
        index = left
        while i < mid and j < right:
            if temp[i - left] > temp[j - left]:
                self.ans += mid - i
                nums[index] = temp[j - left]
                j += 1
            else:
                nums[index] = temp[i - left]
                i += 1
            index += 1

        while i < mid:
            nums[index] = temp[i - left]
            index += 1
            i += 1
        while j < right:
            nums[index] = temp[j - left]
            index += 1
            j += 1


if __name__ == "__main__":
    s = Solution()
    assert s.reversePairs(nums=[7, 5, 6, 4]) == 5
