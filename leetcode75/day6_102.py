from typing import List, Optional

from helper import TreeNode, create_binary_tree


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        ans = []
        queue: List[TreeNode] = [root]
        while queue:
            count = len(queue)
            nums = []
            for index in range(count):
                node = queue[index]
                nums.append(node.val)
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
            ans.append(nums)
            queue = queue[count:]
        return ans


if __name__ == "__main__":
    s = Solution()
    nums = []
    root = create_binary_tree(nums, 0)
    assert s.levelOrder(root) == []

    nums = [1]
    root = create_binary_tree(nums, 0)
    assert s.levelOrder(root) == [[1]]

    nums = [3, 9, 20, None, None, 15, 7]
    root = create_binary_tree(nums, 0)
    assert s.levelOrder(root) == [[3], [9, 20], [15, 7]]
