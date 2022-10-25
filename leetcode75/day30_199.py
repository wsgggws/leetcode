from typing import List, Optional

from helper import TreeNode, create_binary_tree


class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        queue: List[Optional[TreeNode]] = [root]
        ans: List[int] = [root.val]
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
            valid_num = None
            # 特别注意 None 与 0 的严格区别
            for num in nums[::-1]:
                if num is not None:
                    valid_num = num
                    break
            if valid_num is not None:
                ans.append(valid_num)
            queue = queue[size:]
        return ans


if __name__ == "__main__":
    root = create_binary_tree(nums=[1, 2, 3, None, 5, None, 4], index=0)
    assert Solution().rightSideView(root) == [1, 3, 4]

    root = create_binary_tree(nums=[1, None, 3], index=0)
    assert Solution().rightSideView(root) == [1, 3]

    root = create_binary_tree(nums=[], index=0)
    assert Solution().rightSideView(root) == []

    root = create_binary_tree(nums=[1, 2, 3, None, 5, None, 4, None, None, 10, None, None, None, None, None], index=0)
    assert Solution().rightSideView(root) == [1, 3, 4, 10]

    root = create_binary_tree(nums=[1, 2, 0], index=0)
    assert Solution().rightSideView(root) == [1, 0]
