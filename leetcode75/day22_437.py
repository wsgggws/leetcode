from typing import Optional

from helper import TreeNode, create_binary_tree


class Solution:
    ans = 0

    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        self.ans = 0
        self._pathSum(root, targetSum)
        return self.ans

    def _pathSum(self, root: Optional[TreeNode], targetSum: int):
        if root is None:
            return
        self.dfs(root, targetSum)
        self._pathSum(root.left, targetSum)
        self._pathSum(root.right, targetSum)

    def dfs(self, root: Optional[TreeNode], targetSum: int):
        if root is None:
            return
        if root.val == targetSum:
            self.ans += 1
        self.dfs(root.left, targetSum - root.val)
        self.dfs(root.right, targetSum - root.val)


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([10, 5, -3, 3, 2, None, 11, 3, -2, None, 1], index=0)
    assert s.pathSum(root, 8) == 3

    root = create_binary_tree([5, 4, 8, 11, None, 13, 4, 7, 2, None, None, 5, 1], index=0)
    assert s.pathSum(root, 22) == 3
