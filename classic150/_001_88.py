# https://leetcode.cn/problems/merge-sorted-array/?envType=study-plan-v2&id=top-interview-150

from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        # 从后往前, 较大的移动到后面，并移动该索引不断向前
        start = m + n - 1
        while start >= 0 and m > 0 and n > 0:
            if nums1[m - 1] > nums2[n - 1]:
                nums1[start] = nums1[m - 1]
                m -= 1
            else:
                nums1[start] = nums2[n - 1]
                n -= 1
            start -= 1

        # m有多余不需要考虑，不需要移动，所以只需要考虑移动剩余的n到 nums1
        while start >= 0 and n > 0:
            nums1[start] = nums2[n - 1]
            n -= 1
            start -= 1


if __name__ == "__main__":
    nums1 = [1, 2, 3, 0, 0, 0]
    Solution().merge(nums1=nums1, m=3, nums2=[2, 5, 6], n=3)
    assert nums1 == [1, 2, 2, 3, 5, 6]

    nums1 = [1]
    Solution().merge(nums1=nums1, m=1, nums2=[], n=0)
    assert nums1 == [1]

    nums1 = [0]
    Solution().merge(nums1=nums1, m=0, nums2=[1], n=1)
    assert nums1 == [1]

    nums1 = [1, 2, 3, 0]
    Solution().merge(nums1=nums1, m=3, nums2=[0], n=1)
    assert nums1 == [0, 1, 2, 3]
