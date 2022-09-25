from typing import List

from helper import TreeNode, create_binary_tree


class Solution:
    def levelOrder(self, root: TreeNode) -> List[List[int]]:
        if root is None:
            return []
        results = []
        queue = [root]
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
                results.append(cur_nums)
            queue = queue[size:]
        return results


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 9, 20, None, None, 15, 7], 0)

    assert s.levelOrder(root) == [[3], [9, 20], [15, 7]]
