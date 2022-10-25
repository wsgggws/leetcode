from typing import List, Optional

from helper import TreeNode, create_binary_tree


class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        queue: List[Optional[TreeNode]] = [root]
        while queue:
            size = len(queue)
            nums: List[Optional[int]] = []
            for i in range(size):
                node = queue[i]
                if node:
                    queue.append(node.left)
                    queue.append(node.right)
                    nums.append(node.left.val if node.left else None)
                    nums.append(node.right.val if node.right else None)
            start, end = 0, len(nums) - 1
            while start < end:
                if nums[start] != nums[end]:
                    return False
                start += 1
                end -= 1
            queue = queue[size:]
        return True


if __name__ == "__main__":
    root = create_binary_tree(nums=[1, 2, 2, 3, 4, 4, 3], index=0)
    assert Solution().isSymmetric(root) is True

    root = create_binary_tree(nums=[1, 2, 2, None, 3, None, 3], index=0)
    assert Solution().isSymmetric(root) is False

    root = create_binary_tree(nums=[2, 3, 3, 4, 5, 5, 4, None, None, 8, 9, None, None, 9, 8], index=0)
    assert Solution().isSymmetric(root) is False
