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


class Solution:
    # def maxDepth(self, root: TreeNode) -> int:
    #     queue = [root]
    #     level = 0
    #     while queue:
    #         size = len(queue)
    #         cur_nums = []
    #         for i in range(size):
    #             # 添加下一层的节点及收集当前层结果
    #             node = queue[i]
    #             if node:
    #                 cur_nums.append(node.val)
    #                 queue.append(node.left)
    #                 queue.append(node.right)
    #         if cur_nums:
    #             level += 1
    #         queue = queue[size:]
    #     return level

    def maxDepth(self, root: TreeNode) -> int:
        if root is None:
            return 0
        return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))

    def isBalanced(self, root: TreeNode) -> bool:
        if root is None:
            return True
        if abs(self.maxDepth(root.left) - self.maxDepth(root.right)) <= 1:
            return self.isBalanced(root.left) and self.isBalanced(root.right)
        return False


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 9, 20, None, None, 15, 7], 0)
    assert s.isBalanced(root) is True
    root = create_binary_tree([1, 2, 2, 3, 3, None, None, 4, 4], 0)
    assert s.isBalanced(root) is False
