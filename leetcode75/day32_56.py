from typing import List


class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        if len(intervals) < 1:
            raise KeyError
        intervals.sort()
        ans = []
        left = 0
        while left < len(intervals):
            right = left
            # note! 这里要记录历史最大值，并且边界也要与该历史最大值比较；)
            maxs = intervals[right][1]
            while right + 1 < len(intervals) and intervals[right + 1][0] <= maxs:
                maxs = max(maxs, intervals[right + 1][1])
                right += 1
            ans.append([intervals[left][0], maxs])
            left = right + 1
        return ans


if __name__ == "__main__":
    assert Solution().merge(intervals=[[1, 3], [2, 6], [8, 10], [15, 18]]) == [[1, 6], [8, 10], [15, 18]]
    assert Solution().merge(intervals=[[1, 4], [4, 5]]) == [[1, 5]]
    assert Solution().merge(intervals=[[1, 3], [4, 5]]) == [[1, 3], [4, 5]]
    assert Solution().merge(intervals=[[1, 3]]) == [[1, 3]]
    assert Solution().merge(intervals=[[1, 4], [2, 3]]) == [[1, 4]]
    assert Solution().merge(intervals=[[2, 3], [4, 5], [6, 7], [8, 9], [1, 10]]) == [[1, 10]]
