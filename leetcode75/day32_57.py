from typing import List


class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        """
        根据结果来模拟
        return intervals[:start] + [[left, right]] + intervals[end:]
        即求 start, end, left, right 这四个值
        模拟 + 二分查找
        """
        left, right = newInterval[0], newInterval[1]
        start = self.binary_search(nums=[interval[0] for interval in intervals], target=left)
        if start > 0 and left <= intervals[start - 1][1]:
            left = intervals[start - 1][0]
            start -= 1
        end = start
        while end < len(intervals) and intervals[end][1] <= right:
            end += 1
        if end < len(intervals) and intervals[end][0] <= right:
            right = intervals[end][1]
            end += 1
        return intervals[:start] + [[left, right]] + intervals[end:]

    def binary_search(self, nums, target):
        # [)
        left, right = 0, len(nums)
        while left < right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid
        return left


if __name__ == "__main__":
    # 中间，需要判断左右区间
    assert Solution().insert(intervals=[[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], newInterval=[4, 8]) == [
        [1, 2],
        [3, 10],
        [12, 16],
    ]
    # 插在左区间, 不需要合并
    assert Solution().insert(intervals=[[6, 9]], newInterval=[2, 5]) == [[2, 5], [6, 9]]
    # 插在右区间, 不需要合并
    assert Solution().insert(intervals=[[2, 5]], newInterval=[6, 9]) == [[2, 5], [6, 9]]
    # 插入
    assert Solution().insert(intervals=[], newInterval=[5, 7]) == [[5, 7]]
    # 插在左区间, 需要合并
    assert Solution().insert(intervals=[[6, 9]], newInterval=[2, 6]) == [[2, 9]]
    # 插在右区间, 需要合并
    assert Solution().insert(intervals=[[2, 6]], newInterval=[6, 9]) == [[2, 9]]
    # 中间， 不需要合并
    assert Solution().insert(intervals=[[1, 3], [6, 9]], newInterval=[4, 5]) == [[1, 3], [4, 5], [6, 9]]
    # 覆盖
    assert Solution().insert(intervals=[[1, 5]], newInterval=[0, 4]) == [[0, 5]]
    assert Solution().insert(intervals=[[1, 5]], newInterval=[1, 2]) == [[1, 5]]
    assert Solution().insert(intervals=[[1, 5]], newInterval=[1, 7]) == [[1, 7]]
    # 中间, 需要判断右区间
    assert Solution().insert(intervals=[[1, 3], [6, 9]], newInterval=[2, 5]) == [[1, 5], [6, 9]]
    # 中间, 需要判断左区间
    assert Solution().insert(intervals=[[1, 3], [6, 9]], newInterval=[4, 8]) == [[1, 3], [4, 9]]
