from typing import List, Union


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
    results = []

    def pathSum(self, root: TreeNode, target: int) -> List[List[int]]:
        self.results = []
        self.DFS(root, target, [])
        return self.results

    def DFS(self, root: TreeNode, target: int, nums: List[int]):
        if root is None:
            return
        # if target - root.val < 0:
        #     return
        if root.left is None and root.right is None and root.val == target:
            self.results.append(nums + [root.val])
            return
        if root.left:
            self.DFS(root.left, target - root.val, nums + [root.val])
        if root.right:
            self.DFS(root.right, target - root.val, nums + [root.val])


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([5, 4, 8, 11, None, 13, 4, 7, 2, None, None, None, None, 5, 1], 0)
    assert s.pathSum(root, 22) == [[5, 4, 11, 2], [5, 8, 4, 5]]
    root = create_binary_tree([1, 2, 3], 0)
    assert s.pathSum(root, 5) == []
    root = create_binary_tree([1, 2, None], 0)
    assert s.pathSum(root, 0) == []
    assert s.pathSum(root, 1) == []
    root = create_binary_tree([1], 0)
    assert s.pathSum(root, 1) == [[1]]
    root = create_binary_tree([-2, None, -3], 0)
    assert s.pathSum(root, -5) == [[-2, -3]]
