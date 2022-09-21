#!/usr/bin/env python3
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


def create_binary_tree(nums: list, index: int) -> TreeNode:
    if index >= len(nums) or nums[index] is None:
        return
    node = TreeNode(nums[index])
    node.left = create_binary_tree(nums, index * 2 + 1)
    node.right = create_binary_tree(nums, index * 2 + 2)
    return node


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
            results.extend(nums)
        queue = next_queue
    return results


class Solution:
    def mirrorTree(self, root: TreeNode) -> TreeNode:
        """
        1.init

            [root] top
        2. after pop

            [] top

        3. push

            [root.right] top
            [root.left]

        4. after pop

            [root.left] top

        5. push

            [root.right.right] top
            [root.right.left]
            -----上面的是 right 子树------
            [root.left]
        6. ...
        """
        # 迭代
        stack = [root]
        while stack:
            node = stack.pop()
            if node and (node.left or node.right):
                node.left, node.right = node.right, node.left
                stack.append(node.left)
                stack.append(node.right)
        return root

    # def mirrorTree(self, root: TreeNode) -> TreeNode:
    #     # 递归,
    #     # TODO 迭代
    #     if root is None or (root.left is None and root.right is None):
    #         return root
    #     root.left, root.right = root.right, root.left
    #     self.mirrorTree(root.left)
    #     self.mirrorTree(root.right)
    #     return root


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([], 0)
    assert bfs(root) == []
    mirror_tree = s.mirrorTree(root)
    assert bfs(mirror_tree) == []

    root = create_binary_tree([4, 2, 7, 1, 3, 6, 9], 0)
    assert bfs(root) == [4, 2, 7, 1, 3, 6, 9]
    mirror_tree = s.mirrorTree(root)

    assert bfs(mirror_tree) == [4, 7, 2, 9, 6, 3, 1]

    #       3                      3
    #   9        20           20        9
    # N   N   15    7       7   15    N   N
    #        N  N N  N    N  N N  N

    root = create_binary_tree([3, 9, 20, None, None, 15, 7], 0)
    assert bfs(root) == [3, 9, 20, None, None, 15, 7]
    mirror_tree = s.mirrorTree(root)

    assert bfs(mirror_tree) == [3, 20, 9, 7, 15, None, None]

    #                1                                   1
    #          2            3                    3               2
    #      4       5     N    N             N         N      5       4
    #   6    7   N  N                                      N   N   7    6
    # N  N N  N                                                  N  N  N N
    root = create_binary_tree([1, 2, 3, 4, 5, None, None, 6, 7], 0)
    assert bfs(root) == [1, 2, 3, 4, 5, None, None, 6, 7, None, None]
    mirror_tree = s.mirrorTree(root)

    assert bfs(mirror_tree) == [1, 3, 2, None, None, 5, 4, None, None, 7, 6]
