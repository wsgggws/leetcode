from typing import List


class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def create_binary_tree(nums: List[int], index: int) -> TreeNode:
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


def bfs(root: TreeNode) -> list:
    # 层序遍历
    if root is None:
        return []
    queue = [root]
    results = []
    while queue:
        size = len(queue)
        nums = []
        next_queue = []
        for index in range(size):
            node = queue[index]
            nums.append(node.val if node else None)
            if node:
                next_queue.append(node.left)
                next_queue.append(node.right)
        if any(num is not None for num in nums):
            results.append(nums)
        queue = next_queue
    return results


class Solution:
    def isSymmetric(self, root: TreeNode) -> bool:
        if root is None:
            return True
        queue = [root]
        while queue:
            size = len(queue)
            nums = []
            next_queue = []
            for index in range(size):
                node = queue[index]
                nums.append(node.val if node else None)
                if node:
                    next_queue.append(node.left)
                    next_queue.append(node.right)
            if any(num is not None for num in nums) and not self._is_palindrome(nums):
                return False
            queue = next_queue
        return True

    def _is_palindrome(self, nums: List) -> bool:
        start, end = 0, len(nums) - 1
        while start <= end:
            if nums[start] != nums[end]:
                return False
            start += 1
            end -= 1
        return True


if __name__ == "__main__":
    s = Solution()
    nums = []
    root = create_binary_tree(nums, 0)
    # print(bfs(root))
    assert s.isSymmetric(root) is True

    nums = [1, 2, 2, 3, 4, 4, 3]
    root = create_binary_tree(nums, 0)
    # print(bfs(root))
    assert s.isSymmetric(root) is True

    nums = [1, 2, 2, None, 3, None, 3]
    root = create_binary_tree(nums, 0)
    # print(bfs(root))
    assert s.isSymmetric(root) is False

    nums = [2, 3, 3, 4, 5, 5, 4, None, None, 8, 9, None, None, 9, 8]
    root = create_binary_tree(nums, 0)
    # print(bfs(root))
    assert s.isSymmetric(root) is False

    nums = [1, 0]
    root = create_binary_tree(nums, 0)
    # print(bfs(root))
    assert s.isSymmetric(root) is False
