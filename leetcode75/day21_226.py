from typing import Optional

from helper import TreeNode, bfs_binary_tree, create_binary_tree


class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None or (root.left is None and root.right is None):
            return root
        root.left, root.right = root.right, root.left
        self.invertTree(root.left)
        self.invertTree(root.right)
        return root


if __name__ == "__main__":
    root = create_binary_tree([4, 2, 7, 1, 3, 6, 9], index=0)
    assert bfs_binary_tree(Solution().invertTree(root)) == [4, 7, 2, 9, 6, 3, 1]

    root = create_binary_tree([2, 1, 3], index=0)
    assert bfs_binary_tree(Solution().invertTree(root)) == [2, 3, 1]

    root = create_binary_tree([], index=0)
    assert bfs_binary_tree(Solution().invertTree(root)) == []
