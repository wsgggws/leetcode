from typing import Optional

from helper import TreeNode, create_binary_tree


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
