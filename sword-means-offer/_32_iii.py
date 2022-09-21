from typing import List


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
    def levelOrder(self, root: TreeNode) -> List[List[int]]:
        if root is None:
            return []
        results = []
        queue = [root]
        level = 1
        while queue:
            size = len(queue)
            cur_nums = []
            for i in range(size):
                # 添加下一层的节点及收集当前层结果
                node = queue[i]
                if node:
                    cur_nums.append(node.val)
                    queue.append(node.left)
                    queue.append(node.right)
            if cur_nums:
                if level % 2 == 0:
                    results.append(cur_nums[::-1])
                else:
                    results.append(cur_nums)
            queue = queue[size:]
            level += 1
        return results


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 9, 20, None, None, 15, 7], 0)
    assert s.levelOrder(root) == [[3], [20, 9], [15, 7]]
