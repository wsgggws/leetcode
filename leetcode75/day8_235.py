from typing import Optional
from helper import TreeNode, create_binary_tree


class Solution:
    def lowestCommonAncestor(self, root: Optional[TreeNode], p: TreeNode, q: TreeNode) -> Optional[TreeNode]:
        if root and p.val < root.val and q.val < root.val:
            return self.lowestCommonAncestor(root.left, p, q)
        elif root and p.val > root.val and q.val > root.val:
            return self.lowestCommonAncestor(root.right, p, q)
        else:
            # 三种情况
            # p.val < root.val < q.val
            # p.val = root.val
            # q.val = root.val
            return root


if __name__ == "__main__":
    s = Solution()
    nums = [6, 2, 8, 0, 4, 7, 9, None, None, 3, 5, None, None, None, None]
    root = create_binary_tree(nums, 0)
    assert s.lowestCommonAncestor(root, TreeNode(2), TreeNode(8)).val == 6
    assert s.lowestCommonAncestor(root, TreeNode(2), TreeNode(4)).val == 2
