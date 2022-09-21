from typing import Optional
from helper import TreeNode, create_binary_tree


class Solution:
    # def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
    #     # 递归
    #     if root.val > p.val and root.val > q.val:
    #         return self.lowestCommonAncestor(root.left, p, q)
    #     elif root.val < p.val and root.val < q.val:
    #         return self.lowestCommonAncestor(root.right, p, q)
    #     else:
    #         return root

    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> Optional[TreeNode]:
        # 迭代
        node = root
        while node:
            if node.val > p.val and node.val > q.val:
                node = node.left
            elif node.val < p.val and node.val < q.val:
                node = node.right
            else:
                break
        return node


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5], 0)
    assert s.lowestCommonAncestor(root, TreeNode(2), TreeNode(8)).val == 6
    assert s.lowestCommonAncestor(root, TreeNode(2), TreeNode(4)).val == 2
    root = create_binary_tree([6, 2], 0)
    assert s.lowestCommonAncestor(root, TreeNode(2), TreeNode(6)).val == 6
