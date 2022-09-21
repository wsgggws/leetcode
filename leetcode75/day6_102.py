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
