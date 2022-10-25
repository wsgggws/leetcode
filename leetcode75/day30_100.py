from typing import Optional

from helper import TreeNode, create_binary_tree


class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p is None and q is None:
            return True
        elif p and q and p.val == q.val:
            return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)
        else:
            return False


if __name__ == "__main__":
    p = create_binary_tree(nums=[1, 2, 3], index=0)
    q = create_binary_tree(nums=[1, 2, 3], index=0)
    assert Solution().isSameTree(p, q) is True

    p = create_binary_tree(nums=[1, 2, None], index=0)
    q = create_binary_tree(nums=[1, None, 2], index=0)
    assert Solution().isSameTree(p, q) is False

    p = create_binary_tree(nums=[1, 2, 1], index=0)
    q = create_binary_tree(nums=[1, 1, 2], index=0)
    assert Solution().isSameTree(p, q) is False
