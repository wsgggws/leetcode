from helper import TreeNode, create_binary_tree


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
