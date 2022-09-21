from typing import List, Optional


class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left: Optional[TreeNode] = None
        self.right: Optional[TreeNode] = None


def create_binary_tree(nums: List[int], index: int) -> Optional[TreeNode]:
    """
         1
      2     2
    3   4 4   3
    左右结点分别为 2*i+1, 2*i+2
    """
    # 递归创建二叉树
    if index >= len(nums) or nums[index] is None:
        return
    root = TreeNode(nums[index])
    root.left = create_binary_tree(nums, index * 2 + 1)
    root.right = create_binary_tree(nums, index * 2 + 2)
    return root


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
