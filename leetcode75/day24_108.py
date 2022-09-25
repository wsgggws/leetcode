from typing import List, Optional

from helper import TreeNode, bfs_binary_tree


class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        if len(nums) == 0:
            return None
        if len(nums) == 1:
            return TreeNode(nums[0])
        mid = len(nums) // 2
        root = TreeNode(nums[mid])
        root.left = self.sortedArrayToBST(nums[:mid])
        root.right = self.sortedArrayToBST(nums[mid + 1 :])
        return root


if __name__ == "__main__":
    root = Solution().sortedArrayToBST(nums=[-10, -3, 0, 5, 9])
    assert bfs_binary_tree(root) == [0, -3, 9, -10, None, 5, None]

    root = Solution().sortedArrayToBST(nums=[1, 3])
    assert bfs_binary_tree(root) == [3, 1, None]

    root = Solution().sortedArrayToBST(nums=[1])
    assert bfs_binary_tree(root) == [1]

    root = Solution().sortedArrayToBST(nums=[])
    assert bfs_binary_tree(root) == []
