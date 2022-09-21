from typing import Union


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left: Union[TreeNode, None] = None
        self.right: Union[TreeNode, None] = None


def create_binary_tree(nums: list, index: int) -> Union[TreeNode, None]:
    if index >= len(nums) or nums[index] is None:
        return
    node = TreeNode(nums[index])
    node.left = create_binary_tree(nums, index * 2 + 1)
    node.right = create_binary_tree(nums, index * 2 + 2)
    return node


class Solution:
    def isSubStructure(self, A: TreeNode, B: TreeNode) -> bool:
        # 递归
        if A is None or B is None:
            return False
        return self.compare(A, B) or self.isSubStructure(A.left, B) or self.isSubStructure(A.right, B)

    def compare(self, A: TreeNode, B: TreeNode) -> bool:
        if B is None:
            return True
        if A is None:
            return False
        return A.val == B.val and self.compare(A.left, B.left) and self.compare(A.right, B.right)


if __name__ == "__main__":
    s = Solution()
    root_a = create_binary_tree([1, 2, 3], 0)
    root_b = create_binary_tree([3, 1], 0)
    assert s.isSubStructure(root_a, root_b) is False
    root_a = create_binary_tree([3, 4, 5, 1, 2], 0)
    root_b = create_binary_tree([4, 1], 0)
    assert s.isSubStructure(root_a, root_b) is True

    root_a = create_binary_tree([3, 4, 5, 1, 2], 0)
    root_b = create_binary_tree([2], 0)
    assert s.isSubStructure(root_a, root_b) is True
    root_a = create_binary_tree([3, 4, 5, 1, 2], 0)
    root_b = create_binary_tree([4, 1, 2], 0)
    assert s.isSubStructure(root_a, root_b) is True
    root_a = create_binary_tree([3, 4, 5, 1, 2], 0)
    root_b = create_binary_tree([4, 1, 2, 0], 0)
    assert s.isSubStructure(root_a, root_b) is False
