from typing import List

from helper import TreeNode, create_binary_tree


class Solution:
    def levelOrder(self, root: TreeNode) -> List[int]:
        if root is None:
            return []
        results = []
        queue = [root]
        while queue:
            size = len(queue)
            for i in range(size):
                # 添加下一层的节点及收集结果
                node = queue[i]
                if node:
                    results.append(node.val)
                    queue.append(node.left)
                    queue.append(node.right)
            queue = queue[size:]
        return results


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 9, 20, None, None, 15, 7], 0)
    assert s.levelOrder(root) == [3, 9, 20, 15, 7]
