from collections import defaultdict
from typing import List


class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        """
        动态规划

                2       3       6       7
        0
        1
        2
        3
        4
        5
        6
        7

        {
            0: [[]]
            2: [[] + [2]] = [[2]]
            4: [[2] + [2]] = [[2,2]]
            6: [[2,2] + [2], [3]+[3], []+[6]] = [[2,2.2], [3,3], [6]]
            3: [[] + [3]] [[3]]
            5: [[2] + [3]]
            7: [[2,2] + [3], [0] + [7]] = [[2,2,3], [7]]
        }
        """
        dp = defaultdict(list)
        dp[0] = [[]]
        for candidate in candidates:
            for num in range(1, target + 1):
                if num >= candidate and dp[num - candidate]:
                    dp[num].extend([case + [candidate] for case in dp[num - candidate]])
        return dp[target]


# class Solution:
#     def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
#         """
#         搜索 + 剪枝
#                            7
#                -2          -3          -6          -7
#            -2  -3 -6 -7
#          -2 -3 -6 -7
#         -2 -3 -6 -7
#         """
#         self.ans = []
#         candidates.sort()
#         self.dfs(candidates, 0, target, [])
#         return self.ans

#     def dfs(self, candidates, index, target, path):
#         if target == 0:
#             self.ans.append(path)
#             return
#         for i in range(index, len(candidates)):
#             if target - candidates[i] < 0:
#                 break
#             # self.dfs(candidates, i, target - candidates[i], path + [candidates[i]])
#             target -= candidates[i]
#             pre_path = [num for num in path]
#             path.append(candidates[i])
#             self.dfs(candidates, i, target, path)
#             target += candidates[i]
#             path = pre_path


if __name__ == "__main__":
    assert Solution().combinationSum(candidates=[2, 3, 6, 7], target=7) == [[2, 2, 3], [7]]
    assert Solution().combinationSum(candidates=[2, 3, 5], target=8) == [[2, 2, 2, 2], [2, 3, 3], [3, 5]]
    assert Solution().combinationSum(candidates=[2], target=1) == []
