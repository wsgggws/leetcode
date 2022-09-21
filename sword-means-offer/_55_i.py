from helper import TreeNode, create_binary_tree


class Solution:
    def maxDepth(self, root: TreeNode) -> int:
        queue = [root]
        level = 0
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
                level += 1
            queue = queue[size:]
        return level


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 9, 20, None, None, 15, 7], 0)
    assert s.maxDepth(root) == 3
    root = create_binary_tree([], 0)
    assert s.maxDepth(root) == 0
