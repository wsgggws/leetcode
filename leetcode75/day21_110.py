from typing import Optional
from helper import create_binary_tree, TreeNode


class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        left_height = self.height(root.left)
        right_height = self.height(root.right)
        return abs(left_height - right_height) <= 1 and self.isBalanced(root.left) and self.isBalanced(root.right)

    def height(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        return max(self.height(root.left), self.height(root.right)) + 1


if __name__ == "__main__":
    root = create_binary_tree([3, 9, 20, None, None, 15, 7], index=0)
    assert Solution().isBalanced(root) is True

    root = create_binary_tree([1, 2, 2, 3, 3, None, None, 4, 4, None, None, None, None], index=0)
    assert Solution().isBalanced(root) is False

    root = create_binary_tree([], index=0)
    assert Solution().isBalanced(root) is True
