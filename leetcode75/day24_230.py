from typing import Optional

from helper import TreeNode, create_binary_tree


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        self.ans = -1
        self.k = k
        self._inorder(root)
        if self.ans == -1:
            raise KeyError
        return self.ans

    def _inorder(self, root: Optional[TreeNode]) -> None:
        # 中序
        if root is None:
            return
        self._inorder(root.left)
        self.k -= 1
        if self.k == 0:
            self.ans = root.val
            return
        self._inorder(root.right)


if __name__ == "__main__":
    root = create_binary_tree([3, 1, 4, None, 2], index=0)
    assert Solution().kthSmallest(root, k=1) == 1
    assert Solution().kthSmallest(root, k=2) == 2
    assert Solution().kthSmallest(root, k=3) == 3
    assert Solution().kthSmallest(root, k=4) == 4

    root = create_binary_tree([5, 3, 6, 2, 4, None, None, 1, None], index=0)
    assert Solution().kthSmallest(root, k=1) == 1
    assert Solution().kthSmallest(root, k=2) == 2
    assert Solution().kthSmallest(root, k=3) == 3
    assert Solution().kthSmallest(root, k=4) == 4
    assert Solution().kthSmallest(root, k=5) == 5
    assert Solution().kthSmallest(root, k=6) == 6
