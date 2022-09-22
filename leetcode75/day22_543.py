from typing import Optional
from helper import create_binary_tree, TreeNode


class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        return max(
            self.height(root.left) + self.height(root.right),
            self.diameterOfBinaryTree(root.left),
            self.diameterOfBinaryTree(root.right),
        )

    def height(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        return max(self.height(root.left), self.height(root.right)) + 1


if __name__ == "__main__":
    root = create_binary_tree([1, 2, 3, 4, 5, None, None], index=0)
    assert Solution().diameterOfBinaryTree(root) == 3
