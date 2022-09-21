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
    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
        ...


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 0)
    assert s.lowestCommonAncestor(root, TreeNode(5), TreeNode(1)).val == 3
    assert s.lowestCommonAncestor(root, TreeNode(5), TreeNode(4)).val == 5
    assert s.lowestCommonAncestor(root, TreeNode(7), TreeNode(4)).val == 2
    assert s.lowestCommonAncestor(root, TreeNode(7), TreeNode(0)).val == 3
    assert s.lowestCommonAncestor(root, TreeNode(7), TreeNode(6)).val == 5
    assert s.lowestCommonAncestor(root, TreeNode(0), TreeNode(8)).val == 1
