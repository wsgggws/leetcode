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
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        return (
            self.traverse_left(root.left, root.val)
            and self.traverse_right(root.right, root.val)
            and self.isValidBST(root.left)
            and self.isValidBST(root.right)
        )

    def traverse_left(self, root: Optional[TreeNode], value: int) -> bool:
        if root is None:
            return True
        return root.val < value and self.traverse_left(root.left, value) and self.traverse_left(root.right, value)

    def traverse_right(self, root: Optional[TreeNode], value: int) -> bool:
        if root is None:
            return True
        return root.val > value and self.traverse_right(root.left, value) and self.traverse_right(root.right, value)


if __name__ == "__main__":
    s = Solution()
    nums = [2, 1, 3]
    root = create_binary_tree(nums, 0)
    assert s.isValidBST(root) is True

    nums = [5, 1, 4, None, None, 3, 6]
    root = create_binary_tree(nums, 0)
    assert s.isValidBST(root) is False
